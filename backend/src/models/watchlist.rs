//! Watchlist model and related DTOs
//!
//! This module defines watchlist-related structures for managing
//! user watchlists and their assets.

use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::asset::AssetType;

/// Watchlist entity stored in MongoDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watchlist {
    /// Unique identifier
    #[serde(rename = "_id")]
    pub id: ObjectId,

    /// Owner's user ID
    pub user_id: ObjectId,

    /// Watchlist name
    pub name: String,

    /// Assets in this watchlist
    #[serde(default)]
    pub assets: Vec<WatchlistAsset>,

    /// Watchlist creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Watchlist {
    /// Create a new Watchlist
    pub fn new(user_id: ObjectId, name: String) -> Self {
        let now = Utc::now();
        Self {
            id: ObjectId::new(),
            user_id,
            name,
            assets: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if the watchlist contains a specific symbol
    pub fn contains_symbol(&self, symbol: &str) -> bool {
        self.assets
            .iter()
            .any(|a| a.symbol.eq_ignore_ascii_case(symbol))
    }

    /// Add an asset to the watchlist
    pub fn add_asset(&mut self, asset: WatchlistAsset) {
        if !self.contains_symbol(&asset.symbol) {
            self.assets.push(asset);
            self.updated_at = Utc::now();
        }
    }

    /// Remove an asset from the watchlist by symbol
    pub fn remove_asset(&mut self, symbol: &str) -> bool {
        let original_len = self.assets.len();
        self.assets
            .retain(|a| !a.symbol.eq_ignore_ascii_case(symbol));
        let removed = self.assets.len() < original_len;
        if removed {
            self.updated_at = Utc::now();
        }
        removed
    }

    /// Get the number of assets in the watchlist
    pub fn asset_count(&self) -> usize {
        self.assets.len()
    }
}

/// Asset entry in a watchlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistAsset {
    /// Stock ticker symbol
    pub symbol: String,

    /// Asset type (always "stock" in Phase 1)
    pub asset_type: AssetType,

    /// Timestamp when asset was added to watchlist
    pub added_at: DateTime<Utc>,
}

impl WatchlistAsset {
    /// Create a new WatchlistAsset
    pub fn new(symbol: String) -> Self {
        Self {
            symbol: symbol.to_uppercase(),
            asset_type: AssetType::Stock,
            added_at: Utc::now(),
        }
    }
}

/// Request DTO for creating a watchlist
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateWatchlistRequest {
    /// Watchlist name
    #[validate(length(
        min = 1,
        max = 50,
        message = "Watchlist name must be between 1 and 50 characters"
    ))]
    pub name: String,
}

/// Request DTO for updating a watchlist
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateWatchlistRequest {
    /// New watchlist name
    #[validate(length(
        min = 1,
        max = 50,
        message = "Watchlist name must be between 1 and 50 characters"
    ))]
    pub name: String,
}

/// Request DTO for adding an asset to a watchlist
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct AddAssetRequest {
    /// Stock ticker symbol to add
    #[validate(length(
        min = 1,
        max = 10,
        message = "Symbol must be between 1 and 10 characters"
    ))]
    pub symbol: String,
}

/// Response DTO for a watchlist
#[derive(Debug, Clone, Serialize)]
pub struct WatchlistResponse {
    /// Watchlist ID
    pub id: String,

    /// Watchlist name
    pub name: String,

    /// Assets in the watchlist
    pub assets: Vec<WatchlistAssetResponse>,

    /// Number of assets
    pub asset_count: usize,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Response DTO for a watchlist asset
#[derive(Debug, Clone, Serialize)]
pub struct WatchlistAssetResponse {
    /// Stock symbol
    pub symbol: String,

    /// Asset type
    pub asset_type: String,

    /// When the asset was added
    pub added_at: DateTime<Utc>,
}

impl From<Watchlist> for WatchlistResponse {
    fn from(watchlist: Watchlist) -> Self {
        let assets: Vec<WatchlistAssetResponse> = watchlist
            .assets
            .iter()
            .map(|a| WatchlistAssetResponse {
                symbol: a.symbol.clone(),
                asset_type: a.asset_type.to_string(),
                added_at: a.added_at,
            })
            .collect();

        Self {
            id: watchlist.id.to_hex(),
            name: watchlist.name,
            asset_count: assets.len(),
            assets,
            created_at: watchlist.created_at,
            updated_at: watchlist.updated_at,
        }
    }
}

/// Response DTO for a list of watchlists (summary view)
#[derive(Debug, Clone, Serialize)]
pub struct WatchlistSummary {
    /// Watchlist ID
    pub id: String,

    /// Watchlist name
    pub name: String,

    /// Number of assets
    pub asset_count: usize,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl From<Watchlist> for WatchlistSummary {
    fn from(watchlist: Watchlist) -> Self {
        Self {
            id: watchlist.id.to_hex(),
            name: watchlist.name,
            asset_count: watchlist.assets.len(),
            created_at: watchlist.created_at,
            updated_at: watchlist.updated_at,
        }
    }
}

/// Response DTO for list of watchlists
#[derive(Debug, Clone, Serialize)]
pub struct WatchlistListResponse {
    /// List of watchlists
    pub watchlists: Vec<WatchlistSummary>,

    /// Total count
    pub total: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watchlist_new() {
        let user_id = ObjectId::new();
        let watchlist = Watchlist::new(user_id, "My Portfolio".to_string());

        assert_eq!(watchlist.name, "My Portfolio");
        assert_eq!(watchlist.user_id, user_id);
        assert!(watchlist.assets.is_empty());
    }

    #[test]
    fn test_watchlist_add_asset() {
        let mut watchlist = Watchlist::new(ObjectId::new(), "Test".to_string());
        let asset = WatchlistAsset::new("AAPL".to_string());

        watchlist.add_asset(asset);

        assert_eq!(watchlist.asset_count(), 1);
        assert!(watchlist.contains_symbol("AAPL"));
        assert!(watchlist.contains_symbol("aapl")); // Case insensitive
    }

    #[test]
    fn test_watchlist_no_duplicate_assets() {
        let mut watchlist = Watchlist::new(ObjectId::new(), "Test".to_string());

        watchlist.add_asset(WatchlistAsset::new("AAPL".to_string()));
        watchlist.add_asset(WatchlistAsset::new("aapl".to_string())); // Duplicate

        assert_eq!(watchlist.asset_count(), 1);
    }

    #[test]
    fn test_watchlist_remove_asset() {
        let mut watchlist = Watchlist::new(ObjectId::new(), "Test".to_string());
        watchlist.add_asset(WatchlistAsset::new("AAPL".to_string()));
        watchlist.add_asset(WatchlistAsset::new("GOOGL".to_string()));

        let removed = watchlist.remove_asset("AAPL");

        assert!(removed);
        assert_eq!(watchlist.asset_count(), 1);
        assert!(!watchlist.contains_symbol("AAPL"));
        assert!(watchlist.contains_symbol("GOOGL"));
    }

    #[test]
    fn test_watchlist_remove_nonexistent_asset() {
        let mut watchlist = Watchlist::new(ObjectId::new(), "Test".to_string());
        watchlist.add_asset(WatchlistAsset::new("AAPL".to_string()));

        let removed = watchlist.remove_asset("GOOGL");

        assert!(!removed);
        assert_eq!(watchlist.asset_count(), 1);
    }

    #[test]
    fn test_watchlist_asset_uppercase() {
        let asset = WatchlistAsset::new("aapl".to_string());
        assert_eq!(asset.symbol, "AAPL");
    }

    #[test]
    fn test_watchlist_response_from() {
        let mut watchlist = Watchlist::new(ObjectId::new(), "Test Portfolio".to_string());
        watchlist.add_asset(WatchlistAsset::new("AAPL".to_string()));
        watchlist.add_asset(WatchlistAsset::new("GOOGL".to_string()));

        let response: WatchlistResponse = watchlist.into();

        assert_eq!(response.name, "Test Portfolio");
        assert_eq!(response.asset_count, 2);
        assert_eq!(response.assets.len(), 2);
    }

    #[test]
    fn test_watchlist_summary_from() {
        let mut watchlist = Watchlist::new(ObjectId::new(), "Tech Stocks".to_string());
        watchlist.add_asset(WatchlistAsset::new("AAPL".to_string()));

        let summary: WatchlistSummary = watchlist.into();

        assert_eq!(summary.name, "Tech Stocks");
        assert_eq!(summary.asset_count, 1);
    }
}
