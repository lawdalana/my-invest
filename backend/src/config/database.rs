//! Database connection management for MongoDB and Redis
//!
//! This module provides functions to establish and manage connections
//! to MongoDB and Redis databases.

use mongodb::{options::ClientOptions, Client, Database};
use redis::aio::ConnectionManager;
use tracing::{info, instrument};

use super::Config;
use crate::utils::error::{AppError, Result};

/// MongoDB client wrapper
#[derive(Clone)]
pub struct MongoDb {
    client: Client,
    database: Database,
}

impl MongoDb {
    /// Create a new MongoDB connection
    #[instrument(skip(config), name = "mongodb_connect")]
    pub async fn connect(config: &Config) -> Result<Self> {
        info!("Connecting to MongoDB at {}", config.mongodb.uri);

        let mut client_options = ClientOptions::parse(&config.mongodb.uri)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to parse MongoDB URI: {}", e)))?;

        // Set connection pool options
        client_options.min_pool_size = Some(config.mongodb.min_pool_size);
        client_options.max_pool_size = Some(config.mongodb.max_pool_size);

        // Set application name for MongoDB logs
        client_options.app_name = Some("my-invest-backend".to_string());

        let client = Client::with_options(client_options)
            .map_err(|e| AppError::DatabaseError(format!("Failed to create MongoDB client: {}", e)))?;

        // Test the connection by pinging the server
        client
            .database("admin")
            .run_command(mongodb::bson::doc! { "ping": 1 }, None)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to ping MongoDB: {}", e)))?;

        let database = client.database(&config.mongodb.database);

        info!(
            "Successfully connected to MongoDB database: {}",
            config.mongodb.database
        );

        Ok(Self { client, database })
    }

    /// Get a reference to the database
    pub fn database(&self) -> &Database {
        &self.database
    }

    /// Get a reference to the client
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Create database indexes for collections
    #[instrument(skip(self), name = "create_indexes")]
    pub async fn create_indexes(&self) -> Result<()> {
        use mongodb::{bson::doc, options::IndexOptions, IndexModel};

        info!("Creating MongoDB indexes...");

        // Users collection indexes
        let users_collection = self.database.collection::<mongodb::bson::Document>("users");

        // Unique email index
        let email_index = IndexModel::builder()
            .keys(doc! { "email": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        users_collection
            .create_index(email_index, None)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to create email index: {}", e)))?;

        // Watchlists collection indexes
        let watchlists_collection = self.database.collection::<mongodb::bson::Document>("watchlists");

        // User ID index for efficient filtering
        let user_id_index = IndexModel::builder()
            .keys(doc! { "user_id": 1 })
            .build();

        watchlists_collection
            .create_index(user_id_index, None)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to create user_id index: {}", e))
            })?;

        // Compound index for user + name uniqueness
        let user_name_index = IndexModel::builder()
            .keys(doc! { "user_id": 1, "name": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        watchlists_collection
            .create_index(user_name_index, None)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to create user_name index: {}", e))
            })?;

        // Refresh tokens collection indexes
        let tokens_collection = self.database.collection::<mongodb::bson::Document>("refresh_tokens");

        // Token index for lookup
        let token_index = IndexModel::builder()
            .keys(doc! { "token": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        tokens_collection
            .create_index(token_index, None)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to create token index: {}", e)))?;

        // User ID index for cleanup
        let token_user_index = IndexModel::builder()
            .keys(doc! { "user_id": 1 })
            .build();

        tokens_collection
            .create_index(token_user_index, None)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to create token user_id index: {}", e))
            })?;

        // TTL index for automatic token expiration
        let token_ttl_index = IndexModel::builder()
            .keys(doc! { "expires_at": 1 })
            .options(IndexOptions::builder().expire_after(std::time::Duration::from_secs(0)).build())
            .build();

        tokens_collection
            .create_index(token_ttl_index, None)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to create token TTL index: {}", e))
            })?;

        // Alerts collection indexes
        let alerts_collection = self.database.collection::<mongodb::bson::Document>("alerts");

        // User ID index for efficient filtering by user
        let alert_user_id_index = IndexModel::builder()
            .keys(doc! { "user_id": 1 })
            .build();

        alerts_collection
            .create_index(alert_user_id_index, None)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to create alert user_id index: {}", e))
            })?;

        // Status index for finding active alerts
        let alert_status_index = IndexModel::builder()
            .keys(doc! { "status": 1 })
            .build();

        alerts_collection
            .create_index(alert_status_index, None)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to create alert status index: {}", e))
            })?;

        // Compound index for symbol + status (for background processing)
        let alert_symbol_status_index = IndexModel::builder()
            .keys(doc! { "symbol": 1, "status": 1 })
            .build();

        alerts_collection
            .create_index(alert_symbol_status_index, None)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to create alert symbol_status index: {}", e))
            })?;

        // Unique partial index to prevent duplicate alerts
        // (same user, symbol, condition, and target_price for active alerts)
        let alert_unique_index = IndexModel::builder()
            .keys(doc! { "user_id": 1, "symbol": 1, "condition": 1, "target_price": 1 })
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .partial_filter_expression(doc! { "status": "active" })
                    .build(),
            )
            .build();

        alerts_collection
            .create_index(alert_unique_index, None)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to create alert unique index: {}", e))
            })?;

        // Password reset tokens collection indexes
        let reset_tokens_collection = self.database.collection::<mongodb::bson::Document>("password_reset_tokens");

        // Selector index for fast lookups (split-token pattern)
        let reset_selector_index = IndexModel::builder()
            .keys(doc! { "selector": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        reset_tokens_collection
            .create_index(reset_selector_index, None)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to create reset token selector index: {}", e))
            })?;

        // User ID index for invalidating existing tokens
        let reset_user_index = IndexModel::builder()
            .keys(doc! { "user_id": 1 })
            .build();

        reset_tokens_collection
            .create_index(reset_user_index, None)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to create reset token user_id index: {}", e))
            })?;

        // TTL index for automatic token expiration (security critical)
        let reset_ttl_index = IndexModel::builder()
            .keys(doc! { "expires_at": 1 })
            .options(IndexOptions::builder().expire_after(std::time::Duration::from_secs(0)).build())
            .build();

        reset_tokens_collection
            .create_index(reset_ttl_index, None)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to create reset token TTL index: {}", e))
            })?;

        info!("MongoDB indexes created successfully");

        Ok(())
    }
}

/// Redis connection manager wrapper
#[derive(Clone)]
pub struct RedisDb {
    connection: ConnectionManager,
}

impl RedisDb {
    /// Create a new Redis connection
    #[instrument(skip(config), name = "redis_connect")]
    pub async fn connect(config: &Config) -> Result<Self> {
        info!("Connecting to Redis at {}", config.redis.url);

        let client = redis::Client::open(config.redis.url.as_str())
            .map_err(|e| AppError::CacheError(format!("Failed to create Redis client: {}", e)))?;

        let connection = ConnectionManager::new(client)
            .await
            .map_err(|e| AppError::CacheError(format!("Failed to connect to Redis: {}", e)))?;

        // Test the connection with a ping
        let mut conn = connection.clone();
        redis::cmd("PING")
            .query_async::<_, String>(&mut conn)
            .await
            .map_err(|e| AppError::CacheError(format!("Failed to ping Redis: {}", e)))?;

        info!("Successfully connected to Redis");

        Ok(Self { connection })
    }

    /// Get a clone of the connection manager
    pub fn connection(&self) -> ConnectionManager {
        self.connection.clone()
    }

    /// Set a key-value pair with TTL
    pub async fn set_with_ttl(&self, key: &str, value: &str, ttl_seconds: u64) -> Result<()> {
        let mut conn = self.connection.clone();
        redis::cmd("SETEX")
            .arg(key)
            .arg(ttl_seconds)
            .arg(value)
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| AppError::CacheError(format!("Failed to set cache: {}", e)))?;
        Ok(())
    }

    /// Get a value by key
    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.connection.clone();
        let result: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::CacheError(format!("Failed to get cache: {}", e)))?;
        Ok(result)
    }

    /// Delete a key
    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.connection.clone();
        redis::cmd("DEL")
            .arg(key)
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| AppError::CacheError(format!("Failed to delete cache: {}", e)))?;
        Ok(())
    }

    /// Check if a key exists
    pub async fn exists(&self, key: &str) -> Result<bool> {
        let mut conn = self.connection.clone();
        let result: bool = redis::cmd("EXISTS")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::CacheError(format!("Failed to check cache: {}", e)))?;
        Ok(result)
    }

    /// Increment a counter
    pub async fn incr(&self, key: &str) -> Result<i64> {
        let mut conn = self.connection.clone();
        let result: i64 = redis::cmd("INCR")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::CacheError(format!("Failed to increment counter: {}", e)))?;
        Ok(result)
    }

    /// Set expiration on a key
    pub async fn expire(&self, key: &str, seconds: u64) -> Result<()> {
        let mut conn = self.connection.clone();
        redis::cmd("EXPIRE")
            .arg(key)
            .arg(seconds)
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| AppError::CacheError(format!("Failed to set expiration: {}", e)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would go here, but require actual database connections
    // We'll use testcontainers in the integration test suite
}
