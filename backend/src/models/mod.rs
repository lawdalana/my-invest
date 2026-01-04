//! Data models for My-Invest Backend
//!
//! This module contains all domain models, DTOs (Data Transfer Objects),
//! and database entity definitions.

pub mod asset;
pub mod user;
pub mod watchlist;

// Re-export commonly used types
pub use asset::{Asset, AssetType, HistoricalDataPoint, PriceHistory, SearchResult};
pub use user::{AuthResponse, LoginRequest, RegisterRequest, User, UserPreferences};
pub use watchlist::{
    AddAssetRequest, CreateWatchlistRequest, UpdateWatchlistRequest, Watchlist, WatchlistAsset,
    WatchlistResponse,
};
