//! Centralized error handling for My-Invest Backend
//!
//! This module defines a unified error type `AppError` that handles all error cases
//! throughout the application, with proper HTTP status code mapping.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;
use tracing::error;

/// Application-wide result type
pub type Result<T> = std::result::Result<T, AppError>;

/// Application error enum covering all possible error cases
#[derive(Debug, Error)]
pub enum AppError {
    // Authentication & Authorization errors
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Access denied")]
    AccessDenied,

    // Validation errors
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    // Resource errors
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Resource already exists: {0}")]
    AlreadyExists(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    // Database errors
    #[error("Database error: {0}")]
    DatabaseError(String),

    // Cache errors
    #[error("Cache error: {0}")]
    CacheError(String),

    // External API errors
    #[error("External API error: {0}")]
    ExternalApiError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("External service unavailable: {0}")]
    ServiceUnavailable(String),

    // Internal errors
    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    // Password errors
    #[error("Password hashing error")]
    PasswordHashError,

    // Limit errors
    #[error("Limit exceeded: {0}")]
    LimitExceeded(String),

    // Not implemented (for stubs like password reset)
    #[error("Feature not implemented: {0}")]
    NotImplemented(String),
}

/// Error response format for API responses
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl AppError {
    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            // 400 Bad Request
            AppError::ValidationError(_) | AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,

            // 401 Unauthorized
            AppError::InvalidCredentials
            | AppError::Unauthorized(_)
            | AppError::TokenExpired
            | AppError::InvalidToken(_) => StatusCode::UNAUTHORIZED,

            // 403 Forbidden
            AppError::AccessDenied => StatusCode::FORBIDDEN,

            // 404 Not Found
            AppError::NotFound(_) => StatusCode::NOT_FOUND,

            // 409 Conflict
            AppError::AlreadyExists(_) | AppError::Conflict(_) => StatusCode::CONFLICT,

            // 429 Too Many Requests
            AppError::RateLimitExceeded | AppError::LimitExceeded(_) => StatusCode::TOO_MANY_REQUESTS,

            // 500 Internal Server Error
            AppError::DatabaseError(_)
            | AppError::CacheError(_)
            | AppError::InternalError(_)
            | AppError::ConfigError(_)
            | AppError::PasswordHashError => StatusCode::INTERNAL_SERVER_ERROR,

            // 501 Not Implemented
            AppError::NotImplemented(_) => StatusCode::NOT_IMPLEMENTED,

            // 502 Bad Gateway
            AppError::ExternalApiError(_) => StatusCode::BAD_GATEWAY,

            // 503 Service Unavailable
            AppError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    /// Check if this error should be logged at error level
    fn should_log_as_error(&self) -> bool {
        matches!(
            self,
            AppError::DatabaseError(_)
                | AppError::CacheError(_)
                | AppError::InternalError(_)
                | AppError::ConfigError(_)
                | AppError::PasswordHashError
                | AppError::ExternalApiError(_)
                | AppError::ServiceUnavailable(_)
        )
    }

    /// Get the public error message (hides internal details for security)
    fn public_message(&self) -> String {
        match self {
            // Hide internal error details from clients
            AppError::DatabaseError(_) => "A database error occurred".to_string(),
            AppError::CacheError(_) => "A cache error occurred".to_string(),
            AppError::InternalError(_) => "An internal error occurred".to_string(),
            AppError::ConfigError(_) => "A configuration error occurred".to_string(),
            AppError::PasswordHashError => "An authentication error occurred".to_string(),

            // Return the actual error message for other errors
            _ => self.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();

        // Log errors appropriately
        if self.should_log_as_error() {
            error!(
                error = %self,
                status = status.as_u16(),
                "Internal error occurred"
            );
        }

        let body = Json(ErrorResponse {
            error: self.public_message(),
            status: status.as_u16(),
            details: None,
        });

        (status, body).into_response()
    }
}

// Implement From for common error types

impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        // Check for duplicate key error by examining the error message
        let err_str = err.to_string();
        if err_str.contains("E11000") || err_str.contains("duplicate key") {
            return AppError::AlreadyExists("Resource already exists".to_string());
        }
        AppError::DatabaseError(err_str)
    }
}

impl From<mongodb::bson::oid::Error> for AppError {
    fn from(err: mongodb::bson::oid::Error) -> Self {
        AppError::InvalidInput(format!("Invalid ID format: {}", err))
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError::CacheError(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;
        match err.kind() {
            ErrorKind::ExpiredSignature => AppError::TokenExpired,
            ErrorKind::InvalidToken
            | ErrorKind::InvalidSignature
            | ErrorKind::InvalidAlgorithm
            | ErrorKind::InvalidKeyFormat => {
                AppError::InvalidToken(err.to_string())
            }
            _ => AppError::InvalidToken(err.to_string()),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AppError::ServiceUnavailable("External service timeout".to_string())
        } else if err.is_connect() {
            AppError::ServiceUnavailable("Could not connect to external service".to_string())
        } else {
            AppError::ExternalApiError(err.to_string())
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::InvalidInput(format!("JSON parsing error: {}", err))
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        // Format validation errors nicely
        let messages: Vec<String> = err
            .field_errors()
            .into_iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |e| {
                    format!(
                        "{}: {}",
                        field,
                        e.message.clone().unwrap_or_else(|| "invalid value".into())
                    )
                })
            })
            .collect();
        AppError::ValidationError(messages.join("; "))
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(_err: argon2::password_hash::Error) -> Self {
        // Don't expose password hashing details
        AppError::PasswordHashError
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_codes() {
        assert_eq!(
            AppError::InvalidCredentials.status_code(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            AppError::NotFound("test".to_string()).status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            AppError::ValidationError("test".to_string()).status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppError::RateLimitExceeded.status_code(),
            StatusCode::TOO_MANY_REQUESTS
        );
        assert_eq!(
            AppError::DatabaseError("test".to_string()).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            AppError::ExternalApiError("test".to_string()).status_code(),
            StatusCode::BAD_GATEWAY
        );
    }

    #[test]
    fn test_public_message_hides_internal_details() {
        let db_error = AppError::DatabaseError("Connection failed: password wrong".to_string());
        assert_eq!(db_error.public_message(), "A database error occurred");

        let internal_error = AppError::InternalError("Stack trace here".to_string());
        assert_eq!(internal_error.public_message(), "An internal error occurred");

        // Non-internal errors show actual message
        let not_found = AppError::NotFound("User".to_string());
        assert_eq!(not_found.public_message(), "Resource not found: User");
    }

    #[test]
    fn test_error_display() {
        let error = AppError::NotFound("User".to_string());
        assert_eq!(error.to_string(), "Resource not found: User");

        let error = AppError::ValidationError("Email is required".to_string());
        assert_eq!(error.to_string(), "Validation error: Email is required");
    }
}
