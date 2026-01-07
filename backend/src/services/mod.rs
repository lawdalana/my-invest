//! Services module - contains business logic and external service integrations
//!
//! This module includes:
//! - Authentication service (auth_service.rs)
//! - Asset service with caching (asset_service.rs)
//! - Watchlist service (watchlist_service.rs)
//! - Alert service (alert_service.rs)
//! - Alert processor (alert_processor.rs)
//! - Notification sender (notification_sender.rs)
//! - Alpha Vantage API client (alpha_vantage_client.rs)

pub mod alert_processor;
pub mod alert_service;
pub mod alpha_vantage_client;
pub mod asset_service;
pub mod auth_service;
pub mod notification_sender;
pub mod watchlist_service;

// Re-export commonly used types
pub use alert_processor::{AlertProcessor, AlertProcessorConfig};
pub use alert_service::AlertService;
pub use alpha_vantage_client::AlphaVantageClient;
pub use asset_service::AssetService;
pub use auth_service::AuthService;
pub use notification_sender::{
    LoggingNotificationSender, NoOpNotificationSender, NotificationSender,
    QueuedNotificationSender,
};
pub use watchlist_service::WatchlistService;
