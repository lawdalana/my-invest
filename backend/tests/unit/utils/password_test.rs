//! Unit tests for password hashing and verification

use my_invest_backend::utils::{error::AppError, password};

#[test]
fn test_hash_password_success() {
    let password = "SecurePassword123!";
    let result = password::hash(password);

    assert!(result.is_ok());
    let hash = result.unwrap();

    // Argon2id hash should start with $argon2id$
    assert!(hash.starts_with("$argon2id$"));
    // Hash should be different from the original password
    assert_ne!(hash, password);
}

#[test]
fn test_hash_produces_different_hashes() {
    let password = "SecurePassword123!";

    let hash1 = password::hash(password).expect("First hash should succeed");
    let hash2 = password::hash(password).expect("Second hash should succeed");

    // Each hash should be unique due to random salt
    assert_ne!(hash1, hash2);
}

#[test]
fn test_verify_correct_password() {
    let password = "SecurePassword123!";
    let hash = password::hash(password).expect("Hashing should succeed");

    let result = password::verify(password, &hash);

    assert!(result.is_ok());
    assert!(result.unwrap(), "Correct password should verify");
}

#[test]
fn test_verify_incorrect_password() {
    let password = "SecurePassword123!";
    let wrong_password = "WrongPassword456!";
    let hash = password::hash(password).expect("Hashing should succeed");

    let result = password::verify(wrong_password, &hash);

    assert!(result.is_ok());
    assert!(!result.unwrap(), "Incorrect password should not verify");
}

#[test]
fn test_verify_invalid_hash_format() {
    let password = "SecurePassword123!";
    let invalid_hash = "not-a-valid-hash";

    let result = password::verify(password, invalid_hash);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AppError::PasswordHashError));
}

#[test]
fn test_verify_empty_password() {
    let password = "SecurePassword123!";
    let hash = password::hash(password).expect("Hashing should succeed");

    let result = password::verify("", &hash);

    assert!(result.is_ok());
    assert!(!result.unwrap(), "Empty password should not verify");
}

#[test]
fn test_validate_strength_valid_password() {
    let result = password::validate_strength("Password123", 8);
    assert!(result.is_ok());
}

#[test]
fn test_validate_strength_too_short() {
    let result = password::validate_strength("Pass1", 8);

    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("at least 8 characters"));
    } else {
        panic!("Expected ValidationError");
    }
}

#[test]
fn test_validate_strength_no_letters() {
    let result = password::validate_strength("12345678", 8);

    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("at least one letter"));
    } else {
        panic!("Expected ValidationError");
    }
}

#[test]
fn test_validate_strength_no_numbers() {
    let result = password::validate_strength("Password", 8);

    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("at least one number"));
    } else {
        panic!("Expected ValidationError");
    }
}

#[test]
fn test_validate_strength_empty_password() {
    let result = password::validate_strength("", 8);

    assert!(result.is_err());
}

#[test]
fn test_validate_strength_with_special_characters() {
    let result = password::validate_strength("P@ssw0rd!#$%", 8);
    assert!(result.is_ok());
}

#[test]
fn test_validate_strength_unicode() {
    // Unicode characters should count as letters
    let result = password::validate_strength("password1", 8);
    assert!(result.is_ok());
}

#[test]
fn test_hash_long_password() {
    // Test with a very long password
    let long_password = "a".repeat(1000) + "1";
    let result = password::hash(&long_password);

    assert!(result.is_ok());

    // Verify the long password works
    let hash = result.unwrap();
    let verify_result = password::verify(&long_password, &hash);
    assert!(verify_result.is_ok());
    assert!(verify_result.unwrap());
}

#[test]
fn test_hash_special_characters() {
    let special_password = "P@$$w0rd!#$%^&*()_+-=[]{}|;':\",./<>?";
    let result = password::hash(special_password);

    assert!(result.is_ok());

    let hash = result.unwrap();
    let verify_result = password::verify(special_password, &hash);
    assert!(verify_result.is_ok());
    assert!(verify_result.unwrap());
}

#[test]
fn test_validate_strength_custom_min_length() {
    // Test with minimum length of 12
    assert!(password::validate_strength("Password123", 12).is_err());
    assert!(password::validate_strength("Password12345", 12).is_ok());

    // Test with minimum length of 4
    assert!(password::validate_strength("Pw1", 4).is_err());
    assert!(password::validate_strength("Pw12", 4).is_ok());
}
