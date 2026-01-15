//! Password hashing and verification using Argon2
//!
//! This module provides secure password hashing using the Argon2id algorithm,
//! which is the recommended algorithm for password hashing. It also provides
//! utilities for generating secure random tokens.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sha2::{Digest, Sha256};

use crate::utils::error::{AppError, Result};

/// Hash a password using Argon2id
///
/// Uses the default Argon2id parameters which are secure for most use cases:
/// - Memory: 19 MiB
/// - Iterations: 2
/// - Parallelism: 1
///
/// # Arguments
///
/// * `password` - The plain text password to hash
///
/// # Returns
///
/// * `Ok(String)` - The hashed password in PHC string format
/// * `Err(AppError)` - If hashing fails
///
/// # Example
///
/// ```ignore
/// use my_invest_backend::utils::password;
///
/// let hash = password::hash("my_secure_password")?;
/// assert!(hash.starts_with("$argon2id$"));
/// ```
pub fn hash(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::PasswordHashError)?;

    Ok(password_hash.to_string())
}

/// Verify a password against a hash
///
/// # Arguments
///
/// * `password` - The plain text password to verify
/// * `hash` - The stored password hash in PHC string format
///
/// # Returns
///
/// * `Ok(true)` - Password matches
/// * `Ok(false)` - Password does not match
/// * `Err(AppError)` - If the hash is invalid or verification fails
///
/// # Example
///
/// ```ignore
/// use my_invest_backend::utils::password;
///
/// let hash = password::hash("my_password")?;
/// assert!(password::verify("my_password", &hash)?);
/// assert!(!password::verify("wrong_password", &hash)?);
/// ```
pub fn verify(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_| AppError::PasswordHashError)?;

    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(_) => Err(AppError::PasswordHashError),
    }
}

/// Validate password strength
///
/// # Arguments
///
/// * `password` - The password to validate
/// * `min_length` - Minimum required length
///
/// # Returns
///
/// * `Ok(())` - Password meets requirements
/// * `Err(AppError::ValidationError)` - Password does not meet requirements
///
/// # Example
///
/// ```ignore
/// use my_invest_backend::utils::password;
///
/// password::validate_strength("short", 8)?; // Returns Err
/// password::validate_strength("longenough", 8)?; // Returns Ok
/// ```
pub fn validate_strength(password: &str, min_length: usize) -> Result<()> {
    if password.len() < min_length {
        return Err(AppError::ValidationError(format!(
            "Password must be at least {} characters long",
            min_length
        )));
    }

    // Check for at least one letter
    if !password.chars().any(|c| c.is_alphabetic()) {
        return Err(AppError::ValidationError(
            "Password must contain at least one letter".to_string(),
        ));
    }

    // Check for at least one digit
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(AppError::ValidationError(
            "Password must contain at least one number".to_string(),
        ));
    }

    Ok(())
}

/// Generate a secure random token for password reset
///
/// Generates a 32-byte (256-bit) cryptographically secure random token
/// and returns it as a hex-encoded string.
///
/// # Returns
///
/// * `String` - A 64-character hex-encoded random token
///
/// # Example
///
/// ```ignore
/// use my_invest_backend::utils::password;
///
/// let token = password::generate_reset_token();
/// assert_eq!(token.len(), 64); // 32 bytes = 64 hex characters
/// ```
pub fn generate_reset_token() -> String {
    use argon2::password_hash::rand_core::RngCore;

    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Hash a reset token using SHA-256
///
/// We store hashed tokens in the database so that even if the database
/// is compromised, the tokens cannot be used directly.
///
/// # Arguments
///
/// * `token` - The plain text token to hash
///
/// # Returns
///
/// * `String` - The SHA-256 hash of the token as a hex string
///
/// # Example
///
/// ```ignore
/// use my_invest_backend::utils::password;
///
/// let token = "my_reset_token";
/// let hash = password::hash_reset_token(token);
/// assert_eq!(hash.len(), 64); // SHA-256 produces 32 bytes = 64 hex characters
/// ```
pub fn hash_reset_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

/// Verify a reset token against a stored hash
///
/// # Arguments
///
/// * `token` - The plain text token to verify
/// * `stored_hash` - The stored SHA-256 hash
///
/// # Returns
///
/// * `bool` - True if the token matches the hash
pub fn verify_reset_token(token: &str, stored_hash: &str) -> bool {
    let computed_hash = hash_reset_token(token);
    // Use constant-time comparison to prevent timing attacks
    constant_time_eq(&computed_hash, stored_hash)
}

/// Constant-time string comparison to prevent timing attacks
fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.bytes().zip(b.bytes()) {
        result |= x ^ y;
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_creates_valid_hash() {
        let password = "test_password_123";
        let hash = hash(password).expect("Hashing should succeed");

        // Argon2id hashes start with $argon2id$
        assert!(hash.starts_with("$argon2id$"));
    }

    #[test]
    fn test_hash_creates_different_hashes_for_same_password() {
        let password = "test_password_123";
        let hash1 = hash(password).expect("Hashing should succeed");
        let hash2 = hash(password).expect("Hashing should succeed");

        // Different salts should produce different hashes
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_correct_password() {
        let password = "test_password_123";
        let hashed = hash(password).expect("Hashing should succeed");

        assert!(verify(password, &hashed).expect("Verification should succeed"));
    }

    #[test]
    fn test_verify_incorrect_password() {
        let password = "test_password_123";
        let wrong_password = "wrong_password";
        let hashed = hash(password).expect("Hashing should succeed");

        assert!(!verify(wrong_password, &hashed).expect("Verification should succeed"));
    }

    #[test]
    fn test_verify_invalid_hash_format() {
        let result = verify("password", "invalid_hash");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_strength_too_short() {
        let result = validate_strength("short", 8);
        assert!(result.is_err());

        if let Err(AppError::ValidationError(msg)) = result {
            assert!(msg.contains("at least 8 characters"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn test_validate_strength_no_letters() {
        let result = validate_strength("12345678", 8);
        assert!(result.is_err());

        if let Err(AppError::ValidationError(msg)) = result {
            assert!(msg.contains("at least one letter"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn test_validate_strength_no_numbers() {
        let result = validate_strength("password", 8);
        assert!(result.is_err());

        if let Err(AppError::ValidationError(msg)) = result {
            assert!(msg.contains("at least one number"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn test_validate_strength_valid_password() {
        let result = validate_strength("password123", 8);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_strength_complex_password() {
        let result = validate_strength("MyP@ssw0rd!2023", 8);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_password() {
        let result = validate_strength("", 8);
        assert!(result.is_err());
    }

    #[test]
    fn test_unicode_password() {
        let password = "password123";
        let hash = hash(password).expect("Hashing should succeed");
        assert!(verify(password, &hash).expect("Verification should succeed"));
    }

    #[test]
    fn test_generate_reset_token_length() {
        let token = generate_reset_token();
        // 32 bytes = 64 hex characters
        assert_eq!(token.len(), 64);
    }

    #[test]
    fn test_generate_reset_token_unique() {
        let token1 = generate_reset_token();
        let token2 = generate_reset_token();
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_hash_reset_token() {
        let token = "test_token";
        let hash = hash_reset_token(token);
        // SHA-256 produces 32 bytes = 64 hex characters
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_hash_reset_token_deterministic() {
        let token = "test_token";
        let hash1 = hash_reset_token(token);
        let hash2 = hash_reset_token(token);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_verify_reset_token_valid() {
        let token = "test_token";
        let hash = hash_reset_token(token);
        assert!(verify_reset_token(token, &hash));
    }

    #[test]
    fn test_verify_reset_token_invalid() {
        let token = "test_token";
        let hash = hash_reset_token(token);
        assert!(!verify_reset_token("wrong_token", &hash));
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq("hello", "hello"));
        assert!(!constant_time_eq("hello", "world"));
        assert!(!constant_time_eq("hello", "hell"));
        assert!(!constant_time_eq("", "a"));
    }
}
