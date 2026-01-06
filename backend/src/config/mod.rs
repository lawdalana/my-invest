//! Configuration module - handles application configuration loading
//!
//! This module provides configuration structures for all application components:
//! - Database connections (MongoDB, Redis)
//! - JWT authentication
//! - CORS settings
//! - Rate limiting
//! - External APIs (Alpha Vantage)
//! - Application settings

pub mod database;

use serde::Deserialize;

pub use database::{MongoDb, RedisDb};

/// Main application configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// MongoDB configuration
    pub mongodb: MongoDbConfig,

    /// Redis configuration
    pub redis: RedisConfig,

    /// JWT configuration
    pub jwt: JwtConfig,

    /// CORS configuration
    pub cors: CorsConfig,

    /// Rate limiting configuration
    pub rate_limit: RateLimitConfig,

    /// Alpha Vantage API configuration
    pub alpha_vantage: AlphaVantageConfig,

    /// Application-specific settings
    pub app: AppConfig,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            mongodb: MongoDbConfig::from_env(),
            redis: RedisConfig::from_env(),
            jwt: JwtConfig::from_env(),
            cors: CorsConfig::from_env(),
            rate_limit: RateLimitConfig::from_env(),
            alpha_vantage: AlphaVantageConfig::from_env(),
            app: AppConfig::from_env(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mongodb: MongoDbConfig::default(),
            redis: RedisConfig::default(),
            jwt: JwtConfig::default(),
            cors: CorsConfig::default(),
            rate_limit: RateLimitConfig::default(),
            alpha_vantage: AlphaVantageConfig::default(),
            app: AppConfig::default(),
        }
    }
}

/// MongoDB configuration
#[derive(Debug, Clone, Deserialize)]
pub struct MongoDbConfig {
    /// MongoDB connection URI
    pub uri: String,

    /// Database name
    pub database: String,

    /// Minimum connection pool size
    pub min_pool_size: u32,

    /// Maximum connection pool size
    pub max_pool_size: u32,
}

impl MongoDbConfig {
    pub fn from_env() -> Self {
        Self {
            uri: std::env::var("MONGODB_URI")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            database: std::env::var("MONGODB_DATABASE")
                .unwrap_or_else(|_| "myinvest".to_string()),
            min_pool_size: std::env::var("MONGODB_MIN_POOL_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            max_pool_size: std::env::var("MONGODB_MAX_POOL_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(20),
        }
    }
}

impl Default for MongoDbConfig {
    fn default() -> Self {
        Self {
            uri: "mongodb://localhost:27017".to_string(),
            database: "myinvest".to_string(),
            min_pool_size: 5,
            max_pool_size: 20,
        }
    }
}

/// Redis configuration
#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    /// Redis connection URL
    pub url: String,

    /// Default cache TTL in seconds
    pub cache_ttl: u64,

    /// Historical data cache TTL in seconds
    pub history_cache_ttl: u64,
}

impl RedisConfig {
    pub fn from_env() -> Self {
        Self {
            url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            cache_ttl: std::env::var("REDIS_CACHE_TTL")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(300), // 5 minutes
            history_cache_ttl: std::env::var("REDIS_HISTORY_CACHE_TTL")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3600), // 1 hour
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            cache_ttl: 300,
            history_cache_ttl: 3600,
        }
    }
}

/// JWT configuration
#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    /// Secret key for signing JWTs
    pub secret: String,

    /// Access token expiration in seconds
    pub access_expiration: i64,

    /// Refresh token expiration in seconds
    pub refresh_expiration: i64,

    /// Token issuer
    pub issuer: String,
}

impl JwtConfig {
    pub fn from_env() -> Self {
        Self {
            secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "default-secret-please-change-in-production".to_string()),
            access_expiration: std::env::var("JWT_ACCESS_EXPIRATION")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(900), // 15 minutes
            refresh_expiration: std::env::var("JWT_REFRESH_EXPIRATION")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2592000), // 30 days
            issuer: std::env::var("JWT_ISSUER")
                .unwrap_or_else(|_| "my-invest-backend".to_string()),
        }
    }
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "default-secret-please-change-in-production".to_string(),
            access_expiration: 900,
            refresh_expiration: 2592000,
            issuer: "my-invest-backend".to_string(),
        }
    }
}

/// CORS configuration
#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    /// Allowed origins for CORS
    pub allowed_origins: Vec<String>,
}

impl CorsConfig {
    pub fn from_env() -> Self {
        let origins = std::env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string());

        Self {
            allowed_origins: origins
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        }
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec![
                "http://localhost:3000".to_string(),
                "http://localhost:5173".to_string(),
            ],
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub requests: u32,

    /// Time window in seconds
    pub window_seconds: u64,
}

impl RateLimitConfig {
    pub fn from_env() -> Self {
        Self {
            requests: std::env::var("RATE_LIMIT_REQUESTS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            window_seconds: std::env::var("RATE_LIMIT_WINDOW_SECONDS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(60),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests: 100,
            window_seconds: 60,
        }
    }
}

/// Alpha Vantage API configuration
#[derive(Debug, Clone, Deserialize)]
pub struct AlphaVantageConfig {
    /// API key for Alpha Vantage
    pub api_key: String,

    /// Base URL for the API
    pub base_url: String,

    /// Request timeout in seconds
    pub timeout_seconds: u64,
}

impl AlphaVantageConfig {
    pub fn from_env() -> Self {
        Self {
            api_key: std::env::var("ALPHA_VANTAGE_API_KEY")
                .unwrap_or_else(|_| "demo".to_string()),
            base_url: std::env::var("ALPHA_VANTAGE_BASE_URL")
                .unwrap_or_else(|_| "https://www.alphavantage.co/query".to_string()),
            timeout_seconds: std::env::var("ALPHA_VANTAGE_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
        }
    }
}

impl Default for AlphaVantageConfig {
    fn default() -> Self {
        Self {
            api_key: "demo".to_string(),
            base_url: "https://www.alphavantage.co/query".to_string(),
            timeout_seconds: 30,
        }
    }
}

/// Application-specific configuration
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Server host
    pub host: String,

    /// Server port
    pub port: u16,

    /// Maximum watchlists per user
    pub max_watchlists_per_user: u32,

    /// Maximum assets per watchlist
    pub max_assets_per_watchlist: u32,

    /// Maximum alerts per user
    pub max_alerts_per_user: u32,

    /// Alert check interval in seconds
    pub alert_check_interval_seconds: u64,

    /// Cooldown for recurring alerts in seconds
    pub alert_recurring_cooldown_seconds: i64,

    /// Minimum password length
    pub min_password_length: usize,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8080),
            max_watchlists_per_user: std::env::var("MAX_WATCHLISTS_PER_USER")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(20),
            max_assets_per_watchlist: std::env::var("MAX_ASSETS_PER_WATCHLIST")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            max_alerts_per_user: std::env::var("ALERT_MAX_PER_USER")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(50),
            alert_check_interval_seconds: std::env::var("ALERT_CHECK_INTERVAL_SECONDS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            alert_recurring_cooldown_seconds: std::env::var("ALERT_RECURRING_COOLDOWN_SECONDS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(300),
            min_password_length: std::env::var("MIN_PASSWORD_LENGTH")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            max_watchlists_per_user: 20,
            max_assets_per_watchlist: 100,
            max_alerts_per_user: 50,
            alert_check_interval_seconds: 30,
            alert_recurring_cooldown_seconds: 300,
            min_password_length: 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.app.port, 8080);
        assert_eq!(config.app.max_alerts_per_user, 50);
        assert_eq!(config.mongodb.database, "myinvest");
    }

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.max_alerts_per_user, 50);
        assert_eq!(config.alert_check_interval_seconds, 30);
        assert_eq!(config.alert_recurring_cooldown_seconds, 300);
    }
}
