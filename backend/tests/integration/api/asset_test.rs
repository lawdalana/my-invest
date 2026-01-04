//! Asset API integration tests
//!
//! These tests verify the asset endpoints:
//! - GET /api/v1/assets/search?q={query}
//! - GET /api/v1/assets/{symbol}
//! - GET /api/v1/assets/{symbol}/history?timeframe={1D|1W|1M|1Y}

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
async fn test_search_success() {
    let app = TestApp::new(&DOCKER).await;

    // Register and authenticate
    let (access_token, _) = app.register_user("assetuser@example.com", "AssetPass123!").await;

    // Search for assets
    let response = app.server
        .get("/api/v1/assets/search?q=AAPL")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert!(body.get("query").is_some());
    assert_eq!(body["query"].as_str().unwrap(), "AAPL");
    assert!(body.get("results").is_some());
    assert!(body.get("total").is_some());

    // Verify results structure
    let results = body["results"].as_array().unwrap();
    assert!(!results.is_empty());

    let first_result = &results[0];
    assert!(first_result.get("symbol").is_some());
    assert!(first_result.get("name").is_some());
    assert!(first_result.get("asset_type").is_some());
    assert!(first_result.get("region").is_some());
    assert!(first_result.get("currency").is_some());
}

#[tokio::test]
#[serial]
async fn test_search_empty_query() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("search@example.com", "SearchPass123!").await;

    // Search with empty query
    let response = app.server
        .get("/api/v1/assets/search?q=")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 400); // Bad Request

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().is_some());
}

#[tokio::test]
#[serial]
async fn test_search_missing_query() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("search2@example.com", "SearchPass123!").await;

    // Search without query parameter
    let response = app.server
        .get("/api/v1/assets/search")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
#[serial]
async fn test_search_unauthorized() {
    let app = TestApp::new(&DOCKER).await;

    // Search without authentication
    let response = app.server
        .get("/api/v1/assets/search?q=AAPL")
        .await;

    assert_eq!(response.status_code(), 401); // Unauthorized

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().is_some());
}

#[tokio::test]
#[serial]
async fn test_get_quote_success() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("quote@example.com", "QuotePass123!").await;

    // Get quote for AAPL
    let response = app.server
        .get("/api/v1/assets/AAPL")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();

    // Verify required fields
    assert_eq!(body["symbol"].as_str().unwrap(), "AAPL");
    assert!(body.get("name").is_some());
    assert!(body.get("asset_type").is_some());
    assert!(body.get("price").is_some());
    assert!(body.get("change_24h").is_some());
    assert!(body.get("change_percent_24h").is_some());
    assert!(body.get("volume_24h").is_some());
    assert!(body.get("last_updated").is_some());

    // Verify numeric fields
    assert!(body["price"].is_f64() || body["price"].is_u64());
    assert!(body["change_24h"].is_f64() || body["change_24h"].is_i64());

    // Verify OHLC fields
    assert!(body.get("open").is_some());
    assert!(body.get("high").is_some());
    assert!(body.get("low").is_some());
    assert!(body.get("previous_close").is_some());
}

#[tokio::test]
#[serial]
async fn test_get_quote_invalid_symbol() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("invalid@example.com", "InvalidPass123!").await;

    // Use invalid symbol format
    let response = app.server
        .get("/api/v1/assets/INVALID@SYMBOL")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    // Should return 400 for invalid format or 404 if it passes validation
    assert!(response.status_code() == 400 || response.status_code() == 404);
}

#[tokio::test]
#[serial]
async fn test_get_quote_unauthorized() {
    let app = TestApp::new(&DOCKER).await;

    let response = app.server
        .get("/api/v1/assets/AAPL")
        .await;

    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
#[serial]
async fn test_get_history_success() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("history@example.com", "HistoryPass123!").await;

    // Get daily history for AAPL
    let response = app.server
        .get("/api/v1/assets/AAPL/history?timeframe=1D")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();

    // Verify response structure
    assert_eq!(body["symbol"].as_str().unwrap(), "AAPL");
    assert_eq!(body["timeframe"].as_str().unwrap(), "1D");
    assert!(body.get("data").is_some());
    assert!(body.get("count").is_some());
    assert!(body.get("fetched_at").is_some());

    // Verify data points
    let data = body["data"].as_array().unwrap();
    assert!(!data.is_empty());

    let first_point = &data[0];
    assert!(first_point.get("timestamp").is_some());
    assert!(first_point.get("open").is_some());
    assert!(first_point.get("high").is_some());
    assert!(first_point.get("low").is_some());
    assert!(first_point.get("close").is_some());
    assert!(first_point.get("volume").is_some());
}

#[tokio::test]
#[serial]
async fn test_get_history_all_timeframes() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("timeframe@example.com", "TimeframePass123!").await;

    let valid_timeframes = ["1D", "1W", "1M", "1Y"];

    for timeframe in valid_timeframes {
        let response = app.server
            .get(&format!("/api/v1/assets/AAPL/history?timeframe={}", timeframe))
            .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
            .await;

        assert_eq!(
            response.status_code(),
            200,
            "Timeframe {} should be valid",
            timeframe
        );

        let body: serde_json::Value = response.json();
        assert_eq!(body["timeframe"].as_str().unwrap(), timeframe);
    }
}

#[tokio::test]
#[serial]
async fn test_get_history_invalid_timeframe() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("badtf@example.com", "BadTfPass123!").await;

    // Use invalid timeframe
    let response = app.server
        .get("/api/v1/assets/AAPL/history?timeframe=5Y")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 400);

    let body: serde_json::Value = response.json();
    assert!(body["error"].as_str().is_some());
}

#[tokio::test]
#[serial]
async fn test_get_history_missing_timeframe() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("notf@example.com", "NoTfPass123!").await;

    // Request without timeframe parameter
    let response = app.server
        .get("/api/v1/assets/AAPL/history")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
#[serial]
async fn test_get_history_unauthorized() {
    let app = TestApp::new(&DOCKER).await;

    let response = app.server
        .get("/api/v1/assets/AAPL/history?timeframe=1D")
        .await;

    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
#[serial]
async fn test_caching_behavior() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("cache@example.com", "CachePass123!").await;

    // First request (should fetch from API)
    let response1 = app.server
        .get("/api/v1/assets/AAPL")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response1.status_code(), 200);
    let body1: serde_json::Value = response1.json();

    // Second request immediately after (should use cache)
    let response2 = app.server
        .get("/api/v1/assets/AAPL")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response2.status_code(), 200);
    let body2: serde_json::Value = response2.json();

    // Data should be the same (from cache)
    assert_eq!(body1["price"], body2["price"]);
    assert_eq!(body1["last_updated"], body2["last_updated"]);
}

#[tokio::test]
#[serial]
async fn test_symbol_case_insensitive() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("case@example.com", "CasePass123!").await;

    // Test lowercase symbol (should be normalized to uppercase)
    let response = app.server
        .get("/api/v1/assets/aapl")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    // Symbol should be normalized to uppercase
    assert_eq!(body["symbol"].as_str().unwrap(), "AAPL");
}

#[tokio::test]
#[serial]
async fn test_search_results_relevance() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("relevance@example.com", "RelevancePass123!").await;

    let response = app.server
        .get("/api/v1/assets/search?q=AAPL")
        .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    let results = body["results"].as_array().unwrap();

    // Verify first result is most relevant
    if !results.is_empty() {
        let first = &results[0];
        assert_eq!(first["symbol"].as_str().unwrap(), "AAPL");

        // Match score should be present for search results
        if let Some(score) = first.get("match_score") {
            let score_val = score.as_f64().or_else(|| score.as_str().and_then(|s| s.parse().ok())).unwrap();
            assert!(score_val >= 0.0 && score_val <= 1.0, "Match score should be between 0 and 1");
        }
    }
}

#[tokio::test]
#[serial]
async fn test_concurrent_requests() {
    let app = TestApp::new(&DOCKER).await;

    let (access_token, _) = app.register_user("concurrent@example.com", "ConcurrentPass123!").await;

    // Make multiple concurrent requests
    let token = access_token.clone();
    let handle1 = {
        let server = app.server.clone();
        let token = token.clone();
        tokio::spawn(async move {
            server
                .get("/api/v1/assets/AAPL")
                .add_header("Authorization".parse().unwrap(), format!("Bearer {}", token).parse().unwrap())
                .await
        })
    };

    let token = access_token.clone();
    let handle2 = {
        let server = app.server.clone();
        let token = token.clone();
        tokio::spawn(async move {
            server
                .get("/api/v1/assets/GOOGL")
                .add_header("Authorization".parse().unwrap(), format!("Bearer {}", token).parse().unwrap())
                .await
        })
    };

    let token = access_token;
    let handle3 = {
        let server = app.server.clone();
        tokio::spawn(async move {
            server
                .get("/api/v1/assets/search?q=MSFT")
                .add_header("Authorization".parse().unwrap(), format!("Bearer {}", token).parse().unwrap())
                .await
        })
    };

    // All requests should succeed
    let response1 = handle1.await.unwrap();
    let response2 = handle2.await.unwrap();
    let response3 = handle3.await.unwrap();

    assert_eq!(response1.status_code(), 200);
    assert_eq!(response2.status_code(), 200);
    assert_eq!(response3.status_code(), 200);
}
