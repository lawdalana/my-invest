//! Authentication API integration tests
//!
//! These tests verify the authentication endpoints:
//! - POST /api/v1/auth/register
//! - POST /api/v1/auth/login
//! - POST /api/v1/auth/logout
//! - POST /api/v1/auth/refresh
//! - GET /api/v1/auth/me

use once_cell::sync::Lazy;
use serial_test::serial;
use testcontainers::clients::Cli;

// Shared Docker client for testcontainers
static DOCKER: Lazy<Cli> = Lazy::new(Cli::default);

#[path = "../../common/mod.rs"]
mod common;

use common::TestApp;

#[tokio::test]
#[serial]
async fn test_register_success() {
    let app = TestApp::new(&DOCKER).await;

    let response = app.server
        .post("/api/v1/auth/register")
        .json(&serde_json::json!({
            "email": "newuser@example.com",
            "password": "SecurePass123!",
            "password_confirm": "SecurePass123!"
        }))
        .await;

    assert_eq!(response.status_code(), 201);

    let body: serde_json::Value = response.json();

    // Verify response structure
    assert!(body.get("access_token").is_some());
    assert!(body.get("refresh_token").is_some());
    assert_eq!(body["token_type"].as_str().unwrap(), "Bearer");
    assert!(body["expires_in"].as_u64().is_some());

    // Verify user data
    let user = &body["user"];
    assert!(user.get("id").is_some());
    assert_eq!(user["email"].as_str().unwrap(), "newuser@example.com");
    assert!(user.get("created_at").is_some());

    // Verify preferences
    assert_eq!(user["preferences"]["theme"].as_str().unwrap(), "light");
    assert_eq!(user["preferences"]["default_timeframe"].as_str().unwrap(), "1D");
    assert_eq!(user["preferences"]["notifications"].as_bool().unwrap(), true);
}

#[tokio::test]
#[serial]
async fn test_register_duplicate_email() {
    let app = TestApp::new(&DOCKER).await;

    // Register first user
    let email = "duplicate@example.com";
    app.register_user(email, "Password123!").await;

    // Attempt to register with same email
    let response = app.server
        .post("/api/v1/auth/register")
        .json(&serde_json::json!({
            "email": email,
            "password": "DifferentPass123!",
            "password_confirm": "DifferentPass123!"
        }))
        .await;

    assert_eq!(response.status_code(), 409); // Conflict

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().unwrap().contains("already exists"));
}

#[tokio::test]
#[serial]
async fn test_register_invalid_email() {
    let app = TestApp::new(&DOCKER).await;

    let response = app.server
        .post("/api/v1/auth/register")
        .json(&serde_json::json!({
            "email": "not-an-email",
            "password": "Password123!",
            "password_confirm": "Password123!"
        }))
        .await;

    assert_eq!(response.status_code(), 400); // Bad Request

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().is_some());
}

#[tokio::test]
#[serial]
async fn test_register_password_too_short() {
    let app = TestApp::new(&DOCKER).await;

    let response = app.server
        .post("/api/v1/auth/register")
        .json(&serde_json::json!({
            "email": "user@example.com",
            "password": "short1",
            "password_confirm": "short1"
        }))
        .await;

    assert_eq!(response.status_code(), 400);

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().unwrap().to_lowercase().contains("password"));
}

#[tokio::test]
#[serial]
async fn test_register_password_mismatch() {
    let app = TestApp::new(&DOCKER).await;

    let response = app.server
        .post("/api/v1/auth/register")
        .json(&serde_json::json!({
            "email": "user@example.com",
            "password": "Password123!",
            "password_confirm": "DifferentPassword123!"
        }))
        .await;

    assert_eq!(response.status_code(), 400);

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().unwrap().contains("match"));
}

#[tokio::test]
#[serial]
async fn test_register_weak_password() {
    let app = TestApp::new(&DOCKER).await;

    // Password without numbers
    let response = app.server
        .post("/api/v1/auth/register")
        .json(&serde_json::json!({
            "email": "user1@example.com",
            "password": "OnlyLetters",
            "password_confirm": "OnlyLetters"
        }))
        .await;

    assert_eq!(response.status_code(), 400);

    // Password without letters
    let response = app.server
        .post("/api/v1/auth/register")
        .json(&serde_json::json!({
            "email": "user2@example.com",
            "password": "12345678",
            "password_confirm": "12345678"
        }))
        .await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
#[serial]
async fn test_login_success() {
    let app = TestApp::new(&DOCKER).await;

    // Register user first
    let email = "logintest@example.com";
    let password = "LoginPass123!";
    app.register_user(email, password).await;

    // Login
    let response = app.server
        .post("/api/v1/auth/login")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert!(body.get("access_token").is_some());
    assert!(body.get("refresh_token").is_some());
    assert_eq!(body["token_type"].as_str().unwrap(), "Bearer");
}

#[tokio::test]
#[serial]
async fn test_login_invalid_credentials() {
    let app = TestApp::new(&DOCKER).await;

    // Register user
    let email = "wrongpass@example.com";
    app.register_user(email, "CorrectPass123!").await;

    // Login with wrong password
    let response = app.server
        .post("/api/v1/auth/login")
        .json(&serde_json::json!({
            "email": email,
            "password": "WrongPassword123!"
        }))
        .await;

    assert_eq!(response.status_code(), 401); // Unauthorized

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().unwrap().contains("Invalid"));
}

#[tokio::test]
#[serial]
async fn test_login_nonexistent_user() {
    let app = TestApp::new(&DOCKER).await;

    let response = app.server
        .post("/api/v1/auth/login")
        .json(&serde_json::json!({
            "email": "nonexistent@example.com",
            "password": "SomePassword123!"
        }))
        .await;

    assert_eq!(response.status_code(), 401);

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().unwrap().contains("Invalid"));
}

#[tokio::test]
#[serial]
async fn test_refresh_token_success() {
    let app = TestApp::new(&DOCKER).await;

    // Register and get tokens
    let (_, refresh_token) = app.register_user("refresh@example.com", "RefreshPass123!").await;

    // Use refresh token to get new tokens
    let response = app.server
        .post("/api/v1/auth/refresh")
        .json(&serde_json::json!({
            "refresh_token": refresh_token
        }))
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert!(body.get("access_token").is_some());
    assert!(body.get("refresh_token").is_some());

    // New tokens should be different from original
    let new_refresh = body["refresh_token"].as_str().unwrap();
    assert_ne!(new_refresh, refresh_token);
}

#[tokio::test]
#[serial]
async fn test_refresh_token_invalid() {
    let app = TestApp::new(&DOCKER).await;

    let response = app.server
        .post("/api/v1/auth/refresh")
        .json(&serde_json::json!({
            "refresh_token": "invalid.token.here"
        }))
        .await;

    assert_eq!(response.status_code(), 401);

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().is_some());
}

#[tokio::test]
#[serial]
async fn test_get_current_user_success() {
    let app = TestApp::new(&DOCKER).await;

    let email = "currentuser@example.com";
    let (access_token, _) = app.register_user(email, "UserPass123!").await;

    // Get current user info
    let response = app.server
        .get("/api/v1/auth/me")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert!(body.get("id").is_some());
    assert_eq!(body["email"].as_str().unwrap(), email);
    assert!(body.get("preferences").is_some());
}

#[tokio::test]
#[serial]
async fn test_protected_route_without_token() {
    let app = TestApp::new(&DOCKER).await;

    let response = app.server
        .get("/api/v1/auth/me")
        .await;

    assert_eq!(response.status_code(), 401);

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().unwrap().contains("token"));
}

#[tokio::test]
#[serial]
async fn test_protected_route_with_invalid_token() {
    let app = TestApp::new(&DOCKER).await;

    let response = app.server
        .get("/api/v1/auth/me")
        .add_header("Authorization".parse().unwrap(), "Bearer invalid.token.here".parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
#[serial]
async fn test_logout_success() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, refresh_token) = app.register_user("logout@example.com", "LogoutPass123!").await;

    // Logout
    let response = app.server
        .post("/api/v1/auth/logout")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "refresh_token": refresh_token
        }))
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert!(body["message"].as_str().unwrap().contains("success"));

    // Try to use the refresh token after logout (should fail)
    let response = app.server
        .post("/api/v1/auth/refresh")
        .json(&serde_json::json!({
            "refresh_token": refresh_token
        }))
        .await;

    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
#[serial]
async fn test_missing_fields() {
    let app = TestApp::new(&DOCKER).await;

    // Register without email
    let response = app.server
        .post("/api/v1/auth/register")
        .json(&serde_json::json!({
            "password": "Password123!",
            "password_confirm": "Password123!"
        }))
        .await;

    assert_eq!(response.status_code(), 400);

    // Login without password
    let response = app.server
        .post("/api/v1/auth/login")
        .json(&serde_json::json!({
            "email": "test@example.com"
        }))
        .await;

    assert_eq!(response.status_code(), 400);
}
