//! Authentication HTTP handlers
//!
//! This module contains handlers for authentication endpoints:
//! - POST /api/v1/auth/register
//! - POST /api/v1/auth/login
//! - POST /api/v1/auth/logout
//! - POST /api/v1/auth/refresh
//! - POST /api/v1/auth/reset-password (stub)
//! - POST /api/v1/auth/reset-password/confirm (stub)

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::{info, instrument};
use validator::Validate;

use crate::middleware::auth::RequireAuth;
use crate::models::user::{
    LoginRequest, LogoutResponse, MessageResponse, PasswordResetConfirmRequest,
    PasswordResetRequest, RefreshTokenRequest, RegisterRequest,
};
use crate::services::AuthService;
use crate::utils::error::AppError;

/// Handler for user registration
///
/// POST /api/v1/auth/register
#[instrument(skip(auth_service, request))]
pub async fn register(
    State(auth_service): State<AuthService>,
    Json(request): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    request.validate()?;

    info!(email = %request.email, "Processing registration request");

    let response = auth_service.register(request).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

/// Handler for user login
///
/// POST /api/v1/auth/login
#[instrument(skip(auth_service, request))]
pub async fn login(
    State(auth_service): State<AuthService>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    request.validate()?;

    info!(email = %request.email, "Processing login request");

    let response = auth_service.login(request).await?;

    Ok(Json(response))
}

/// Handler for user logout
///
/// POST /api/v1/auth/logout
#[instrument(skip(auth_service))]
pub async fn logout(
    State(auth_service): State<AuthService>,
    RequireAuth(user): RequireAuth,
    refresh_token: Option<Json<RefreshTokenRequest>>,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %user.id, "Processing logout request");

    let token = refresh_token.map(|rt| rt.refresh_token.clone());

    auth_service.logout(token, &user.id).await?;

    Ok(Json(LogoutResponse {
        message: "Logged out successfully".to_string(),
    }))
}

/// Handler for token refresh
///
/// POST /api/v1/auth/refresh
#[instrument(skip(auth_service, request))]
pub async fn refresh_token(
    State(auth_service): State<AuthService>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    info!("Processing token refresh request");

    let response = auth_service.refresh_token(request).await?;

    Ok(Json(response))
}

/// Handler for password reset request
///
/// POST /api/v1/auth/reset-password
///
/// Initiates a password reset by generating a secure token and storing it
/// in the database. In a production environment, this token would be sent
/// via email. Currently, the token is logged for development purposes.
///
/// For security, this endpoint always returns success even if the email
/// doesn't exist, to prevent email enumeration attacks.
#[instrument(skip(auth_service, request))]
pub async fn request_password_reset(
    State(auth_service): State<AuthService>,
    Json(request): Json<PasswordResetRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    // Validate request
    request.validate()?;

    info!(email = %request.email, "Password reset requested");

    // Process the password reset request
    let token = auth_service.request_password_reset(request.clone()).await?;

    // Log the token for development purposes (in production, this would be sent via email)
    if let Some(ref plain_token) = token {
        info!(
            email = %request.email,
            token = %plain_token,
            "Password reset token generated (would be sent via email in production)"
        );
    }

    // Always return success message to prevent email enumeration
    Ok(Json(MessageResponse {
        message: "If an account with this email exists, you will receive a password reset link shortly.".to_string(),
    }))
}

/// Handler for password reset confirmation
///
/// POST /api/v1/auth/reset-password/confirm
///
/// Validates the reset token, updates the user's password, and invalidates
/// all existing sessions (refresh tokens) for the user.
#[instrument(skip(auth_service, request))]
pub async fn confirm_password_reset(
    State(auth_service): State<AuthService>,
    Json(request): Json<PasswordResetConfirmRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    // Validate request
    request.validate()?;

    info!("Password reset confirmation requested");

    // Process the password reset confirmation
    auth_service.confirm_password_reset(request).await?;

    Ok(Json(MessageResponse {
        message: "Password has been reset successfully. Please log in with your new password.".to_string(),
    }))
}

/// Handler for getting current user info
///
/// GET /api/v1/auth/me
#[instrument(skip(auth_service))]
pub async fn get_current_user(
    State(auth_service): State<AuthService>,
    RequireAuth(user): RequireAuth,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %user.id, "Fetching current user info");

    let user_data = auth_service.get_user(&user.id).await?;

    Ok(Json(crate::models::user::UserResponse::from(user_data)))
}

#[cfg(test)]
mod tests {
    // Handler tests are in the integration test suite
    // See tests/integration/api/auth_test.rs
}
