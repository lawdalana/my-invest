//! JWT (JSON Web Token) utilities for authentication
//!
//! This module provides JWT token creation and validation using HS256 algorithm.
//! Supports both access tokens (short-lived) and refresh tokens (long-lived).

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::config::JwtConfig;
use crate::utils::error::{AppError, Result};

/// Token type for distinguishing between access and refresh tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
}

/// JWT Claims structure
///
/// Contains the standard JWT claims plus custom claims for our application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject - the user ID
    pub sub: String,

    /// User email
    pub email: String,

    /// Expiration time (Unix timestamp)
    pub exp: i64,

    /// Issued at time (Unix timestamp)
    pub iat: i64,

    /// Issuer
    pub iss: String,

    /// Token type (access or refresh)
    pub token_type: TokenType,

    /// JWT ID (unique identifier for this token)
    pub jti: String,
}

impl Claims {
    /// Check if this is an access token
    pub fn is_access_token(&self) -> bool {
        self.token_type == TokenType::Access
    }

    /// Check if this is a refresh token
    pub fn is_refresh_token(&self) -> bool {
        self.token_type == TokenType::Refresh
    }

    /// Get the user ID as an ObjectId
    pub fn user_id(&self) -> Result<ObjectId> {
        ObjectId::parse_str(&self.sub)
            .map_err(|_| AppError::InvalidToken("Invalid user ID in token".to_string()))
    }
}

/// Token pair containing both access and refresh tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

/// JWT manager for creating and validating tokens
#[derive(Clone)]
pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_expiration: i64,
    refresh_expiration: i64,
    issuer: String,
}

impl JwtManager {
    /// Create a new JWT manager from configuration
    pub fn new(config: &JwtConfig) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(config.secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(config.secret.as_bytes()),
            access_expiration: config.access_expiration,
            refresh_expiration: config.refresh_expiration,
            issuer: config.issuer.clone(),
        }
    }

    /// Create a token pair (access + refresh) for a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user's ObjectId
    /// * `email` - The user's email address
    ///
    /// # Returns
    ///
    /// * `Ok(TokenPair)` - The generated token pair
    /// * `Err(AppError)` - If token creation fails
    pub fn create_token_pair(&self, user_id: &ObjectId, email: &str) -> Result<TokenPair> {
        let access_token = self.create_token(user_id, email, TokenType::Access)?;
        let refresh_token = self.create_token(user_id, email, TokenType::Refresh)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.access_expiration,
        })
    }

    /// Create a single token
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user's ObjectId
    /// * `email` - The user's email address
    /// * `token_type` - The type of token to create
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The encoded JWT token
    /// * `Err(AppError)` - If token creation fails
    pub fn create_token(
        &self,
        user_id: &ObjectId,
        email: &str,
        token_type: TokenType,
    ) -> Result<String> {
        let now = Utc::now();
        let expiration = match token_type {
            TokenType::Access => now + Duration::seconds(self.access_expiration),
            TokenType::Refresh => now + Duration::seconds(self.refresh_expiration),
        };

        let claims = Claims {
            sub: user_id.to_hex(),
            email: email.to_string(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
            iss: self.issuer.clone(),
            token_type,
            jti: uuid::Uuid::new_v4().to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::InternalError(format!("Failed to create token: {}", e)))
    }

    /// Validate and decode a token
    ///
    /// # Arguments
    ///
    /// * `token` - The JWT token string
    ///
    /// # Returns
    ///
    /// * `Ok(Claims)` - The decoded claims
    /// * `Err(AppError::TokenExpired)` - If the token is expired
    /// * `Err(AppError::InvalidToken)` - If the token is invalid
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let mut validation = Validation::default();
        validation.set_issuer(&[&self.issuer]);

        let token_data: TokenData<Claims> =
            decode(token, &self.decoding_key, &validation)?;

        Ok(token_data.claims)
    }

    /// Validate an access token specifically
    ///
    /// Ensures the token is an access token, not a refresh token.
    pub fn validate_access_token(&self, token: &str) -> Result<Claims> {
        let claims = self.validate_token(token)?;

        if !claims.is_access_token() {
            return Err(AppError::InvalidToken(
                "Expected access token, got refresh token".to_string(),
            ));
        }

        Ok(claims)
    }

    /// Validate a refresh token specifically
    ///
    /// Ensures the token is a refresh token, not an access token.
    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims> {
        let claims = self.validate_token(token)?;

        if !claims.is_refresh_token() {
            return Err(AppError::InvalidToken(
                "Expected refresh token, got access token".to_string(),
            ));
        }

        Ok(claims)
    }

    /// Get the expiration time for access tokens in seconds
    pub fn access_expiration(&self) -> i64 {
        self.access_expiration
    }

    /// Get the expiration time for refresh tokens in seconds
    pub fn refresh_expiration(&self) -> i64 {
        self.refresh_expiration
    }
}

/// Extract the bearer token from an Authorization header value
///
/// # Arguments
///
/// * `auth_header` - The Authorization header value (e.g., "Bearer eyJ...")
///
/// # Returns
///
/// * `Some(&str)` - The token without the "Bearer " prefix
/// * `None` - If the header is not a valid Bearer token
pub fn extract_bearer_token(auth_header: &str) -> Option<&str> {
    auth_header.strip_prefix("Bearer ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_jwt_manager() -> JwtManager {
        let config = JwtConfig {
            secret: "test-secret-key-for-testing-only".to_string(),
            access_expiration: 3600,      // 1 hour
            refresh_expiration: 2592000,  // 30 days
            issuer: "test-issuer".to_string(),
        };
        JwtManager::new(&config)
    }

    #[test]
    fn test_create_access_token() {
        let jwt = create_test_jwt_manager();
        let user_id = ObjectId::new();
        let email = "test@example.com";

        let token = jwt.create_token(&user_id, email, TokenType::Access)
            .expect("Token creation should succeed");

        assert!(!token.is_empty());

        // Validate the token
        let claims = jwt.validate_token(&token)
            .expect("Token validation should succeed");

        assert_eq!(claims.sub, user_id.to_hex());
        assert_eq!(claims.email, email);
        assert!(claims.is_access_token());
    }

    #[test]
    fn test_create_refresh_token() {
        let jwt = create_test_jwt_manager();
        let user_id = ObjectId::new();
        let email = "test@example.com";

        let token = jwt.create_token(&user_id, email, TokenType::Refresh)
            .expect("Token creation should succeed");

        let claims = jwt.validate_token(&token)
            .expect("Token validation should succeed");

        assert!(claims.is_refresh_token());
    }

    #[test]
    fn test_create_token_pair() {
        let jwt = create_test_jwt_manager();
        let user_id = ObjectId::new();
        let email = "test@example.com";

        let pair = jwt.create_token_pair(&user_id, email)
            .expect("Token pair creation should succeed");

        assert!(!pair.access_token.is_empty());
        assert!(!pair.refresh_token.is_empty());
        assert_eq!(pair.token_type, "Bearer");
        assert_eq!(pair.expires_in, 3600);

        // Validate both tokens
        let access_claims = jwt.validate_access_token(&pair.access_token)
            .expect("Access token validation should succeed");
        assert!(access_claims.is_access_token());

        let refresh_claims = jwt.validate_refresh_token(&pair.refresh_token)
            .expect("Refresh token validation should succeed");
        assert!(refresh_claims.is_refresh_token());
    }

    #[test]
    fn test_invalid_token() {
        let jwt = create_test_jwt_manager();

        let result = jwt.validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_expired_token() {
        let config = JwtConfig {
            secret: "test-secret".to_string(),
            access_expiration: -3600,  // Expired 1 hour ago (ensures definitive expiration)
            refresh_expiration: -3600,
            issuer: "test".to_string(),
        };
        let jwt = JwtManager::new(&config);
        let user_id = ObjectId::new();

        let token = jwt.create_token(&user_id, "test@example.com", TokenType::Access)
            .expect("Token creation should succeed");

        let result = jwt.validate_token(&token);
        assert!(matches!(result, Err(AppError::TokenExpired)));
    }

    #[test]
    fn test_wrong_token_type() {
        let jwt = create_test_jwt_manager();
        let user_id = ObjectId::new();

        // Create a refresh token
        let refresh = jwt.create_token(&user_id, "test@example.com", TokenType::Refresh)
            .expect("Token creation should succeed");

        // Try to validate it as an access token
        let result = jwt.validate_access_token(&refresh);
        assert!(matches!(result, Err(AppError::InvalidToken(_))));

        // Create an access token
        let access = jwt.create_token(&user_id, "test@example.com", TokenType::Access)
            .expect("Token creation should succeed");

        // Try to validate it as a refresh token
        let result = jwt.validate_refresh_token(&access);
        assert!(matches!(result, Err(AppError::InvalidToken(_))));
    }

    #[test]
    fn test_extract_bearer_token() {
        assert_eq!(
            extract_bearer_token("Bearer abc123"),
            Some("abc123")
        );
        assert_eq!(
            extract_bearer_token("Bearer "),
            Some("")
        );
        assert_eq!(
            extract_bearer_token("Basic abc123"),
            None
        );
        assert_eq!(
            extract_bearer_token("bearer abc123"),  // Case sensitive
            None
        );
    }

    #[test]
    fn test_claims_user_id() {
        let jwt = create_test_jwt_manager();
        let user_id = ObjectId::new();

        let token = jwt.create_token(&user_id, "test@example.com", TokenType::Access)
            .expect("Token creation should succeed");

        let claims = jwt.validate_token(&token)
            .expect("Token validation should succeed");

        let extracted_id = claims.user_id()
            .expect("User ID extraction should succeed");

        assert_eq!(extracted_id, user_id);
    }

    #[test]
    fn test_token_has_unique_jti() {
        let jwt = create_test_jwt_manager();
        let user_id = ObjectId::new();

        let token1 = jwt.create_token(&user_id, "test@example.com", TokenType::Access)
            .expect("Token creation should succeed");
        let token2 = jwt.create_token(&user_id, "test@example.com", TokenType::Access)
            .expect("Token creation should succeed");

        let claims1 = jwt.validate_token(&token1).expect("Validation should succeed");
        let claims2 = jwt.validate_token(&token2).expect("Validation should succeed");

        // Each token should have a unique JTI
        assert_ne!(claims1.jti, claims2.jti);
    }
}
