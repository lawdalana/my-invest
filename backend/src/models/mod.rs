//! Models module - contains all data structures and domain models
//!
//! This module includes:
//! - User models (user.rs)
//! - Asset models (asset.rs)
//! - Watchlist models (watchlist.rs)
//! - Alert models (alert.rs)
//! - WebSocket models (websocket.rs)

pub mod alert;
pub mod asset;
pub mod user;
pub mod watchlist;
pub mod websocket;

// Re-export commonly used types
pub use alert::{
    Alert, AlertCondition, AlertListResponse, AlertNotification, AlertResponse, AlertStatus,
    AlertType, CreateAlertRequest, UpdateAlertRequest,
};
pub use asset::{Asset, AssetType, HistoricalDataPoint, PriceHistory, SearchResult, Timeframe};
pub use user::{MessageResponse, PasswordResetToken, User, UserResponse};
pub use watchlist::{Watchlist, WatchlistAsset, WatchlistResponse};
pub use websocket::{
    PriceUpdate, WsClientMessage, WsError, WsErrorCode, WsServerMessage,
    MAX_SUBSCRIPTIONS_PER_CONNECTION, MAX_SYMBOL_LENGTH,
};
