//! Unit tests for JWT token creation and validation

use chrono::Utc;
use mongodb::bson::oid::ObjectId;

use my_invest_backend::{
    config::JwtConfig,
    utils::{
        error::AppError,
        jwt::{extract_bearer_token, Claims, JwtManager, TokenType},
    },
};

fn create_jwt_manager() -> JwtManager {
    let config = JwtConfig {
        secret: "test-secret-key-for-testing".to_string(),
        access_expiration: 3600,      // 1 hour
        refresh_expiration: 86400,    // 1 day
        issuer: "test-issuer".to_string(),
    };
    JwtManager::new(&config)
}

#[test]
fn test_create_access_token() {
    let jwt = create_jwt_manager();
    let user_id = ObjectId::new();
    let email = "test@example.com";

    let token = jwt
        .create_token(&user_id, email, TokenType::Access)
        .expect("Token creation should succeed");

    assert!(!token.is_empty());

    // Token should have three parts separated by dots
    let parts: Vec<&str> = token.split('.').collect();
    assert_eq!(parts.len(), 3);
}

#[test]
fn test_create_refresh_token() {
    let jwt = create_jwt_manager();
    let user_id = ObjectId::new();
    let email = "test@example.com";

    let token = jwt
        .create_token(&user_id, email, TokenType::Refresh)
        .expect("Token creation should succeed");

    let claims = jwt
        .validate_token(&token)
        .expect("Token validation should succeed");

    assert!(claims.is_refresh_token());
    assert!(!claims.is_access_token());
}

#[test]
fn test_create_token_pair() {
    let jwt = create_jwt_manager();
    let user_id = ObjectId::new();
    let email = "test@example.com";

    let pair = jwt
        .create_token_pair(&user_id, email)
        .expect("Token pair creation should succeed");

    assert!(!pair.access_token.is_empty());
    assert!(!pair.refresh_token.is_empty());
    assert_eq!(pair.token_type, "Bearer");
    assert_eq!(pair.expires_in, 3600);

    // Verify access token
    let access_claims = jwt
        .validate_access_token(&pair.access_token)
        .expect("Access token should be valid");
    assert!(access_claims.is_access_token());
    assert_eq!(access_claims.email, email);

    // Verify refresh token
    let refresh_claims = jwt
        .validate_refresh_token(&pair.refresh_token)
        .expect("Refresh token should be valid");
    assert!(refresh_claims.is_refresh_token());
}

#[test]
fn test_validate_token_success() {
    let jwt = create_jwt_manager();
    let user_id = ObjectId::new();
    let email = "test@example.com";

    let token = jwt
        .create_token(&user_id, email, TokenType::Access)
        .expect("Token creation should succeed");

    let claims = jwt
        .validate_token(&token)
        .expect("Token validation should succeed");

    assert_eq!(claims.sub, user_id.to_hex());
    assert_eq!(claims.email, email);
    assert_eq!(claims.iss, "test-issuer");
    assert!(claims.exp > Utc::now().timestamp());
}

#[test]
fn test_validate_token_invalid() {
    let jwt = create_jwt_manager();

    let result = jwt.validate_token("invalid.token.here");

    assert!(result.is_err());
}

#[test]
fn test_validate_token_wrong_secret() {
    let jwt1 = create_jwt_manager();

    let config2 = JwtConfig {
        secret: "different-secret".to_string(),
        access_expiration: 3600,
        refresh_expiration: 86400,
        issuer: "test-issuer".to_string(),
    };
    let jwt2 = JwtManager::new(&config2);

    let user_id = ObjectId::new();
    let token = jwt1
        .create_token(&user_id, "test@example.com", TokenType::Access)
        .expect("Token creation should succeed");

    let result = jwt2.validate_token(&token);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AppError::InvalidToken(_)));
}

#[test]
fn test_validate_token_expired() {
    let config = JwtConfig {
        secret: "test-secret".to_string(),
        access_expiration: -1, // Already expired
        refresh_expiration: -1,
        issuer: "test-issuer".to_string(),
    };
    let jwt = JwtManager::new(&config);

    let user_id = ObjectId::new();
    let token = jwt
        .create_token(&user_id, "test@example.com", TokenType::Access)
        .expect("Token creation should succeed");

    let result = jwt.validate_token(&token);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AppError::TokenExpired));
}

#[test]
fn test_validate_access_token_with_refresh_token() {
    let jwt = create_jwt_manager();
    let user_id = ObjectId::new();

    let refresh_token = jwt
        .create_token(&user_id, "test@example.com", TokenType::Refresh)
        .expect("Token creation should succeed");

    let result = jwt.validate_access_token(&refresh_token);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AppError::InvalidToken(_)));
}

#[test]
fn test_validate_refresh_token_with_access_token() {
    let jwt = create_jwt_manager();
    let user_id = ObjectId::new();

    let access_token = jwt
        .create_token(&user_id, "test@example.com", TokenType::Access)
        .expect("Token creation should succeed");

    let result = jwt.validate_refresh_token(&access_token);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AppError::InvalidToken(_)));
}

#[test]
fn test_claims_user_id() {
    let jwt = create_jwt_manager();
    let user_id = ObjectId::new();
    let email = "test@example.com";

    let token = jwt
        .create_token(&user_id, email, TokenType::Access)
        .expect("Token creation should succeed");

    let claims = jwt
        .validate_token(&token)
        .expect("Token validation should succeed");

    let extracted_id = claims.user_id().expect("User ID extraction should succeed");
    assert_eq!(extracted_id, user_id);
}

#[test]
fn test_claims_user_id_invalid() {
    let claims = Claims {
        sub: "not-a-valid-object-id".to_string(),
        email: "test@example.com".to_string(),
        exp: Utc::now().timestamp() + 3600,
        iat: Utc::now().timestamp(),
        iss: "test".to_string(),
        token_type: TokenType::Access,
        jti: uuid::Uuid::new_v4().to_string(),
    };

    let result = claims.user_id();

    assert!(result.is_err());
}

#[test]
fn test_token_has_unique_jti() {
    let jwt = create_jwt_manager();
    let user_id = ObjectId::new();

    let token1 = jwt
        .create_token(&user_id, "test@example.com", TokenType::Access)
        .expect("Token creation should succeed");

    let token2 = jwt
        .create_token(&user_id, "test@example.com", TokenType::Access)
        .expect("Token creation should succeed");

    let claims1 = jwt.validate_token(&token1).expect("Validation should succeed");
    let claims2 = jwt.validate_token(&token2).expect("Validation should succeed");

    // JTI should be unique for each token
    assert_ne!(claims1.jti, claims2.jti);
}

#[test]
fn test_extract_bearer_token_valid() {
    assert_eq!(
        extract_bearer_token("Bearer abc123xyz"),
        Some("abc123xyz")
    );
}

#[test]
fn test_extract_bearer_token_empty() {
    assert_eq!(extract_bearer_token("Bearer "), Some(""));
}

#[test]
fn test_extract_bearer_token_no_prefix() {
    assert_eq!(extract_bearer_token("abc123xyz"), None);
}

#[test]
fn test_extract_bearer_token_wrong_prefix() {
    assert_eq!(extract_bearer_token("Basic abc123xyz"), None);
}

#[test]
fn test_extract_bearer_token_case_sensitive() {
    // "bearer" (lowercase) should not match
    assert_eq!(extract_bearer_token("bearer abc123xyz"), None);
    assert_eq!(extract_bearer_token("BEARER abc123xyz"), None);
}

#[test]
fn test_claims_is_access_token() {
    let claims = Claims {
        sub: ObjectId::new().to_hex(),
        email: "test@example.com".to_string(),
        exp: Utc::now().timestamp() + 3600,
        iat: Utc::now().timestamp(),
        iss: "test".to_string(),
        token_type: TokenType::Access,
        jti: uuid::Uuid::new_v4().to_string(),
    };

    assert!(claims.is_access_token());
    assert!(!claims.is_refresh_token());
}

#[test]
fn test_claims_is_refresh_token() {
    let claims = Claims {
        sub: ObjectId::new().to_hex(),
        email: "test@example.com".to_string(),
        exp: Utc::now().timestamp() + 86400,
        iat: Utc::now().timestamp(),
        iss: "test".to_string(),
        token_type: TokenType::Refresh,
        jti: uuid::Uuid::new_v4().to_string(),
    };

    assert!(claims.is_refresh_token());
    assert!(!claims.is_access_token());
}

#[test]
fn test_jwt_manager_expiration_getters() {
    let jwt = create_jwt_manager();

    assert_eq!(jwt.access_expiration(), 3600);
    assert_eq!(jwt.refresh_expiration(), 86400);
}
