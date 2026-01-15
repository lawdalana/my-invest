//! My-Invest Backend API Server
//!
//! A RESTful API server for the My-Invest Dashboard application.
//! Built with Axum, provides endpoints for authentication, assets,
//! watchlists, and price alerts with real-time monitoring.

use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

mod api;
mod config;
mod middleware;
mod models;
mod services;
mod utils;

use crate::api::routes::{build_router, AppState};
use crate::config::{database::MongoDb, Config};
use crate::services::{
    alert_processor::{AlertProcessor, AlertProcessorConfig},
    alert_service::AlertService,
    alpha_vantage_client::AlphaVantageClient,
    asset_service::AssetService,
    auth_service::AuthService,
    notification_sender::QueuedNotificationSender,
    watchlist_service::WatchlistService,
    ws_service::{WsConfig, WsService},
};
use crate::utils::jwt::JwtManager;
use crate::utils::logging;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration from environment
    let config = Config::from_env();

    // Initialize logging with environment
    logging::init("development");

    info!("Starting My-Invest Backend API Server");
    info!("Configuration loaded successfully");

    // Connect to MongoDB
    info!("Connecting to MongoDB...");
    let mongo_db = MongoDb::connect(&config).await?;
    info!("Connected to MongoDB");

    // Create indexes
    info!("Creating database indexes...");
    mongo_db.create_indexes().await?;
    info!("Database indexes created");

    // Connect to Redis
    info!("Connecting to Redis...");
    let redis_db = crate::config::database::RedisDb::connect(&config).await?;
    info!("Connected to Redis");

    // Create JWT manager
    let jwt_manager = JwtManager::new(&config.jwt);

    // Create Alpha Vantage client
    let alpha_vantage_client = AlphaVantageClient::new(&config.alpha_vantage);

    // Create services
    let auth_service = AuthService::new(
        &mongo_db,
        jwt_manager.clone(),
        config.app.min_password_length,
    );

    let asset_service = AssetService::new(
        alpha_vantage_client,
        redis_db.clone(),
        &config.redis,
    );

    let watchlist_service = WatchlistService::new(
        &mongo_db,
        asset_service.clone(),
        &config.app,
    );

    let alert_service = AlertService::new(
        &mongo_db,
        asset_service.clone(),
        config.app.max_alerts_per_user,
    );

    // Create WebSocket service
    let ws_config = WsConfig::from_env();
    info!(
        "Creating WebSocket service with update interval: {}s",
        ws_config.update_interval_seconds
    );
    let ws_service = Arc::new(WsService::new(
        asset_service.clone(),
        ws_config.update_interval_seconds,
    ));

    // Create application state
    let app_state = AppState {
        auth_service,
        asset_service: asset_service.clone(),
        watchlist_service,
        alert_service: alert_service.clone(),
        jwt_manager: Arc::new(jwt_manager),
        ws_service: Some(ws_service.clone()),
    };

    // Build the router
    let app = build_router(app_state, &config);

    // Start the alert processor in a background task
    let alert_processor_config = AlertProcessorConfig {
        check_interval_seconds: config.app.alert_check_interval_seconds,
        recurring_cooldown_seconds: config.app.alert_recurring_cooldown_seconds,
        batch_size: 1000,
    };

    let notification_sender = QueuedNotificationSender::new(redis_db);

    let alert_processor = AlertProcessor::new(
        alert_service,
        asset_service,
        Arc::new(notification_sender),
        alert_processor_config,
    );

    // Spawn the alert processor background task
    tokio::spawn(async move {
        info!("Starting alert processor background task");
        alert_processor.run().await;
        info!("Alert processor stopped");
    });

    // Spawn the WebSocket price updater background task
    tokio::spawn(async move {
        info!("Starting WebSocket price updater background task");
        ws_service.run_price_updater().await;
        info!("WebSocket price updater stopped");
    });

    // Parse server address
    let addr: SocketAddr = format!("{}:{}", config.app.host, config.app.port)
        .parse()
        .expect("Invalid server address");

    info!("Server listening on http://{}", addr);

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(addr).await?;

    // Serve the application with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Server shutdown complete");
    Ok(())
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    info!("Received shutdown signal, starting graceful shutdown...");
}
