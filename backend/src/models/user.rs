//! User model and related DTOs
//!
//! This module defines the User entity and all request/response DTOs
//! for authentication operations.

use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// User entity stored in MongoDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier
    #[serde(rename = "_id")]
    pub id: ObjectId,

    /// User's email address (unique)
    pub email: String,

    /// Argon2 hashed password
    pub password_hash: String,

    /// Account creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,

    /// User preferences
    #[serde(default)]
    pub preferences: UserPreferences,
}

/// User preferences stored alongside the user profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// UI theme preference
    #[serde(default = "default_theme")]
    pub theme: String,

    /// Default chart timeframe
    #[serde(default = "default_timeframe")]
    pub default_timeframe: String,

    /// Whether notifications are enabled
    #[serde(default = "default_notifications")]
    pub notifications: bool,
}

fn default_theme() -> String {
    "light".to_string()
}

fn default_timeframe() -> String {
    "1D".to_string()
}

fn default_notifications() -> bool {
    true
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            default_timeframe: default_timeframe(),
            notifications: default_notifications(),
        }
    }
}

impl User {
    /// Create a new User instance
    pub fn new(email: String, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: ObjectId::new(),
            email,
            password_hash,
            created_at: now,
            updated_at: now,
            preferences: UserPreferences::default(),
        }
    }
}

/// Request DTO for user registration
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct RegisterRequest {
    /// User's email address
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    /// Plain text password (will be hashed)
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    /// Password confirmation
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub password_confirm: String,
}

/// Request DTO for user login
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginRequest {
    /// User's email address
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    /// Plain text password
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

/// Request DTO for token refresh
#[derive(Debug, Clone, Deserialize)]
pub struct RefreshTokenRequest {
    /// The refresh token
    pub refresh_token: String,
}

/// Request DTO for password reset request (stub for Phase 1)
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct PasswordResetRequest {
    /// User's email address
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

/// Request DTO for password reset confirmation (stub for Phase 1)
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct PasswordResetConfirmRequest {
    /// Password reset token
    pub token: String,

    /// New password
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub new_password: String,

    /// New password confirmation
    #[validate(must_match(other = "new_password", message = "Passwords do not match"))]
    pub new_password_confirm: String,
}

/// Response DTO for successful authentication
#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    /// JWT access token
    pub access_token: String,

    /// JWT refresh token
    pub refresh_token: String,

    /// Token type (always "Bearer")
    pub token_type: String,

    /// Access token expiration in seconds
    pub expires_in: i64,

    /// User information
    pub user: UserResponse,
}

/// User information included in auth response
#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    /// User ID
    pub id: String,

    /// User's email
    pub email: String,

    /// Account creation date
    pub created_at: DateTime<Utc>,

    /// User preferences
    pub preferences: UserPreferences,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_hex(),
            email: user.email,
            created_at: user.created_at,
            preferences: user.preferences,
        }
    }
}

/// Response for logout operation
#[derive(Debug, Clone, Serialize)]
pub struct LogoutResponse {
    pub message: String,
}

/// Response for simple message operations
#[derive(Debug, Clone, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

/// Refresh token entity stored in MongoDB
///
/// This is used to track refresh tokens and enable logout functionality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken {
    /// Unique identifier
    #[serde(rename = "_id")]
    pub id: ObjectId,

    /// Associated user ID
    pub user_id: ObjectId,

    /// The refresh token string
    pub token: String,

    /// Token expiration timestamp
    pub expires_at: DateTime<Utc>,

    /// Token creation timestamp
    pub created_at: DateTime<Utc>,

    /// Whether the token has been revoked
    #[serde(default)]
    pub revoked: bool,
}

impl RefreshToken {
    /// Create a new refresh token entry
    pub fn new(user_id: ObjectId, token: String, expires_at: DateTime<Utc>) -> Self {
        Self {
            id: ObjectId::new(),
            user_id,
            token,
            expires_at,
            created_at: Utc::now(),
            revoked: false,
        }
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Check if the token is valid (not expired and not revoked)
    pub fn is_valid(&self) -> bool {
        !self.revoked && !self.is_expired()
    }
}

/// Password reset token entity stored in MongoDB
///
/// Uses the split-token pattern for security:
/// - `selector`: Stored in plain text for fast O(1) database lookups
/// - `verifier_hash`: Argon2id hash of the verifier for cryptographic security
///
/// This provides both performance (indexed lookups) and security
/// (even with database access, tokens can't be brute-forced).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetToken {
    /// Unique identifier
    #[serde(rename = "_id")]
    pub id: ObjectId,

    /// Associated user ID
    pub user_id: ObjectId,

    /// Token selector for database lookup (first 16 chars of full token)
    /// Stored in plain text, indexed for fast lookups
    pub selector: String,

    /// Argon2id hash of the verifier (remaining 48 chars of full token)
    /// Even with DB access, attackers can't recover the verifier
    pub verifier_hash: String,

    /// Token expiration timestamp (typically 1 hour from creation)
    pub expires_at: DateTime<Utc>,

    /// Token creation timestamp
    pub created_at: DateTime<Utc>,

    /// Whether the token has been used
    #[serde(default)]
    pub used: bool,
}

impl PasswordResetToken {
    /// Create a new password reset token entry
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user requesting password reset
    /// * `selector` - The selector portion for database lookup
    /// * `verifier_hash` - Argon2id hash of the verifier
    /// * `expires_at` - When the token expires
    pub fn new(user_id: ObjectId, selector: String, verifier_hash: String, expires_at: DateTime<Utc>) -> Self {
        Self {
            id: ObjectId::new(),
            user_id,
            selector,
            verifier_hash,
            expires_at,
            created_at: Utc::now(),
            used: false,
        }
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Check if the token is valid (not expired and not used)
    pub fn is_valid(&self) -> bool {
        !self.used && !self.is_expired()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_new() {
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
        );

        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.password_hash, "hashed_password");
        assert!(user.created_at <= Utc::now());
        assert_eq!(user.preferences.theme, "light");
    }

    #[test]
    fn test_user_preferences_default() {
        let prefs = UserPreferences::default();

        assert_eq!(prefs.theme, "light");
        assert_eq!(prefs.default_timeframe, "1D");
        assert!(prefs.notifications);
    }

    #[test]
    fn test_user_response_from_user() {
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
        );
        let user_id = user.id;

        let response: UserResponse = user.into();

        assert_eq!(response.id, user_id.to_hex());
        assert_eq!(response.email, "test@example.com");
    }

    #[test]
    fn test_refresh_token_is_valid() {
        let token = RefreshToken::new(
            ObjectId::new(),
            "token".to_string(),
            Utc::now() + chrono::Duration::hours(1),
        );

        assert!(token.is_valid());
        assert!(!token.is_expired());
    }

    #[test]
    fn test_refresh_token_expired() {
        let token = RefreshToken::new(
            ObjectId::new(),
            "token".to_string(),
            Utc::now() - chrono::Duration::hours(1),
        );

        assert!(!token.is_valid());
        assert!(token.is_expired());
    }

    #[test]
    fn test_refresh_token_revoked() {
        let mut token = RefreshToken::new(
            ObjectId::new(),
            "token".to_string(),
            Utc::now() + chrono::Duration::hours(1),
        );
        token.revoked = true;

        assert!(!token.is_valid());
        assert!(!token.is_expired());
    }

    #[test]
    fn test_password_reset_token_is_valid() {
        let token = PasswordResetToken::new(
            ObjectId::new(),
            "selector123456ab".to_string(),
            "$argon2id$v=19$m=19456,t=2,p=1$hash".to_string(),
            Utc::now() + chrono::Duration::hours(1),
        );

        assert!(token.is_valid());
        assert!(!token.is_expired());
        assert!(!token.used);
    }

    #[test]
    fn test_password_reset_token_expired() {
        let token = PasswordResetToken::new(
            ObjectId::new(),
            "selector123456ab".to_string(),
            "$argon2id$v=19$m=19456,t=2,p=1$hash".to_string(),
            Utc::now() - chrono::Duration::hours(1),
        );

        assert!(!token.is_valid());
        assert!(token.is_expired());
    }

    #[test]
    fn test_password_reset_token_used() {
        let mut token = PasswordResetToken::new(
            ObjectId::new(),
            "selector123456ab".to_string(),
            "$argon2id$v=19$m=19456,t=2,p=1$hash".to_string(),
            Utc::now() + chrono::Duration::hours(1),
        );
        token.used = true;

        assert!(!token.is_valid());
        assert!(!token.is_expired());
    }
}
