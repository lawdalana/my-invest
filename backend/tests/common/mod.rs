//! Common test utilities and fixtures
//!
//! This module provides shared utilities for testing including:
//! - Test configuration
//! - Mock servers for external APIs
//! - Database test helpers
//! - Test fixtures

pub mod test_app;

use std::sync::Arc;

use mongodb::bson::oid::ObjectId;
use once_cell::sync::Lazy;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

use my_invest_backend::{
    config::{
        AlphaVantageConfig, AppConfig, CorsConfig, JwtConfig, MongoDbConfig, RateLimitConfig,
        RedisConfig, ServerConfig, Config,
    },
    utils::jwt::JwtManager,
};

pub use test_app::TestApp;

/// Test JWT secret
pub static TEST_JWT_SECRET: &str = "test-secret-key-for-testing-only-do-not-use-in-production";

/// Create a test configuration
pub fn create_test_config() -> Config {
    Config {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 0, // Random available port
            environment: "test".to_string(),
        },
        jwt: JwtConfig {
            secret: TEST_JWT_SECRET.to_string(),
            access_expiration: 3600,
            refresh_expiration: 86400,
            issuer: "test-issuer".to_string(),
        },
        mongodb: MongoDbConfig {
            uri: "mongodb://localhost:27017".to_string(),
            database: "my_invest_test".to_string(),
            min_pool_size: 1,
            max_pool_size: 5,
        },
        redis: RedisConfig {
            url: "redis://localhost:6379".to_string(),
            cache_ttl: 60,
            history_cache_ttl: 120,
        },
        alpha_vantage: AlphaVantageConfig {
            api_key: "test-api-key".to_string(),
            base_url: "http://localhost:8888".to_string(), // Will be overridden by mock
            rate_limit: 5,
            daily_limit: 500,
        },
        rate_limit: RateLimitConfig {
            requests: 100,
            window_seconds: 60,
        },
        cors: CorsConfig {
            allowed_origins: vec![
                "http://localhost:3000".to_string(),
                "http://localhost:5173".to_string(),
            ],
        },
        app: AppConfig {
            max_watchlists_per_user: 20,
            max_assets_per_watchlist: 50,
            min_password_length: 8,
        },
    }
}

/// Create a test JWT manager
pub fn create_test_jwt_manager() -> JwtManager {
    let config = JwtConfig {
        secret: TEST_JWT_SECRET.to_string(),
        access_expiration: 3600,
        refresh_expiration: 86400,
        issuer: "test-issuer".to_string(),
    };
    JwtManager::new(&config)
}

/// Generate a test access token for a user
pub fn generate_test_token(user_id: &ObjectId, email: &str) -> String {
    let jwt = create_test_jwt_manager();
    jwt.create_token(user_id, email, my_invest_backend::utils::jwt::TokenType::Access)
        .expect("Failed to create test token")
}

/// Create a mock Alpha Vantage server
pub async fn create_mock_alpha_vantage() -> MockServer {
    let mock_server = MockServer::start().await;

    // Mock SYMBOL_SEARCH endpoint
    Mock::given(method("GET"))
        .and(query_param("function", "SYMBOL_SEARCH"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bestMatches": [
                {
                    "1. symbol": "AAPL",
                    "2. name": "Apple Inc.",
                    "3. type": "Equity",
                    "4. region": "United States",
                    "8. currency": "USD",
                    "9. matchScore": "1.0000"
                },
                {
                    "1. symbol": "AAPL.L",
                    "2. name": "Apple Inc.",
                    "3. type": "Equity",
                    "4. region": "United Kingdom",
                    "8. currency": "GBP",
                    "9. matchScore": "0.8000"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    // Mock GLOBAL_QUOTE endpoint
    Mock::given(method("GET"))
        .and(query_param("function", "GLOBAL_QUOTE"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "Global Quote": {
                "01. symbol": "AAPL",
                "02. open": "150.00",
                "03. high": "155.00",
                "04. low": "149.00",
                "05. price": "152.50",
                "06. volume": "50000000",
                "07. latest trading day": "2024-01-15",
                "08. previous close": "151.00",
                "09. change": "1.50",
                "10. change percent": "0.99%"
            }
        })))
        .mount(&mock_server)
        .await;

    // Mock TIME_SERIES_DAILY endpoint
    Mock::given(method("GET"))
        .and(query_param("function", "TIME_SERIES_DAILY"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "Meta Data": {
                "1. Information": "Daily Prices",
                "2. Symbol": "AAPL",
                "3. Last Refreshed": "2024-01-15",
                "4. Output Size": "Compact",
                "5. Time Zone": "US/Eastern"
            },
            "Time Series (Daily)": {
                "2024-01-15": {
                    "1. open": "150.00",
                    "2. high": "155.00",
                    "3. low": "149.00",
                    "4. close": "152.50",
                    "5. volume": "50000000"
                },
                "2024-01-14": {
                    "1. open": "148.00",
                    "2. high": "152.00",
                    "3. low": "147.00",
                    "4. close": "151.00",
                    "5. volume": "45000000"
                },
                "2024-01-13": {
                    "1. open": "145.00",
                    "2. high": "149.00",
                    "3. low": "144.00",
                    "4. close": "148.00",
                    "5. volume": "42000000"
                }
            }
        })))
        .mount(&mock_server)
        .await;

    mock_server
}

/// Test user credentials
pub struct TestUser {
    pub email: String,
    pub password: String,
}

impl TestUser {
    pub fn new() -> Self {
        Self {
            email: format!("test_{}@example.com", uuid::Uuid::new_v4()),
            password: "TestPassword123!".to_string(),
        }
    }
}

impl Default for TestUser {
    fn default() -> Self {
        Self::new()
    }
}

/// Test watchlist data
pub struct TestWatchlist {
    pub name: String,
}

impl TestWatchlist {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Default for TestWatchlist {
    fn default() -> Self {
        Self::new("Test Watchlist")
    }
}

/// Cleanup function for test databases
#[allow(dead_code)]
pub async fn cleanup_test_database(db_name: &str) {
    // This would be implemented to clean up test data
    // For now, we rely on unique user emails and test isolation
    tracing::info!("Cleaning up test database: {}", db_name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_config() {
        let config = create_test_config();
        assert_eq!(config.server.environment, "test");
        assert_eq!(config.jwt.secret, TEST_JWT_SECRET);
    }

    #[test]
    fn test_generate_test_token() {
        let user_id = ObjectId::new();
        let email = "test@example.com";

        let token = generate_test_token(&user_id, email);
        assert!(!token.is_empty());

        // Verify token can be validated
        let jwt = create_test_jwt_manager();
        let claims = jwt.validate_access_token(&token).expect("Token should be valid");
        assert_eq!(claims.email, email);
    }

    #[test]
    fn test_test_user() {
        let user1 = TestUser::new();
        let user2 = TestUser::new();

        // Each user should have a unique email
        assert_ne!(user1.email, user2.email);
    }

    #[tokio::test]
    async fn test_mock_alpha_vantage() {
        let mock_server = create_mock_alpha_vantage().await;
        assert!(!mock_server.uri().is_empty());
    }
}
