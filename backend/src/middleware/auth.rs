//! JWT Authentication middleware
//!
//! This module provides authentication middleware for protecting routes
//! that require a valid JWT token.

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use mongodb::bson::oid::ObjectId;
use serde::Serialize;
use std::sync::Arc;

use crate::utils::error::AppError;
use crate::utils::jwt::{extract_bearer_token, Claims, JwtManager};

/// Authenticated user extracted from JWT token
#[derive(Debug, Clone)]
pub struct AuthUser {
    /// User's ObjectId
    pub id: ObjectId,
    /// User's email
    pub email: String,
    /// JWT claims
    pub claims: Claims,
}

impl AuthUser {
    /// Create a new AuthUser from JWT claims
    pub fn from_claims(claims: Claims) -> Result<Self, AppError> {
        let id = claims.user_id()?;
        Ok(Self {
            id,
            email: claims.email.clone(),
            claims,
        })
    }
}

/// Authentication error response
#[derive(Debug, Serialize)]
struct AuthErrorResponse {
    error: String,
    status: u16,
}

/// Middleware extractor that requires authentication
///
/// Use this extractor in handlers that require authentication.
/// It will extract and validate the JWT token from the Authorization header.
///
/// # Example
///
/// ```ignore
/// async fn protected_handler(
///     RequireAuth(user): RequireAuth,
/// ) -> impl IntoResponse {
///     format!("Hello, {}!", user.email)
/// }
/// ```
#[derive(Debug, Clone)]
pub struct RequireAuth(pub AuthUser);

#[async_trait]
impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Get the JWT manager from extensions
        let jwt = parts
            .extensions
            .get::<Arc<JwtManager>>()
            .cloned()
            .ok_or_else(|| {
                let error = AuthErrorResponse {
                    error: "Authentication not configured".to_string(),
                    status: 500,
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response()
            })?;

        // Get Authorization header
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| {
                let error = AuthErrorResponse {
                    error: "Missing Authorization header".to_string(),
                    status: 401,
                };
                (StatusCode::UNAUTHORIZED, Json(error)).into_response()
            })?;

        // Extract bearer token
        let token = extract_bearer_token(auth_header).ok_or_else(|| {
            let error = AuthErrorResponse {
                error: "Invalid Authorization header format. Expected: Bearer <token>".to_string(),
                status: 401,
            };
            (StatusCode::UNAUTHORIZED, Json(error)).into_response()
        })?;

        // Validate token
        let claims = jwt.validate_access_token(token).map_err(|e| {
            let (status, message) = match e {
                AppError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token has expired".to_string()),
                AppError::InvalidToken(msg) => (StatusCode::UNAUTHORIZED, msg),
                _ => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            };
            let error = AuthErrorResponse {
                error: message,
                status: status.as_u16(),
            };
            (status, Json(error)).into_response()
        })?;

        // Create AuthUser from claims
        let user = AuthUser::from_claims(claims).map_err(|_| {
            let error = AuthErrorResponse {
                error: "Invalid user data in token".to_string(),
                status: 401,
            };
            (StatusCode::UNAUTHORIZED, Json(error)).into_response()
        })?;

        Ok(RequireAuth(user))
    }
}

/// Optional authentication extractor
///
/// Similar to RequireAuth but returns None if no valid token is present
/// instead of rejecting the request.
#[derive(Debug, Clone)]
pub struct OptionalAuth(pub Option<AuthUser>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuth
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match RequireAuth::from_request_parts(parts, state).await {
            Ok(RequireAuth(user)) => Ok(OptionalAuth(Some(user))),
            Err(_) => Ok(OptionalAuth(None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_user_from_claims() {
        let claims = Claims {
            sub: ObjectId::new().to_hex(),
            email: "test@example.com".to_string(),
            exp: chrono::Utc::now().timestamp() + 3600,
            iat: chrono::Utc::now().timestamp(),
            iss: "test".to_string(),
            token_type: crate::utils::jwt::TokenType::Access,
            jti: uuid::Uuid::new_v4().to_string(),
        };

        let user = AuthUser::from_claims(claims.clone()).expect("Should create user");

        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.id.to_hex(), claims.sub);
    }

    #[test]
    fn test_auth_user_invalid_id() {
        let claims = Claims {
            sub: "not-a-valid-object-id".to_string(),
            email: "test@example.com".to_string(),
            exp: chrono::Utc::now().timestamp() + 3600,
            iat: chrono::Utc::now().timestamp(),
            iss: "test".to_string(),
            token_type: crate::utils::jwt::TokenType::Access,
            jti: uuid::Uuid::new_v4().to_string(),
        };

        let result = AuthUser::from_claims(claims);
        assert!(result.is_err());
    }
}
