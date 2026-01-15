//! Authentication service
//!
//! This service handles user registration, login, logout, and token refresh
//! operations. It coordinates between password hashing, JWT management,
//! and database operations.

use chrono::{Duration, Utc};
use mongodb::bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use mongodb::Collection;
use tracing::{info, instrument, warn};

use crate::config::database::MongoDb;
use crate::models::user::{
    AuthResponse, LoginRequest, PasswordResetConfirmRequest, PasswordResetRequest,
    PasswordResetToken, RefreshToken, RefreshTokenRequest, RegisterRequest, User, UserResponse,
};
use crate::utils::error::{AppError, Result};
use crate::utils::jwt::JwtManager;
use crate::utils::password;

/// Authentication service for handling user authentication
#[derive(Clone)]
pub struct AuthService {
    users: Collection<User>,
    refresh_tokens: Collection<RefreshToken>,
    password_reset_tokens: Collection<PasswordResetToken>,
    jwt: JwtManager,
    min_password_length: usize,
}

impl AuthService {
    /// Create a new AuthService instance
    pub fn new(db: &MongoDb, jwt: JwtManager, min_password_length: usize) -> Self {
        Self {
            users: db.database().collection("users"),
            refresh_tokens: db.database().collection("refresh_tokens"),
            password_reset_tokens: db.database().collection("password_reset_tokens"),
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

    /// Request a password reset
    ///
    /// Generates a secure reset token, stores it hashed in the database,
    /// and returns the plain token (for logging/email). The token expires
    /// after 1 hour.
    ///
    /// For security, this method always succeeds (returns Ok) even if the
    /// email doesn't exist, to prevent email enumeration attacks.
    ///
    /// # Arguments
    ///
    /// * `request` - Password reset request containing the email
    ///
    /// # Returns
    ///
    /// * `Ok(Option<String>)` - The plain reset token if user exists, None otherwise
    #[instrument(skip(self, request), fields(email = %request.email))]
    pub async fn request_password_reset(
        &self,
        request: PasswordResetRequest,
    ) -> Result<Option<String>> {
        info!("Processing password reset request");

        // Find user by email
        let user = self
            .users
            .find_one(doc! { "email": &request.email.to_lowercase() }, None)
            .await?;

        // If user doesn't exist, return Ok(None) to prevent email enumeration
        let user = match user {
            Some(u) => u,
            None => {
                info!("Password reset requested for non-existent email");
                return Ok(None);
            }
        };

        // Invalidate any existing password reset tokens for this user
        self.password_reset_tokens
            .update_many(
                doc! { "user_id": user.id, "used": false },
                doc! { "$set": { "used": true } },
                None,
            )
            .await?;

        // Generate a secure random token using split-token pattern
        let token_parts = password::generate_reset_token();
        let verifier_hash = password::hash_reset_token(&token_parts.verifier)?;

        // Token expires in 1 hour
        let expires_at = Utc::now() + Duration::hours(1);

        // Create and store the token with selector and verifier hash
        let reset_token = PasswordResetToken::new(
            user.id,
            token_parts.selector,
            verifier_hash,
            expires_at,
        );
        self.password_reset_tokens.insert_one(&reset_token, None).await?;

        info!(user_id = %user.id, "Password reset token created");

        // Return the full token to be sent to the user (via email)
        Ok(Some(token_parts.full_token))
    }

    /// Confirm a password reset
    ///
    /// Validates the reset token, updates the user's password, and invalidates
    /// all existing refresh tokens for the user.
    ///
    /// # Arguments
    ///
    /// * `request` - Password reset confirmation request with token and new password
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Password reset successful
    /// * `Err(AppError::InvalidToken)` - If token is invalid, expired, or already used
    /// * `Err(AppError::ValidationError)` - If password doesn't meet requirements
    #[instrument(skip(self, request))]
    pub async fn confirm_password_reset(
        &self,
        request: PasswordResetConfirmRequest,
    ) -> Result<()> {
        info!("Processing password reset confirmation");

        // Validate password strength
        password::validate_strength(&request.new_password, self.min_password_length)?;

        // Parse the token into selector and verifier
        let (selector, verifier) = password::parse_reset_token(&request.token)?;

        // Find the token by selector (fast indexed lookup)
        let stored_token = self
            .password_reset_tokens
            .find_one(
                doc! {
                    "selector": &selector,
                    "used": false
                },
                None,
            )
            .await?
            .ok_or_else(|| {
                warn!("Password reset token not found or already used");
                AppError::InvalidToken("Invalid or expired reset token".to_string())
            })?;

        // Check if token is valid (not expired)
        if !stored_token.is_valid() {
            warn!("Password reset token expired");
            return Err(AppError::InvalidToken("Reset token has expired".to_string()));
        }

        // Verify the verifier against the stored hash (cryptographically secure)
        let is_valid = password::verify_reset_token(&verifier, &stored_token.verifier_hash)?;
        if !is_valid {
            warn!("Password reset token verifier mismatch");
            return Err(AppError::InvalidToken("Invalid or expired reset token".to_string()));
        }

        // Get the user
        let user = self
            .users
            .find_one(doc! { "_id": stored_token.user_id }, None)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Hash the new password
        let password_hash = password::hash(&request.new_password)?;

        // Update the user's password
        let updated_at = BsonDateTime::now();
        self.users
            .update_one(
                doc! { "_id": user.id },
                doc! {
                    "$set": {
                        "password_hash": password_hash,
                        "updated_at": updated_at
                    }
                },
                None,
            )
            .await?;

        // Mark the reset token as used
        self.password_reset_tokens
            .update_one(
                doc! { "_id": stored_token.id },
                doc! { "$set": { "used": true } },
                None,
            )
            .await?;

        // Invalidate all refresh tokens for this user (force re-login)
        self.refresh_tokens
            .update_many(
                doc! { "user_id": user.id },
                doc! { "$set": { "revoked": true } },
                None,
            )
            .await?;

        info!(user_id = %user.id, "Password reset completed successfully");

        Ok(())
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
