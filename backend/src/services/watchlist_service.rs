//! Watchlist service
//!
//! This service handles all watchlist CRUD operations with proper
//! user isolation and business logic validation.

use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use mongodb::options::FindOptions;
use mongodb::Collection;
use tracing::{info, instrument, warn};

use crate::config::database::MongoDb;
use crate::config::AppConfig;
use crate::models::watchlist::{
    AddAssetRequest, CreateWatchlistRequest, UpdateWatchlistRequest, Watchlist, WatchlistAsset,
    WatchlistListResponse, WatchlistResponse, WatchlistSummary,
};
use crate::services::asset_service::AssetService;
use crate::utils::error::{AppError, Result};

/// Watchlist service for CRUD operations
#[derive(Clone)]
pub struct WatchlistService {
    watchlists: Collection<Watchlist>,
    asset_service: AssetService,
    max_watchlists_per_user: u32,
    max_assets_per_watchlist: u32,
}

impl WatchlistService {
    /// Create a new WatchlistService instance
    pub fn new(db: &MongoDb, asset_service: AssetService, app_config: &AppConfig) -> Self {
        Self {
            watchlists: db.database().collection("watchlists"),
            asset_service,
            max_watchlists_per_user: app_config.max_watchlists_per_user,
            max_assets_per_watchlist: app_config.max_assets_per_watchlist,
        }
    }

    /// Get all watchlists for a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user's ObjectId
    ///
    /// # Returns
    ///
    /// * `Ok(WatchlistListResponse)` - List of user's watchlists
    #[instrument(skip(self))]
    pub async fn get_all(&self, user_id: &ObjectId) -> Result<WatchlistListResponse> {
        info!(user_id = %user_id, "Fetching all watchlists for user");

        let filter = doc! { "user_id": user_id };
        let options = FindOptions::builder()
            .sort(doc! { "created_at": -1 })
            .build();

        let mut cursor = self.watchlists.find(filter, options).await?;

        let mut watchlists = Vec::new();
        while let Some(watchlist) = cursor.try_next().await? {
            watchlists.push(WatchlistSummary::from(watchlist));
        }

        let total = watchlists.len();

        Ok(WatchlistListResponse { watchlists, total })
    }

    /// Get a specific watchlist by ID
    ///
    /// # Arguments
    ///
    /// * `id` - Watchlist ObjectId
    /// * `user_id` - User's ObjectId (for authorization)
    ///
    /// # Returns
    ///
    /// * `Ok(WatchlistResponse)` - The watchlist
    /// * `Err(AppError::NotFound)` - If not found or not owned by user
    #[instrument(skip(self))]
    pub async fn get_by_id(
        &self,
        id: &ObjectId,
        user_id: &ObjectId,
    ) -> Result<WatchlistResponse> {
        info!(watchlist_id = %id, user_id = %user_id, "Fetching watchlist");

        let filter = doc! {
            "_id": id,
            "user_id": user_id
        };

        let watchlist = self
            .watchlists
            .find_one(filter, None)
            .await?
            .ok_or_else(|| AppError::NotFound("Watchlist not found".to_string()))?;

        Ok(WatchlistResponse::from(watchlist))
    }

    /// Create a new watchlist
    ///
    /// # Arguments
    ///
    /// * `user_id` - User's ObjectId
    /// * `request` - Create request with watchlist name
    ///
    /// # Returns
    ///
    /// * `Ok(WatchlistResponse)` - The created watchlist
    /// * `Err(AppError::LimitExceeded)` - If user has too many watchlists
    #[instrument(skip(self))]
    pub async fn create(
        &self,
        user_id: &ObjectId,
        request: CreateWatchlistRequest,
    ) -> Result<WatchlistResponse> {
        info!(user_id = %user_id, name = %request.name, "Creating new watchlist");

        // Check watchlist limit
        let count = self
            .watchlists
            .count_documents(doc! { "user_id": user_id }, None)
            .await?;

        if count >= self.max_watchlists_per_user as u64 {
            warn!(
                user_id = %user_id,
                count = count,
                limit = self.max_watchlists_per_user,
                "Watchlist limit exceeded"
            );
            return Err(AppError::LimitExceeded(format!(
                "Maximum {} watchlists allowed per user",
                self.max_watchlists_per_user
            )));
        }

        // Check for duplicate name
        let existing = self
            .watchlists
            .find_one(
                doc! {
                    "user_id": user_id,
                    "name": &request.name
                },
                None,
            )
            .await?;

        if existing.is_some() {
            return Err(AppError::AlreadyExists(
                "A watchlist with this name already exists".to_string(),
            ));
        }

        // Create watchlist
        let watchlist = Watchlist::new(*user_id, request.name);
        self.watchlists.insert_one(&watchlist, None).await?;

        info!(watchlist_id = %watchlist.id, "Watchlist created successfully");

        Ok(WatchlistResponse::from(watchlist))
    }

    /// Update a watchlist (rename)
    ///
    /// # Arguments
    ///
    /// * `id` - Watchlist ObjectId
    /// * `user_id` - User's ObjectId (for authorization)
    /// * `request` - Update request with new name
    ///
    /// # Returns
    ///
    /// * `Ok(WatchlistResponse)` - The updated watchlist
    #[instrument(skip(self))]
    pub async fn update(
        &self,
        id: &ObjectId,
        user_id: &ObjectId,
        request: UpdateWatchlistRequest,
    ) -> Result<WatchlistResponse> {
        info!(watchlist_id = %id, user_id = %user_id, new_name = %request.name, "Updating watchlist");

        // Check if watchlist exists and is owned by user
        let filter = doc! {
            "_id": id,
            "user_id": user_id
        };

        let watchlist = self
            .watchlists
            .find_one(filter.clone(), None)
            .await?
            .ok_or_else(|| AppError::NotFound("Watchlist not found".to_string()))?;

        // Check for duplicate name (excluding current watchlist)
        let duplicate = self
            .watchlists
            .find_one(
                doc! {
                    "user_id": user_id,
                    "name": &request.name,
                    "_id": { "$ne": id }
                },
                None,
            )
            .await?;

        if duplicate.is_some() {
            return Err(AppError::AlreadyExists(
                "A watchlist with this name already exists".to_string(),
            ));
        }

        // Update the watchlist
        let update = doc! {
            "$set": {
                "name": &request.name,
                "updated_at": BsonDateTime::now()
            }
        };

        self.watchlists.update_one(filter, update, None).await?;

        // Return updated watchlist
        let mut updated = watchlist;
        updated.name = request.name;
        updated.updated_at = chrono::Utc::now();

        info!(watchlist_id = %id, "Watchlist updated successfully");

        Ok(WatchlistResponse::from(updated))
    }

    /// Delete a watchlist
    ///
    /// # Arguments
    ///
    /// * `id` - Watchlist ObjectId
    /// * `user_id` - User's ObjectId (for authorization)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - On successful deletion
    #[instrument(skip(self))]
    pub async fn delete(&self, id: &ObjectId, user_id: &ObjectId) -> Result<()> {
        info!(watchlist_id = %id, user_id = %user_id, "Deleting watchlist");

        let filter = doc! {
            "_id": id,
            "user_id": user_id
        };

        let result = self.watchlists.delete_one(filter, None).await?;

        if result.deleted_count == 0 {
            return Err(AppError::NotFound("Watchlist not found".to_string()));
        }

        info!(watchlist_id = %id, "Watchlist deleted successfully");

        Ok(())
    }

    /// Add an asset to a watchlist
    ///
    /// # Arguments
    ///
    /// * `watchlist_id` - Watchlist ObjectId
    /// * `user_id` - User's ObjectId (for authorization)
    /// * `request` - Request containing the symbol to add
    ///
    /// # Returns
    ///
    /// * `Ok(WatchlistResponse)` - The updated watchlist
    #[instrument(skip(self))]
    pub async fn add_asset(
        &self,
        watchlist_id: &ObjectId,
        user_id: &ObjectId,
        request: AddAssetRequest,
    ) -> Result<WatchlistResponse> {
        let symbol = request.symbol.to_uppercase();
        info!(
            watchlist_id = %watchlist_id,
            user_id = %user_id,
            symbol = %symbol,
            "Adding asset to watchlist"
        );

        // Get the watchlist
        let filter = doc! {
            "_id": watchlist_id,
            "user_id": user_id
        };

        let mut watchlist = self
            .watchlists
            .find_one(filter.clone(), None)
            .await?
            .ok_or_else(|| AppError::NotFound("Watchlist not found".to_string()))?;

        // Check asset limit
        if watchlist.asset_count() >= self.max_assets_per_watchlist as usize {
            warn!(
                watchlist_id = %watchlist_id,
                count = watchlist.asset_count(),
                limit = self.max_assets_per_watchlist,
                "Asset limit exceeded"
            );
            return Err(AppError::LimitExceeded(format!(
                "Maximum {} assets allowed per watchlist",
                self.max_assets_per_watchlist
            )));
        }

        // Check if symbol already exists
        if watchlist.contains_symbol(&symbol) {
            return Err(AppError::AlreadyExists(format!(
                "Symbol {} is already in this watchlist",
                symbol
            )));
        }

        // Verify the symbol exists
        if !self.asset_service.verify_symbol(&symbol).await? {
            return Err(AppError::NotFound(format!(
                "Stock symbol {} not found",
                symbol
            )));
        }

        // Add the asset
        let asset = WatchlistAsset::new(symbol.clone());
        watchlist.add_asset(asset.clone());

        // Update database
        let asset_doc = mongodb::bson::to_document(&asset)
            .map_err(|e| AppError::DatabaseError(format!("Failed to serialize asset: {}", e)))?;
        let update = doc! {
            "$push": {
                "assets": asset_doc
            },
            "$set": {
                "updated_at": BsonDateTime::now()
            }
        };

        self.watchlists.update_one(filter, update, None).await?;

        info!(
            watchlist_id = %watchlist_id,
            symbol = %symbol,
            "Asset added successfully"
        );

        Ok(WatchlistResponse::from(watchlist))
    }

    /// Remove an asset from a watchlist
    ///
    /// # Arguments
    ///
    /// * `watchlist_id` - Watchlist ObjectId
    /// * `user_id` - User's ObjectId (for authorization)
    /// * `symbol` - Stock symbol to remove
    ///
    /// # Returns
    ///
    /// * `Ok(WatchlistResponse)` - The updated watchlist
    #[instrument(skip(self))]
    pub async fn remove_asset(
        &self,
        watchlist_id: &ObjectId,
        user_id: &ObjectId,
        symbol: &str,
    ) -> Result<WatchlistResponse> {
        let symbol_upper = symbol.to_uppercase();
        info!(
            watchlist_id = %watchlist_id,
            user_id = %user_id,
            symbol = %symbol_upper,
            "Removing asset from watchlist"
        );

        // Get the watchlist
        let filter = doc! {
            "_id": watchlist_id,
            "user_id": user_id
        };

        let mut watchlist = self
            .watchlists
            .find_one(filter.clone(), None)
            .await?
            .ok_or_else(|| AppError::NotFound("Watchlist not found".to_string()))?;

        // Check if symbol exists in watchlist
        if !watchlist.contains_symbol(&symbol_upper) {
            return Err(AppError::NotFound(format!(
                "Symbol {} not found in this watchlist",
                symbol_upper
            )));
        }

        // Remove the asset
        watchlist.remove_asset(&symbol_upper);

        // Update database using case-insensitive removal
        let update = doc! {
            "$pull": {
                "assets": {
                    "symbol": {
                        "$regex": format!("^{}$", symbol_upper),
                        "$options": "i"
                    }
                }
            },
            "$set": {
                "updated_at": BsonDateTime::now()
            }
        };

        self.watchlists.update_one(filter, update, None).await?;

        info!(
            watchlist_id = %watchlist_id,
            symbol = %symbol_upper,
            "Asset removed successfully"
        );

        Ok(WatchlistResponse::from(watchlist))
    }

    /// Get watchlists containing a specific symbol
    ///
    /// # Arguments
    ///
    /// * `user_id` - User's ObjectId
    /// * `symbol` - Stock symbol to search for
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<WatchlistSummary>)` - Watchlists containing the symbol
    #[instrument(skip(self))]
    pub async fn get_by_symbol(
        &self,
        user_id: &ObjectId,
        symbol: &str,
    ) -> Result<Vec<WatchlistSummary>> {
        let symbol_upper = symbol.to_uppercase();

        let filter = doc! {
            "user_id": user_id,
            "assets.symbol": {
                "$regex": format!("^{}$", symbol_upper),
                "$options": "i"
            }
        };

        let mut cursor = self.watchlists.find(filter, None).await?;

        let mut watchlists = Vec::new();
        while let Some(watchlist) = cursor.try_next().await? {
            watchlists.push(WatchlistSummary::from(watchlist));
        }

        Ok(watchlists)
    }
}

#[cfg(test)]
mod tests {
    // Integration tests require database connection
    // See tests/integration/api/watchlist_test.rs
}
