//! Configuration module for My-Invest Backend
//!
//! This module handles loading and managing application configuration
//! from environment variables.

pub mod database;

use serde::Deserialize;
use std::env;

/// Main application configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub mongodb: MongoDbConfig,
    pub redis: RedisConfig,
    pub alpha_vantage: AlphaVantageConfig,
    pub rate_limit: RateLimitConfig,
    pub cors: CorsConfig,
    pub app: AppConfig,
}

/// Server configuration
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub environment: String,
}

/// JWT authentication configuration
#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub access_expiration: i64,
    pub refresh_expiration: i64,
    pub issuer: String,
}

/// MongoDB configuration
#[derive(Debug, Clone, Deserialize)]
pub struct MongoDbConfig {
    pub uri: String,
    pub database: String,
    pub min_pool_size: u32,
    pub max_pool_size: u32,
}

/// Redis configuration
#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub cache_ttl: u64,
    pub history_cache_ttl: u64,
}

/// Alpha Vantage API configuration
#[derive(Debug, Clone, Deserialize)]
pub struct AlphaVantageConfig {
    pub api_key: String,
    pub base_url: String,
    pub rate_limit: u32,
    pub daily_limit: u32,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitConfig {
    pub requests: u32,
    pub window_seconds: u64,
}

/// CORS configuration
#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

/// Application-specific configuration
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub max_watchlists_per_user: u32,
    pub max_assets_per_watchlist: u32,
    pub min_password_length: usize,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// # Panics
    ///
    /// Panics if required environment variables are not set or invalid.
    pub fn from_env() -> Self {
        // Load .env file if it exists (for development)
        dotenvy::dotenv().ok();

        Config {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .expect("SERVER_PORT must be a valid port number"),
                environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET")
                    .expect("JWT_SECRET must be set"),
                access_expiration: env::var("JWT_ACCESS_EXPIRATION")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .expect("JWT_ACCESS_EXPIRATION must be a valid integer"),
                refresh_expiration: env::var("JWT_REFRESH_EXPIRATION")
                    .unwrap_or_else(|_| "2592000".to_string())
                    .parse()
                    .expect("JWT_REFRESH_EXPIRATION must be a valid integer"),
                issuer: env::var("JWT_ISSUER").unwrap_or_else(|_| "my-invest-api".to_string()),
            },
            mongodb: MongoDbConfig {
                uri: env::var("MONGODB_URI")
                    .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
                database: env::var("MONGODB_DATABASE")
                    .unwrap_or_else(|_| "my_invest".to_string()),
                min_pool_size: env::var("MONGODB_MIN_POOL_SIZE")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .expect("MONGODB_MIN_POOL_SIZE must be a valid integer"),
                max_pool_size: env::var("MONGODB_MAX_POOL_SIZE")
                    .unwrap_or_else(|_| "20".to_string())
                    .parse()
                    .expect("MONGODB_MAX_POOL_SIZE must be a valid integer"),
            },
            redis: RedisConfig {
                url: env::var("REDIS_URL")
                    .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
                cache_ttl: env::var("REDIS_CACHE_TTL")
                    .unwrap_or_else(|_| "300".to_string())
                    .parse()
                    .expect("REDIS_CACHE_TTL must be a valid integer"),
                history_cache_ttl: env::var("REDIS_HISTORY_CACHE_TTL")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .expect("REDIS_HISTORY_CACHE_TTL must be a valid integer"),
            },
            alpha_vantage: AlphaVantageConfig {
                api_key: env::var("ALPHA_VANTAGE_API_KEY")
                    .unwrap_or_else(|_| "demo".to_string()),
                base_url: env::var("ALPHA_VANTAGE_BASE_URL")
                    .unwrap_or_else(|_| "https://www.alphavantage.co/query".to_string()),
                rate_limit: env::var("ALPHA_VANTAGE_RATE_LIMIT")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .expect("ALPHA_VANTAGE_RATE_LIMIT must be a valid integer"),
                daily_limit: env::var("ALPHA_VANTAGE_DAILY_LIMIT")
                    .unwrap_or_else(|_| "500".to_string())
                    .parse()
                    .expect("ALPHA_VANTAGE_DAILY_LIMIT must be a valid integer"),
            },
            rate_limit: RateLimitConfig {
                requests: env::var("RATE_LIMIT_REQUESTS")
                    .unwrap_or_else(|_| "100".to_string())
                    .parse()
                    .expect("RATE_LIMIT_REQUESTS must be a valid integer"),
                window_seconds: env::var("RATE_LIMIT_WINDOW")
                    .unwrap_or_else(|_| "60".to_string())
                    .parse()
                    .expect("RATE_LIMIT_WINDOW must be a valid integer"),
            },
            cors: CorsConfig {
                allowed_origins: env::var("ALLOWED_ORIGINS")
                    .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            },
            app: AppConfig {
                max_watchlists_per_user: env::var("MAX_WATCHLISTS_PER_USER")
                    .unwrap_or_else(|_| "20".to_string())
                    .parse()
                    .expect("MAX_WATCHLISTS_PER_USER must be a valid integer"),
                max_assets_per_watchlist: env::var("MAX_ASSETS_PER_WATCHLIST")
                    .unwrap_or_else(|_| "50".to_string())
                    .parse()
                    .expect("MAX_ASSETS_PER_WATCHLIST must be a valid integer"),
                min_password_length: env::var("MIN_PASSWORD_LENGTH")
                    .unwrap_or_else(|_| "8".to_string())
                    .parse()
                    .expect("MIN_PASSWORD_LENGTH must be a valid integer"),
            },
        }
    }

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        self.server.environment == "development"
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        self.server.environment == "production"
    }

    /// Get the server address as a string
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Use a mutex to ensure tests don't run in parallel and pollute env vars
    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    fn cleanup_env_vars() {
        std::env::remove_var("SERVER_HOST");
        std::env::remove_var("SERVER_PORT");
        std::env::remove_var("ENVIRONMENT");
        std::env::remove_var("ALLOWED_ORIGINS");
        std::env::remove_var("JWT_SECRET");
        std::env::remove_var("JWT_ACCESS_EXPIRATION");
        std::env::remove_var("JWT_REFRESH_EXPIRATION");
        std::env::remove_var("REDIS_CACHE_TTL");
        std::env::remove_var("RATE_LIMIT_REQUESTS");
        std::env::remove_var("RATE_LIMIT_WINDOW");
    }

    #[test]
    fn test_default_values() {
        let _lock = TEST_MUTEX.lock().unwrap();
        cleanup_env_vars();

        // Set only required environment variables
        std::env::set_var("JWT_SECRET", "test-secret");

        let config = Config::from_env();

        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.jwt.access_expiration, 3600);
        assert_eq!(config.jwt.refresh_expiration, 2592000);
        assert_eq!(config.redis.cache_ttl, 300);
        assert_eq!(config.rate_limit.requests, 100);
        assert_eq!(config.rate_limit.window_seconds, 60);

        cleanup_env_vars();
    }

    #[test]
    fn test_server_address() {
        let _lock = TEST_MUTEX.lock().unwrap();
        cleanup_env_vars();

        std::env::set_var("JWT_SECRET", "test-secret");
        std::env::set_var("SERVER_HOST", "127.0.0.1");
        std::env::set_var("SERVER_PORT", "3000");

        let config = Config::from_env();

        assert_eq!(config.server_address(), "127.0.0.1:3000");

        cleanup_env_vars();
    }

    #[test]
    fn test_is_development() {
        let _lock = TEST_MUTEX.lock().unwrap();
        cleanup_env_vars();

        std::env::set_var("JWT_SECRET", "test-secret");
        std::env::set_var("ENVIRONMENT", "development");

        let config = Config::from_env();

        assert!(config.is_development());
        assert!(!config.is_production());

        cleanup_env_vars();
    }

    #[test]
    fn test_allowed_origins_parsing() {
        let _lock = TEST_MUTEX.lock().unwrap();
        cleanup_env_vars();

        std::env::set_var("JWT_SECRET", "test-secret");
        std::env::set_var("ALLOWED_ORIGINS", "http://localhost:3000, http://localhost:5173");

        let config = Config::from_env();

        assert_eq!(config.cors.allowed_origins.len(), 2);
        assert!(config.cors.allowed_origins.contains(&"http://localhost:3000".to_string()));
        assert!(config.cors.allowed_origins.contains(&"http://localhost:5173".to_string()));

        cleanup_env_vars();
    }
}
