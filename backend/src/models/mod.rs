//! Models module - contains all data structures and domain models
//!
//! This module includes:
//! - User models (user.rs)
//! - Asset models (asset.rs)
//! - Watchlist models (watchlist.rs)
//! - Alert models (alert.rs)

pub mod alert;
pub mod asset;
pub mod user;
pub mod watchlist;

// Re-export commonly used types
pub use alert::{
    Alert, AlertCondition, AlertListResponse, AlertNotification, AlertResponse, AlertStatus,
    AlertType, CreateAlertRequest, UpdateAlertRequest,
};
pub use asset::{Asset, AssetType, HistoricalDataPoint, PriceHistory, SearchResult, Timeframe};
pub use user::{MessageResponse, User, UserResponse};
pub use watchlist::{Watchlist, WatchlistAsset, WatchlistResponse};
