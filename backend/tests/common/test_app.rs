//! Test application setup for integration tests
//!
//! Provides a test server with isolated MongoDB and Redis instances.

use std::sync::Arc;

use axum::Router;
use axum_test::TestServer;
use mongodb::Client as MongoClient;
use redis::aio::ConnectionManager;
use testcontainers::{clients::Cli, Container, RunnableImage};
use testcontainers_modules::{mongo::Mongo, redis::Redis};

use my_invest_backend::{
    api::routes::{build_router, AppState},
    config::{
        database::{MongoDb, RedisDb},
        AlphaVantageConfig, AppConfig, CorsConfig, JwtConfig, MongoDbConfig, RateLimitConfig,
        RedisConfig, ServerConfig, Config,
    },
    services::{AlphaVantageClient, AssetService, AuthService, WatchlistService},
    utils::jwt::JwtManager,
};

use super::{create_mock_alpha_vantage, create_test_config, TEST_JWT_SECRET};

/// Test application with isolated infrastructure
pub struct TestApp {
    pub server: TestServer,
    pub config: Config,
    pub mongo_container: Container<'static, Mongo>,
    pub redis_container: Container<'static, Redis>,
    pub mock_alpha_vantage: wiremock::MockServer,
}

impl TestApp {
    /// Create a new test application instance
    ///
    /// This sets up:
    /// - Isolated MongoDB container
    /// - Isolated Redis container
    /// - Mock Alpha Vantage server
    /// - Fully configured test server
    pub async fn new(docker: &'static Cli) -> Self {
        // Start containers
        let mongo_container = docker.run(RunnableImage::from(Mongo));
        let redis_container = docker.run(RunnableImage::from(Redis));

        // Get connection URLs
        let mongo_port = mongo_container.get_host_port_ipv4(27017);
        let redis_port = redis_container.get_host_port_ipv4(6379);

        let mongo_uri = format!("mongodb://127.0.0.1:{}", mongo_port);
        let redis_url = format!("redis://127.0.0.1:{}", redis_port);

        // Create mock Alpha Vantage server
        let mock_alpha_vantage = create_mock_alpha_vantage().await;

        // Create config with test infrastructure URLs
        let mut config = create_test_config();
        config.mongodb.uri = mongo_uri;
        config.redis.url = redis_url;
        config.alpha_vantage.base_url = mock_alpha_vantage.uri();

        // Initialize databases
        let mongodb = MongoDb::connect(&config)
            .await
            .expect("Failed to connect to test MongoDB");

        mongodb
            .create_indexes()
            .await
            .expect("Failed to create indexes");

        let redis = RedisDb::connect(&config)
            .await
            .expect("Failed to connect to test Redis");

        // Initialize services
        let jwt_manager = Arc::new(JwtManager::new(&config.jwt));

        let auth_service = AuthService::new(&mongodb, (*jwt_manager).clone(), config.app.min_password_length);

        let alpha_vantage_client = AlphaVantageClient::new(&config.alpha_vantage);

        let asset_service = AssetService::new(alpha_vantage_client, redis.clone(), &config.redis);

        let watchlist_service = WatchlistService::new(&mongodb, asset_service.clone(), &config.app);

        // Create application state
        let app_state = AppState {
            auth_service,
            asset_service,
            watchlist_service,
            jwt_manager,
        };

        // Build router
        let router = build_router(app_state, &config);

        // Create test server
        let server = TestServer::new(router).expect("Failed to create test server");

        Self {
            server,
            config,
            mongo_container,
            redis_container,
            mock_alpha_vantage,
        }
    }

    /// Get the base URL for the test server
    pub fn url(&self) -> String {
        self.server.server_address().unwrap()
    }

    /// Helper to register a test user and return their credentials + tokens
    pub async fn register_user(&self, email: &str, password: &str) -> (String, String) {
        let response = self
            .server
            .post("/api/v1/auth/register")
            .json(&serde_json::json!({
                "email": email,
                "password": password,
                "password_confirm": password
            }))
            .await;

        assert_eq!(response.status_code(), 201);

        let body: serde_json::Value = response.json();
        let access_token = body["access_token"].as_str().unwrap().to_string();
        let refresh_token = body["refresh_token"].as_str().unwrap().to_string();

        (access_token, refresh_token)
    }

    /// Helper to login a user and return their tokens
    pub async fn login_user(&self, email: &str, password: &str) -> (String, String) {
        let response = self
            .server
            .post("/api/v1/auth/login")
            .json(&serde_json::json!({
                "email": email,
                "password": password
            }))
            .await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = response.json();
        let access_token = body["access_token"].as_str().unwrap().to_string();
        let refresh_token = body["refresh_token"].as_str().unwrap().to_string();

        (access_token, refresh_token)
    }

    /// Helper to create a test watchlist
    pub async fn create_watchlist(&self, access_token: &str, name: &str) -> String {
        let response = self
            .server
            .post("/api/v1/watchlists")
            .add_header("Authorization".parse().unwrap(), format!("Bearer {}", access_token).parse().unwrap())
            .json(&serde_json::json!({
                "name": name
            }))
            .await;

        assert_eq!(response.status_code(), 201);

        let body: serde_json::Value = response.json();
        body["id"].as_str().unwrap().to_string()
    }
}
