//! Authentication service
//!
//! This service handles user registration, login, logout, and token refresh
//! operations. It coordinates between password hashing, JWT management,
//! and database operations.

use chrono::{Duration, Utc};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Collection;
use tracing::{info, instrument, warn};

use crate::config::database::MongoDb;
use crate::models::user::{
    AuthResponse, LoginRequest, RefreshToken, RefreshTokenRequest, RegisterRequest, User,
    UserResponse,
};
use crate::utils::error::{AppError, Result};
use crate::utils::jwt::JwtManager;
use crate::utils::password;

/// Authentication service for handling user authentication
#[derive(Clone)]
pub struct AuthService {
    users: Collection<User>,
    refresh_tokens: Collection<RefreshToken>,
    jwt: JwtManager,
    min_password_length: usize,
}

impl AuthService {
    /// Create a new AuthService instance
    pub fn new(db: &MongoDb, jwt: JwtManager, min_password_length: usize) -> Self {
        Self {
            users: db.database().collection("users"),
            refresh_tokens: db.database().collection("refresh_tokens"),
            jwt,
            min_password_length,
        }
    }

    /// Register a new user
    ///
    /// # Arguments
    ///
    /// * `request` - Registration request containing email and password
    ///
    /// # Returns
    ///
    /// * `Ok(AuthResponse)` - Authentication response with tokens
    /// * `Err(AppError)` - If registration fails
    #[instrument(skip(self, request), fields(email = %request.email))]
    pub async fn register(&self, request: RegisterRequest) -> Result<AuthResponse> {
        info!("Attempting to register new user");

        // Validate password strength
        password::validate_strength(&request.password, self.min_password_length)?;

        // Check if email already exists
        let existing = self
            .users
            .find_one(doc! { "email": &request.email.to_lowercase() }, None)
            .await?;

        if existing.is_some() {
            warn!("Registration failed: email already exists");
            return Err(AppError::AlreadyExists(
                "An account with this email already exists".to_string(),
            ));
        }

        // Hash password
        let password_hash = password::hash(&request.password)?;

        // Create user
        let user = User::new(request.email.to_lowercase(), password_hash);

        // Insert into database
        self.users.insert_one(&user, None).await?;

        info!(user_id = %user.id, "User registered successfully");

        // Generate tokens
        self.create_auth_response(&user).await
    }

    /// Login an existing user
    ///
    /// # Arguments
    ///
    /// * `request` - Login request containing email and password
    ///
    /// # Returns
    ///
    /// * `Ok(AuthResponse)` - Authentication response with tokens
    /// * `Err(AppError::InvalidCredentials)` - If credentials are invalid
    #[instrument(skip(self, request), fields(email = %request.email))]
    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse> {
        info!("Attempting user login");

        // Find user by email
        let user = self
            .users
            .find_one(doc! { "email": &request.email.to_lowercase() }, None)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        // Verify password
        let is_valid = password::verify(&request.password, &user.password_hash)?;

        if !is_valid {
            warn!("Login failed: invalid password");
            return Err(AppError::InvalidCredentials);
        }

        info!(user_id = %user.id, "User logged in successfully");

        // Generate tokens
        self.create_auth_response(&user).await
    }

    /// Refresh access token using a refresh token
    ///
    /// # Arguments
    ///
    /// * `request` - Refresh token request
    ///
    /// # Returns
    ///
    /// * `Ok(AuthResponse)` - New authentication response with fresh tokens
    /// * `Err(AppError)` - If token is invalid or expired
    #[instrument(skip(self, request))]
    pub async fn refresh_token(&self, request: RefreshTokenRequest) -> Result<AuthResponse> {
        info!("Attempting token refresh");

        // Validate refresh token
        let claims = self.jwt.validate_refresh_token(&request.refresh_token)?;

        // Check if token is in database and not revoked
        let stored_token = self
            .refresh_tokens
            .find_one(
                doc! {
                    "token": &request.refresh_token,
                    "revoked": false
                },
                None,
            )
            .await?
            .ok_or_else(|| AppError::InvalidToken("Refresh token not found or revoked".to_string()))?;

        if !stored_token.is_valid() {
            warn!("Token refresh failed: token is expired or revoked");
            return Err(AppError::TokenExpired);
        }

        // Get user
        let user_id = claims.user_id()?;
        let user = self
            .users
            .find_one(doc! { "_id": user_id }, None)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Revoke the old refresh token
        self.refresh_tokens
            .update_one(
                doc! { "_id": stored_token.id },
                doc! { "$set": { "revoked": true } },
                None,
            )
            .await?;

        info!(user_id = %user.id, "Token refreshed successfully");

        // Generate new tokens
        self.create_auth_response(&user).await
    }

    /// Logout user by revoking their refresh token
    ///
    /// # Arguments
    ///
    /// * `refresh_token` - The refresh token to revoke
    /// * `user_id` - The user's ID
    ///
    /// # Returns
    ///
    /// * `Ok(())` - On successful logout
    #[instrument(skip(self, refresh_token))]
    pub async fn logout(&self, refresh_token: Option<String>, user_id: &ObjectId) -> Result<()> {
        info!(user_id = %user_id, "User logging out");

        if let Some(token) = refresh_token {
            // Revoke the specific token
            self.refresh_tokens
                .update_one(
                    doc! { "token": token, "user_id": user_id },
                    doc! { "$set": { "revoked": true } },
                    None,
                )
                .await?;
        } else {
            // Revoke all user's refresh tokens
            self.refresh_tokens
                .update_many(
                    doc! { "user_id": user_id },
                    doc! { "$set": { "revoked": true } },
                    None,
                )
                .await?;
        }

        info!("Logout successful");
        Ok(())
    }

    /// Get user by ID
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user's ObjectId
    ///
    /// # Returns
    ///
    /// * `Ok(User)` - The user if found
    /// * `Err(AppError::NotFound)` - If user doesn't exist
    pub async fn get_user(&self, user_id: &ObjectId) -> Result<User> {
        self.users
            .find_one(doc! { "_id": user_id }, None)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))
    }

    /// Create authentication response with tokens
    async fn create_auth_response(&self, user: &User) -> Result<AuthResponse> {
        // Generate token pair
        let token_pair = self.jwt.create_token_pair(&user.id, &user.email)?;

        // Store refresh token in database
        let refresh_expiration = Utc::now() + Duration::seconds(self.jwt.refresh_expiration());
        let refresh_token_entry = RefreshToken::new(
            user.id,
            token_pair.refresh_token.clone(),
            refresh_expiration,
        );

        self.refresh_tokens
            .insert_one(&refresh_token_entry, None)
            .await?;

        Ok(AuthResponse {
            access_token: token_pair.access_token,
            refresh_token: token_pair.refresh_token,
            token_type: token_pair.token_type,
            expires_in: token_pair.expires_in,
            user: UserResponse::from(user.clone()),
        })
    }
}

#[cfg(test)]
mod tests {
    // Integration tests require database connection
    // See tests/integration/api/auth_test.rs
}
