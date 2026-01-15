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

/// Reset token parts for the split-token security pattern
///
/// The split-token pattern provides both fast database lookups and
/// strong cryptographic security:
/// - `selector`: Used for database lookup (stored in plain text)
/// - `verifier`: Hashed with Argon2 for verification (stored as hash)
#[derive(Debug)]
pub struct ResetTokenParts {
    /// The selector portion used for database lookup (16 hex chars)
    pub selector: String,
    /// The verifier portion to be hashed with Argon2 (48 hex chars)
    pub verifier: String,
    /// The full token to return to the user (64 hex chars)
    pub full_token: String,
}

/// Generate a secure random token for password reset using split-token pattern
///
/// Generates a 32-byte (256-bit) cryptographically secure random token
/// split into:
/// - Selector (8 bytes / 16 hex chars): stored in plain text for lookups
/// - Verifier (24 bytes / 48 hex chars): hashed with Argon2 for verification
///
/// This pattern provides:
/// - Fast O(1) database lookups via indexed selector
/// - Strong cryptographic protection via Argon2-hashed verifier
/// - Even with database compromise, attackers can't use tokens directly
///
/// # Returns
///
/// * `ResetTokenParts` - Containing selector, verifier, and full token
///
/// # Example
///
/// ```ignore
/// use my_invest_backend::utils::password;
///
/// let parts = password::generate_reset_token();
/// assert_eq!(parts.selector.len(), 16);  // 8 bytes hex-encoded
/// assert_eq!(parts.verifier.len(), 48);  // 24 bytes hex-encoded
/// assert_eq!(parts.full_token.len(), 64); // 32 bytes hex-encoded
/// ```
pub fn generate_reset_token() -> ResetTokenParts {
    use argon2::password_hash::rand_core::RngCore;

    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);

    let full_token = hex::encode(bytes);

    ResetTokenParts {
        selector: full_token[..16].to_string(),    // First 8 bytes
        verifier: full_token[16..].to_string(),    // Remaining 24 bytes
        full_token,
    }
}

/// Parse a reset token into selector and verifier parts
///
/// # Arguments
///
/// * `token` - The full 64-character hex token
///
/// # Returns
///
/// * `Ok((selector, verifier))` - The parsed parts
/// * `Err(AppError)` - If token format is invalid
pub fn parse_reset_token(token: &str) -> Result<(String, String)> {
    if token.len() != 64 {
        return Err(AppError::ValidationError(
            "Invalid reset token format".to_string()
        ));
    }

    // Validate it's all hex characters
    if !token.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(AppError::ValidationError(
            "Invalid reset token format".to_string()
        ));
    }

    Ok((token[..16].to_string(), token[16..].to_string()))
}

/// Hash a reset token verifier using Argon2id
///
/// We store hashed verifiers in the database so that even if the database
/// is compromised, the tokens cannot be brute-forced. Using Argon2id
/// provides strong protection against GPU-based attacks.
///
/// # Arguments
///
/// * `verifier` - The verifier portion of the reset token
///
/// # Returns
///
/// * `Ok(String)` - The Argon2id hash in PHC string format
/// * `Err(AppError)` - If hashing fails
pub fn hash_reset_token(verifier: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let verifier_hash = argon2
        .hash_password(verifier.as_bytes(), &salt)
        .map_err(|_| AppError::PasswordHashError)?;

    Ok(verifier_hash.to_string())
}

/// Verify a reset token verifier against a stored hash
///
/// Uses Argon2id verification which provides built-in timing-attack resistance.
///
/// # Arguments
///
/// * `verifier` - The verifier portion of the reset token
/// * `stored_hash` - The stored Argon2id hash in PHC string format
///
/// # Returns
///
/// * `Ok(true)` - Verifier matches
/// * `Ok(false)` - Verifier does not match
/// * `Err(AppError)` - If the hash format is invalid
pub fn verify_reset_token(verifier: &str, stored_hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(stored_hash)
        .map_err(|_| AppError::PasswordHashError)?;

    let argon2 = Argon2::default();

    match argon2.verify_password(verifier.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(_) => Err(AppError::PasswordHashError),
    }
}

/// Hash a value using SHA-256
///
/// Used internally for non-sensitive lookups only.
#[allow(dead_code)]
fn sha256_hash(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    hex::encode(hasher.finalize())
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
    fn test_generate_reset_token_parts() {
        let parts = generate_reset_token();
        // Selector: 8 bytes = 16 hex characters
        assert_eq!(parts.selector.len(), 16);
        // Verifier: 24 bytes = 48 hex characters
        assert_eq!(parts.verifier.len(), 48);
        // Full token: 32 bytes = 64 hex characters
        assert_eq!(parts.full_token.len(), 64);
        // Full token should be selector + verifier
        assert_eq!(format!("{}{}", parts.selector, parts.verifier), parts.full_token);
    }

    #[test]
    fn test_generate_reset_token_unique() {
        let parts1 = generate_reset_token();
        let parts2 = generate_reset_token();
        assert_ne!(parts1.full_token, parts2.full_token);
        assert_ne!(parts1.selector, parts2.selector);
    }

    #[test]
    fn test_parse_reset_token_valid() {
        let parts = generate_reset_token();
        let (selector, verifier) = parse_reset_token(&parts.full_token).expect("Parse should succeed");
        assert_eq!(selector, parts.selector);
        assert_eq!(verifier, parts.verifier);
    }

    #[test]
    fn test_parse_reset_token_invalid_length() {
        let result = parse_reset_token("too_short");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_reset_token_invalid_chars() {
        // 64 chars but with invalid hex
        let result = parse_reset_token("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz");
        assert!(result.is_err());
    }

    #[test]
    fn test_hash_reset_token() {
        let verifier = "test_verifier_string_for_hashing";
        let hash = hash_reset_token(verifier).expect("Hashing should succeed");
        // Argon2id hashes start with $argon2id$
        assert!(hash.starts_with("$argon2id$"));
    }

    #[test]
    fn test_hash_reset_token_different_salts() {
        let verifier = "test_verifier";
        let hash1 = hash_reset_token(verifier).expect("Hashing should succeed");
        let hash2 = hash_reset_token(verifier).expect("Hashing should succeed");
        // Different salts should produce different hashes
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_reset_token_valid() {
        let verifier = "test_verifier_for_verification";
        let hash = hash_reset_token(verifier).expect("Hashing should succeed");
        assert!(verify_reset_token(verifier, &hash).expect("Verification should succeed"));
    }

    #[test]
    fn test_verify_reset_token_invalid() {
        let verifier = "test_verifier";
        let hash = hash_reset_token(verifier).expect("Hashing should succeed");
        assert!(!verify_reset_token("wrong_verifier", &hash).expect("Verification should succeed"));
    }

    #[test]
    fn test_verify_reset_token_invalid_hash_format() {
        let result = verify_reset_token("verifier", "invalid_hash");
        assert!(result.is_err());
    }

    #[test]
    fn test_full_reset_token_flow() {
        // Generate token
        let parts = generate_reset_token();

        // Hash verifier (what gets stored in DB)
        let stored_hash = hash_reset_token(&parts.verifier).expect("Hashing should succeed");

        // Parse the full token (simulating user submitting token)
        let (selector, verifier) = parse_reset_token(&parts.full_token).expect("Parse should succeed");

        // Verify selector matches (for DB lookup)
        assert_eq!(selector, parts.selector);

        // Verify verifier against stored hash
        assert!(verify_reset_token(&verifier, &stored_hash).expect("Verification should succeed"));
    }
}
