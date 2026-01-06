//! Alert API integration tests
//!
//! These tests verify the alert endpoints:
//! - GET /api/v1/alerts
//! - POST /api/v1/alerts
//! - GET /api/v1/alerts/{id}
//! - PUT /api/v1/alerts/{id}
//! - DELETE /api/v1/alerts/{id}
//! - PATCH /api/v1/alerts/{id}/toggle

use once_cell::sync::Lazy;
use serial_test::serial;
use testcontainers::clients::Cli;

// Shared Docker client for testcontainers
static DOCKER: Lazy<Cli> = Lazy::new(Cli::default);

#[path = "../../common/mod.rs"]
mod common;

use common::TestApp;

// ============================================================================
// CRUD Endpoint Tests
// ============================================================================

#[tokio::test]
#[serial]
async fn test_create_alert_success() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("alert@example.com", "AlertPass123!").await;

    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": 150.0,
            "condition": "above",
            "alert_type": "one_time"
        }))
        .await;

    assert_eq!(response.status_code(), 201);

    let body: serde_json::Value = response.json();
    assert!(body.get("id").is_some());
    assert_eq!(body["symbol"].as_str().unwrap(), "AAPL");
    assert_eq!(body["target_price"].as_f64().unwrap(), 150.0);
    assert_eq!(body["condition"].as_str().unwrap(), "above");
    assert_eq!(body["alert_type"].as_str().unwrap(), "one_time");
    assert_eq!(body["status"].as_str().unwrap(), "active");
    assert_eq!(body["trigger_count"].as_u64().unwrap(), 0);
    assert!(body.get("created_at").is_some());
    assert!(body.get("updated_at").is_some());
}

#[tokio::test]
#[serial]
async fn test_create_alert_invalid_symbol() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("invalidsym@example.com", "InvalidSymPass123!").await;

    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "INVALID@SYMBOL!",
            "target_price": 100.0,
            "condition": "above",
            "alert_type": "one_time"
        }))
        .await;

    // Should be 404 - symbol not found
    assert_eq!(response.status_code(), 404);

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().unwrap().contains("not found"));
}

#[tokio::test]
#[serial]
async fn test_create_alert_invalid_price() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("invalidprice@example.com", "InvalidPricePass123!").await;

    // Negative price
    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": -10.0,
            "condition": "above",
            "alert_type": "one_time"
        }))
        .await;

    assert_eq!(response.status_code(), 400);

    // Zero price
    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": 0.0,
            "condition": "above",
            "alert_type": "one_time"
        }))
        .await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
#[serial]
async fn test_create_alert_limit_exceeded() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("limit@example.com", "LimitPass123!").await;

    // Create 50 alerts (max limit)
    for i in 1..=50 {
        let response = app.server
            .post("/api/v1/alerts")
            .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
            .json(&serde_json::json!({
                "symbol": "AAPL",
                "target_price": 100.0 + (i as f64),
                "condition": "above",
                "alert_type": "one_time"
            }))
            .await;

        assert_eq!(response.status_code(), 201);
    }

    // Try to create 51st alert - should fail
    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": 999.0,
            "condition": "above",
            "alert_type": "one_time"
        }))
        .await;

    assert_eq!(response.status_code(), 429); // Too Many Requests / Limit Exceeded

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().unwrap().contains("Maximum"));
    assert!(body["error"].as_str().unwrap().contains("50"));
}

#[tokio::test]
#[serial]
async fn test_create_alert_duplicate() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("duplicate@example.com", "DuplicatePass123!").await;

    // Create first alert
    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": 150.0,
            "condition": "above",
            "alert_type": "one_time"
        }))
        .await;

    assert_eq!(response.status_code(), 201);

    // Try to create duplicate alert (same symbol, condition, target_price, status=active)
    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": 150.0,
            "condition": "above",
            "alert_type": "one_time"
        }))
        .await;

    assert_eq!(response.status_code(), 409); // Conflict

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().unwrap().contains("already exists"));
}

#[tokio::test]
#[serial]
async fn test_list_alerts_empty() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("emptylist@example.com", "EmptyListPass123!").await;

    let response = app.server
        .get("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert_eq!(body["total"].as_u64().unwrap(), 0);
    assert_eq!(body["alerts"].as_array().unwrap().len(), 0);
}

#[tokio::test]
#[serial]
async fn test_list_alerts_with_data() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("listalerts@example.com", "ListAlertsPass123!").await;

    // Create multiple alerts
    app.create_alert(&access_token, "AAPL", 150.0, "above").await;
    app.create_alert(&access_token, "AAPL", 140.0, "below").await;
    app.create_alert(&access_token, "AAPL", 145.0, "crosses").await;

    let response = app.server
        .get("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert_eq!(body["total"].as_u64().unwrap(), 3);

    let alerts = body["alerts"].as_array().unwrap();
    assert_eq!(alerts.len(), 3);

    // Verify structure of first alert
    let first = &alerts[0];
    assert!(first.get("id").is_some());
    assert!(first.get("symbol").is_some());
    assert!(first.get("target_price").is_some());
    assert!(first.get("condition").is_some());
    assert!(first.get("alert_type").is_some());
    assert!(first.get("status").is_some());
    assert!(first.get("trigger_count").is_some());
    assert!(first.get("created_at").is_some());
    assert!(first.get("updated_at").is_some());
}

#[tokio::test]
#[serial]
async fn test_get_alert_success() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("getalert@example.com", "GetAlertPass123!").await;

    let alert_id = app.create_alert(&access_token, "AAPL", 150.0, "above").await;

    let response = app.server
        .get(&format!("/api/v1/alerts/{}", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert_eq!(body["id"].as_str().unwrap(), alert_id);
    assert_eq!(body["symbol"].as_str().unwrap(), "AAPL");
    assert_eq!(body["target_price"].as_f64().unwrap(), 150.0);
    assert_eq!(body["condition"].as_str().unwrap(), "above");
}

#[tokio::test]
#[serial]
async fn test_get_alert_not_found() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("notfound@example.com", "NotFoundPass123!").await;

    // Use a valid ObjectId format but non-existent
    let response = app.server
        .get("/api/v1/alerts/507f1f77bcf86cd799439011")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 404);

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().unwrap().contains("not found"));
}

#[tokio::test]
#[serial]
async fn test_get_alert_wrong_user() {
    let app = TestApp::new(&DOCKER).await;

    // User A creates an alert
    let (token_a, _) = app.register_user("usera@example.com", "UserAPass123!").await;
    let alert_id = app.create_alert(&token_a, "AAPL", 150.0, "above").await;

    // User B tries to access User A's alert
    let (token_b, _) = app.register_user("userb@example.com", "UserBPass123!").await;

    let response = app.server
        .get(&format!("/api/v1/alerts/{}", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", token_b).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 404); // Should appear not found for security
}

#[tokio::test]
#[serial]
async fn test_update_alert_success() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("update@example.com", "UpdatePass123!").await;

    let alert_id = app.create_alert(&access_token, "AAPL", 150.0, "above").await;

    // Update target price
    let response = app.server
        .put(&format!("/api/v1/alerts/{}", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "target_price": 175.0
        }))
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert_eq!(body["target_price"].as_f64().unwrap(), 175.0);
    assert_eq!(body["symbol"].as_str().unwrap(), "AAPL"); // Unchanged
    assert_eq!(body["condition"].as_str().unwrap(), "above"); // Unchanged
}

#[tokio::test]
#[serial]
async fn test_update_alert_partial() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("partial@example.com", "PartialPass123!").await;

    let alert_id = app.create_alert(&access_token, "AAPL", 150.0, "above").await;

    // Update only condition and alert_type
    let response = app.server
        .put(&format!("/api/v1/alerts/{}", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "condition": "below",
            "alert_type": "recurring"
        }))
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert_eq!(body["condition"].as_str().unwrap(), "below");
    assert_eq!(body["alert_type"].as_str().unwrap(), "recurring");
    assert_eq!(body["target_price"].as_f64().unwrap(), 150.0); // Unchanged
}

#[tokio::test]
#[serial]
async fn test_delete_alert_success() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("delete@example.com", "DeletePass123!").await;

    let alert_id = app.create_alert(&access_token, "AAPL", 150.0, "above").await;

    // Delete the alert
    let response = app.server
        .delete(&format!("/api/v1/alerts/{}", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert!(body["message"].as_str().unwrap().contains("success"));

    // Verify it's gone
    let response = app.server
        .get(&format!("/api/v1/alerts/{}", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 404);
}

#[tokio::test]
#[serial]
async fn test_toggle_alert_active_to_disabled() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("toggle1@example.com", "TogglePass123!").await;

    let alert_id = app.create_alert(&access_token, "AAPL", 150.0, "above").await;

    // Alert starts as Active
    let response = app.server
        .get(&format!("/api/v1/alerts/{}", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    let body: serde_json::Value = response.json();
    assert_eq!(body["status"].as_str().unwrap(), "active");

    // Toggle to Disabled
    let response = app.server
        .patch(&format!("/api/v1/alerts/{}/toggle", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert_eq!(body["status"].as_str().unwrap(), "disabled");
}

#[tokio::test]
#[serial]
async fn test_toggle_alert_disabled_to_active() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("toggle2@example.com", "TogglePass123!").await;

    let alert_id = app.create_alert(&access_token, "AAPL", 150.0, "above").await;

    // Toggle to Disabled
    app.server
        .patch(&format!("/api/v1/alerts/{}/toggle", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    // Toggle back to Active
    let response = app.server
        .patch(&format!("/api/v1/alerts/{}/toggle", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert_eq!(body["status"].as_str().unwrap(), "active");
}

// ============================================================================
// Authorization Tests
// ============================================================================

#[tokio::test]
#[serial]
async fn test_alerts_require_authentication() {
    let app = TestApp::new(&DOCKER).await;

    // Test all endpoints without authentication

    // GET /api/v1/alerts
    let response = app.server
        .get("/api/v1/alerts")
        .await;
    assert_eq!(response.status_code(), 401);

    // POST /api/v1/alerts
    let response = app.server
        .post("/api/v1/alerts")
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": 150.0,
            "condition": "above",
            "alert_type": "one_time"
        }))
        .await;
    assert_eq!(response.status_code(), 401);

    // GET /api/v1/alerts/{id}
    let response = app.server
        .get("/api/v1/alerts/507f1f77bcf86cd799439011")
        .await;
    assert_eq!(response.status_code(), 401);

    // PUT /api/v1/alerts/{id}
    let response = app.server
        .put("/api/v1/alerts/507f1f77bcf86cd799439011")
        .json(&serde_json::json!({
            "target_price": 175.0
        }))
        .await;
    assert_eq!(response.status_code(), 401);

    // DELETE /api/v1/alerts/{id}
    let response = app.server
        .delete("/api/v1/alerts/507f1f77bcf86cd799439011")
        .await;
    assert_eq!(response.status_code(), 401);

    // PATCH /api/v1/alerts/{id}/toggle
    let response = app.server
        .patch("/api/v1/alerts/507f1f77bcf86cd799439011/toggle")
        .await;
    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
#[serial]
async fn test_alert_user_isolation() {
    let app = TestApp::new(&DOCKER).await;

    // User A creates 3 alerts
    let (token_a, _) = app.register_user("isolation_a@example.com", "IsolationAPass123!").await;
    app.create_alert(&token_a, "AAPL", 150.0, "above").await;
    app.create_alert(&token_a, "AAPL", 140.0, "below").await;
    app.create_alert(&token_a, "AAPL", 145.0, "crosses").await;

    // User B creates 2 alerts
    let (token_b, _) = app.register_user("isolation_b@example.com", "IsolationBPass123!").await;
    app.create_alert(&token_b, "AAPL", 160.0, "above").await;
    app.create_alert(&token_b, "AAPL", 130.0, "below").await;

    // User A should only see their 3 alerts
    let response = app.server
        .get("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", token_a).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 200);
    let body: serde_json::Value = response.json();
    assert_eq!(body["total"].as_u64().unwrap(), 3);

    // User B should only see their 2 alerts
    let response = app.server
        .get("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", token_b).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 200);
    let body: serde_json::Value = response.json();
    assert_eq!(body["total"].as_u64().unwrap(), 2);
}

// ============================================================================
// Workflow Tests
// ============================================================================

#[tokio::test]
#[serial]
async fn test_comprehensive_alert_workflow() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("workflow@example.com", "WorkflowPass123!").await;

    // 1. Create an alert
    let alert_id = app.create_alert(&access_token, "AAPL", 150.0, "above").await;

    // 2. Get the alert
    let response = app.server
        .get(&format!("/api/v1/alerts/{}", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;
    assert_eq!(response.status_code(), 200);

    // 3. Update the alert
    let response = app.server
        .put(&format!("/api/v1/alerts/{}", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "target_price": 175.0,
            "note": "Updated target price"
        }))
        .await;
    assert_eq!(response.status_code(), 200);
    let body: serde_json::Value = response.json();
    assert_eq!(body["target_price"].as_f64().unwrap(), 175.0);
    assert_eq!(body["note"].as_str().unwrap(), "Updated target price");

    // 4. Toggle to disabled
    let response = app.server
        .patch(&format!("/api/v1/alerts/{}/toggle", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;
    assert_eq!(response.status_code(), 200);
    let body: serde_json::Value = response.json();
    assert_eq!(body["status"].as_str().unwrap(), "disabled");

    // 5. Toggle back to active
    let response = app.server
        .patch(&format!("/api/v1/alerts/{}/toggle", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;
    assert_eq!(response.status_code(), 200);
    let body: serde_json::Value = response.json();
    assert_eq!(body["status"].as_str().unwrap(), "active");

    // 6. List all alerts
    let response = app.server
        .get("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;
    assert_eq!(response.status_code(), 200);
    let body: serde_json::Value = response.json();
    assert_eq!(body["total"].as_u64().unwrap(), 1);

    // 7. Delete the alert
    let response = app.server
        .delete(&format!("/api/v1/alerts/{}", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;
    assert_eq!(response.status_code(), 200);

    // 8. Verify it's gone
    let response = app.server
        .get(&format!("/api/v1/alerts/{}", alert_id))
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;
    assert_eq!(response.status_code(), 404);

    // 9. List should be empty
    let response = app.server
        .get("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;
    assert_eq!(response.status_code(), 200);
    let body: serde_json::Value = response.json();
    assert_eq!(body["total"].as_u64().unwrap(), 0);
}

// ============================================================================
// Validation Tests
// ============================================================================

#[tokio::test]
#[serial]
async fn test_create_alert_validation_errors() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("validation@example.com", "ValidationPass123!").await;

    // Missing symbol
    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "target_price": 150.0,
            "condition": "above",
            "alert_type": "one_time"
        }))
        .await;
    assert_eq!(response.status_code(), 400);

    // Missing target_price
    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "condition": "above",
            "alert_type": "one_time"
        }))
        .await;
    assert_eq!(response.status_code(), 400);

    // Missing condition
    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": 150.0,
            "alert_type": "one_time"
        }))
        .await;
    assert_eq!(response.status_code(), 400);

    // Invalid condition value
    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": 150.0,
            "condition": "invalid_condition",
            "alert_type": "one_time"
        }))
        .await;
    assert_eq!(response.status_code(), 400);

    // Symbol too long (> 10 characters)
    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "VERYLONGSYMBOL123",
            "target_price": 150.0,
            "condition": "above",
            "alert_type": "one_time"
        }))
        .await;
    assert_eq!(response.status_code(), 400);

    // Note too long (> 500 characters)
    let long_note = "A".repeat(501);
    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": 150.0,
            "condition": "above",
            "alert_type": "one_time",
            "note": long_note
        }))
        .await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
#[serial]
async fn test_create_alert_with_note() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("withnote@example.com", "WithNotePass123!").await;

    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": 150.0,
            "condition": "above",
            "alert_type": "one_time",
            "note": "Buy signal - breakout expected"
        }))
        .await;

    assert_eq!(response.status_code(), 201);

    let body: serde_json::Value = response.json();
    assert_eq!(body["note"].as_str().unwrap(), "Buy signal - breakout expected");
}

#[tokio::test]
#[serial]
async fn test_create_alert_recurring_type() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("recurring@example.com", "RecurringPass123!").await;

    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": 150.0,
            "condition": "crosses",
            "alert_type": "recurring"
        }))
        .await;

    assert_eq!(response.status_code(), 201);

    let body: serde_json::Value = response.json();
    assert_eq!(body["alert_type"].as_str().unwrap(), "recurring");
}

#[tokio::test]
#[serial]
async fn test_update_alert_not_found() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("updatenotfound@example.com", "UpdateNotFoundPass123!").await;

    let response = app.server
        .put("/api/v1/alerts/507f1f77bcf86cd799439011")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "target_price": 175.0
        }))
        .await;

    assert_eq!(response.status_code(), 404);
}

#[tokio::test]
#[serial]
async fn test_delete_alert_not_found() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("deletenotfound@example.com", "DeleteNotFoundPass123!").await;

    let response = app.server
        .delete("/api/v1/alerts/507f1f77bcf86cd799439011")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 404);
}

#[tokio::test]
#[serial]
async fn test_toggle_alert_not_found() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("togglenotfound@example.com", "ToggleNotFoundPass123!").await;

    let response = app.server
        .patch("/api/v1/alerts/507f1f77bcf86cd799439011/toggle")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 404);
}

#[tokio::test]
#[serial]
async fn test_get_alert_invalid_id_format() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("invalidid@example.com", "InvalidIdPass123!").await;

    let response = app.server
        .get("/api/v1/alerts/not-a-valid-object-id")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 400);

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().unwrap().contains("Invalid ID format"));
}

#[tokio::test]
#[serial]
async fn test_create_alert_different_conditions() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("conditions@example.com", "ConditionsPass123!").await;

    // Test "above" condition
    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": 150.0,
            "condition": "above",
            "alert_type": "one_time"
        }))
        .await;
    assert_eq!(response.status_code(), 201);
    let body: serde_json::Value = response.json();
    assert_eq!(body["condition"].as_str().unwrap(), "above");

    // Test "below" condition
    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": 140.0,
            "condition": "below",
            "alert_type": "one_time"
        }))
        .await;
    assert_eq!(response.status_code(), 201);
    let body: serde_json::Value = response.json();
    assert_eq!(body["condition"].as_str().unwrap(), "below");

    // Test "crosses" condition
    let response = app.server
        .post("/api/v1/alerts")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .json(&serde_json::json!({
            "symbol": "AAPL",
            "target_price": 145.0,
            "condition": "crosses",
            "alert_type": "one_time"
        }))
        .await;
    assert_eq!(response.status_code(), 201);
    let body: serde_json::Value = response.json();
    assert_eq!(body["condition"].as_str().unwrap(), "crosses");
}
