//! My-Invest Backend API Server
//!
//! This is the main entry point for the My-Invest backend server.
//! It initializes all components and starts the HTTP server.

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpListener;
use tracing::{error, info};

use my_invest_backend::{
    api::routes::{build_router, AppState},
    config::{
        database::{MongoDb, RedisDb},
        Config,
    },
    services::{AlphaVantageClient, AssetService, AuthService, WatchlistService},
    utils::{jwt::JwtManager, logging},
};

#[tokio::main]
async fn main() {
    // Load configuration from environment
    let config = Config::from_env();

    // Initialize logging
    logging::init(&config.server.environment);

    info!(
        version = env!("CARGO_PKG_VERSION"),
        environment = config.server.environment,
        "Starting My-Invest Backend"
    );

    // Initialize databases
    info!("Connecting to databases...");

    let mongodb = match MongoDb::connect(&config).await {
        Ok(db) => {
            info!("MongoDB connected successfully");
            db
        }
        Err(e) => {
            error!(error = %e, "Failed to connect to MongoDB");
            std::process::exit(1);
        }
    };

    // Create database indexes
    if let Err(e) = mongodb.create_indexes().await {
        error!(error = %e, "Failed to create MongoDB indexes");
        std::process::exit(1);
    }

    let redis = match RedisDb::connect(&config).await {
        Ok(db) => {
            info!("Redis connected successfully");
            db
        }
        Err(e) => {
            error!(error = %e, "Failed to connect to Redis");
            std::process::exit(1);
        }
    };

    // Initialize services
    info!("Initializing services...");

    let jwt_manager = Arc::new(JwtManager::new(&config.jwt));

    let auth_service = AuthService::new(
        &mongodb,
        (*jwt_manager).clone(),
        config.app.min_password_length,
    );

    let alpha_vantage_client = AlphaVantageClient::new(&config.alpha_vantage);

    let asset_service = AssetService::new(
        alpha_vantage_client,
        redis.clone(),
        &config.redis,
    );

    let watchlist_service = WatchlistService::new(
        &mongodb,
        asset_service.clone(),
        &config.app,
    );

    // Create application state
    let app_state = AppState {
        auth_service,
        asset_service,
        watchlist_service,
        jwt_manager,
    };

    // Build router
    let app = build_router(app_state, &config);

    // Create server address
    let addr: SocketAddr = config
        .server_address()
        .parse()
        .expect("Invalid server address");

    info!(
        host = config.server.host,
        port = config.server.port,
        "Server starting"
    );

    // Start server
    let listener = TcpListener::bind(addr).await.expect("Failed to bind to address");

    info!(
        address = %addr,
        "Server listening"
    );

    // Run the server
    if let Err(e) = axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
    {
        error!(error = %e, "Server error");
        std::process::exit(1);
    }
}
