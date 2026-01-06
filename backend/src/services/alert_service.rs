//! Alert service
//!
//! This service handles all alert CRUD operations with proper
//! user isolation and business logic validation.

use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use mongodb::options::FindOptions;
use mongodb::Collection;
use tracing::{info, instrument, warn};

use crate::config::database::MongoDb;
use crate::models::alert::{
    Alert, AlertListResponse, AlertResponse, AlertStatus, AlertType, CreateAlertRequest,
    UpdateAlertRequest,
};
use crate::services::asset_service::AssetService;
use crate::utils::error::{AppError, Result};

/// Alert service for CRUD operations
#[derive(Clone)]
pub struct AlertService {
    alerts: Collection<Alert>,
    asset_service: AssetService,
    max_alerts_per_user: u32,
}

impl AlertService {
    /// Create a new AlertService instance
    pub fn new(db: &MongoDb, asset_service: AssetService, max_alerts_per_user: u32) -> Self {
        Self {
            alerts: db.database().collection("alerts"),
            asset_service,
            max_alerts_per_user,
        }
    }

    /// Get all alerts for a user
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user's ObjectId
    ///
    /// # Returns
    ///
    /// * `Ok(AlertListResponse)` - List of user's alerts
    #[instrument(skip(self))]
    pub async fn get_all(&self, user_id: &ObjectId) -> Result<AlertListResponse> {
        info!(user_id = %user_id, "Fetching all alerts for user");

        let filter = doc! { "user_id": user_id };
        let options = FindOptions::builder()
            .sort(doc! { "created_at": -1 })
            .build();

        let mut cursor = self.alerts.find(filter, options).await?;

        let mut alerts = Vec::new();
        while let Some(alert) = cursor.try_next().await? {
            alerts.push(AlertResponse::from(alert));
        }

        let total = alerts.len();

        Ok(AlertListResponse { alerts, total })
    }

    /// Get a specific alert by ID
    ///
    /// # Arguments
    ///
    /// * `id` - Alert ObjectId
    /// * `user_id` - User's ObjectId (for authorization)
    ///
    /// # Returns
    ///
    /// * `Ok(AlertResponse)` - The alert
    /// * `Err(AppError::NotFound)` - If not found or not owned by user
    #[instrument(skip(self))]
    pub async fn get_by_id(&self, id: &ObjectId, user_id: &ObjectId) -> Result<AlertResponse> {
        info!(alert_id = %id, user_id = %user_id, "Fetching alert");

        let filter = doc! {
            "_id": id,
            "user_id": user_id
        };

        let alert = self
            .alerts
            .find_one(filter, None)
            .await?
            .ok_or_else(|| AppError::NotFound("Alert not found".to_string()))?;

        Ok(AlertResponse::from(alert))
    }

    /// Create a new alert
    ///
    /// # Arguments
    ///
    /// * `user_id` - User's ObjectId
    /// * `request` - Create request with alert details
    ///
    /// # Returns
    ///
    /// * `Ok(AlertResponse)` - The created alert
    /// * `Err(AppError::LimitExceeded)` - If user has too many alerts
    /// * `Err(AppError::AlreadyExists)` - If duplicate alert exists
    #[instrument(skip(self))]
    pub async fn create(
        &self,
        user_id: &ObjectId,
        request: CreateAlertRequest,
    ) -> Result<AlertResponse> {
        let symbol = request.symbol.to_uppercase();
        info!(
            user_id = %user_id,
            symbol = %symbol,
            target_price = request.target_price,
            condition = ?request.condition,
            "Creating new alert"
        );

        // Check alert limit
        let count = self
            .alerts
            .count_documents(doc! { "user_id": user_id }, None)
            .await?;

        if count >= self.max_alerts_per_user as u64 {
            warn!(
                user_id = %user_id,
                count = count,
                limit = self.max_alerts_per_user,
                "Alert limit exceeded"
            );
            return Err(AppError::LimitExceeded(format!(
                "Maximum {} alerts allowed per user",
                self.max_alerts_per_user
            )));
        }

        // Verify the symbol exists
        if !self.asset_service.verify_symbol(&symbol).await? {
            return Err(AppError::NotFound(format!(
                "Stock symbol {} not found",
                symbol
            )));
        }

        // Check for duplicate alert (same user, symbol, condition, target_price, status=active)
        let existing = self
            .alerts
            .find_one(
                doc! {
                    "user_id": user_id,
                    "symbol": &symbol,
                    "condition": request.condition.as_str(),
                    "target_price": request.target_price,
                    "status": "active"
                },
                None,
            )
            .await?;

        if existing.is_some() {
            return Err(AppError::AlreadyExists(
                "An active alert with these parameters already exists".to_string(),
            ));
        }

        // Create alert
        let alert = Alert::new(
            *user_id,
            symbol,
            request.target_price,
            request.condition,
            request.alert_type,
            request.note,
        );

        self.alerts.insert_one(&alert, None).await?;

        info!(alert_id = %alert.id, "Alert created successfully");

        Ok(AlertResponse::from(alert))
    }

    /// Update an alert
    ///
    /// # Arguments
    ///
    /// * `id` - Alert ObjectId
    /// * `user_id` - User's ObjectId (for authorization)
    /// * `request` - Update request with new values
    ///
    /// # Returns
    ///
    /// * `Ok(AlertResponse)` - The updated alert
    #[instrument(skip(self))]
    pub async fn update(
        &self,
        id: &ObjectId,
        user_id: &ObjectId,
        request: UpdateAlertRequest,
    ) -> Result<AlertResponse> {
        info!(alert_id = %id, user_id = %user_id, "Updating alert");

        // Check if alert exists and is owned by user
        let filter = doc! {
            "_id": id,
            "user_id": user_id
        };

        let alert = self
            .alerts
            .find_one(filter.clone(), None)
            .await?
            .ok_or_else(|| AppError::NotFound("Alert not found".to_string()))?;

        // Build update document
        let mut set_doc = doc! {
            "updated_at": BsonDateTime::now()
        };

        if let Some(target_price) = request.target_price {
            set_doc.insert("target_price", target_price);
        }

        if let Some(condition) = &request.condition {
            set_doc.insert("condition", condition.as_str());
        }

        if let Some(alert_type) = &request.alert_type {
            set_doc.insert("alert_type", alert_type.as_str());
        }

        if let Some(note) = &request.note {
            set_doc.insert("note", note);
        }

        let update = doc! { "$set": set_doc };

        self.alerts.update_one(filter, update, None).await?;

        // Return updated alert
        let mut updated = alert;
        if let Some(target_price) = request.target_price {
            updated.target_price = target_price;
        }
        if let Some(condition) = request.condition {
            updated.condition = condition;
        }
        if let Some(alert_type) = request.alert_type {
            updated.alert_type = alert_type;
        }
        if let Some(note) = request.note {
            updated.note = Some(note);
        }
        updated.updated_at = chrono::Utc::now();

        info!(alert_id = %id, "Alert updated successfully");

        Ok(AlertResponse::from(updated))
    }

    /// Delete an alert
    ///
    /// # Arguments
    ///
    /// * `id` - Alert ObjectId
    /// * `user_id` - User's ObjectId (for authorization)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - On successful deletion
    #[instrument(skip(self))]
    pub async fn delete(&self, id: &ObjectId, user_id: &ObjectId) -> Result<()> {
        info!(alert_id = %id, user_id = %user_id, "Deleting alert");

        let filter = doc! {
            "_id": id,
            "user_id": user_id
        };

        let result = self.alerts.delete_one(filter, None).await?;

        if result.deleted_count == 0 {
            return Err(AppError::NotFound("Alert not found".to_string()));
        }

        info!(alert_id = %id, "Alert deleted successfully");

        Ok(())
    }

    /// Toggle an alert's status between Active and Disabled
    ///
    /// # Arguments
    ///
    /// * `id` - Alert ObjectId
    /// * `user_id` - User's ObjectId (for authorization)
    ///
    /// # Returns
    ///
    /// * `Ok(AlertResponse)` - The toggled alert
    #[instrument(skip(self))]
    pub async fn toggle(&self, id: &ObjectId, user_id: &ObjectId) -> Result<AlertResponse> {
        info!(alert_id = %id, user_id = %user_id, "Toggling alert status");

        let filter = doc! {
            "_id": id,
            "user_id": user_id
        };

        let mut alert = self
            .alerts
            .find_one(filter.clone(), None)
            .await?
            .ok_or_else(|| AppError::NotFound("Alert not found".to_string()))?;

        // Toggle between Active and Disabled
        // If already Triggered (one-time alert), re-enable it as Active
        let new_status = match alert.status {
            AlertStatus::Active => AlertStatus::Disabled,
            AlertStatus::Disabled | AlertStatus::Triggered => AlertStatus::Active,
        };

        let update = doc! {
            "$set": {
                "status": new_status.as_str(),
                "updated_at": BsonDateTime::now()
            }
        };

        self.alerts.update_one(filter, update, None).await?;

        alert.status = new_status;
        alert.updated_at = chrono::Utc::now();

        info!(
            alert_id = %id,
            new_status = ?new_status,
            "Alert status toggled successfully"
        );

        Ok(AlertResponse::from(alert))
    }

    /// Get all active alerts (for background processor)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Alert>)` - List of active alerts
    #[instrument(skip(self))]
    pub async fn get_active_alerts(&self) -> Result<Vec<Alert>> {
        let filter = doc! { "status": "active" };
        let options = FindOptions::builder()
            .sort(doc! { "symbol": 1 })
            .build();

        let mut cursor = self.alerts.find(filter, options).await?;

        let mut alerts = Vec::new();
        while let Some(alert) = cursor.try_next().await? {
            alerts.push(alert);
        }

        Ok(alerts)
    }

    /// Update an alert after checking its condition
    ///
    /// # Arguments
    ///
    /// * `id` - Alert ObjectId
    /// * `current_price` - Current price of the asset
    /// * `triggered` - Whether the alert was triggered
    ///
    /// # Returns
    ///
    /// * `Ok(())` - On successful update
    #[instrument(skip(self))]
    pub async fn update_after_check(
        &self,
        id: &ObjectId,
        current_price: f64,
        triggered: bool,
    ) -> Result<()> {
        let filter = doc! { "_id": id };

        let mut update_doc = doc! {
            "last_checked_price": current_price,
            "updated_at": BsonDateTime::now()
        };

        if triggered {
            // Get the alert to check its type
            let alert = self
                .alerts
                .find_one(filter.clone(), None)
                .await?
                .ok_or_else(|| AppError::NotFound("Alert not found".to_string()))?;

            update_doc.insert("last_triggered_at", BsonDateTime::now());
            update_doc.insert("trigger_count", alert.trigger_count + 1);

            // If one-time alert, set status to triggered
            if alert.alert_type == AlertType::OneTime {
                update_doc.insert("status", AlertStatus::Triggered.as_str());
            }
        }

        let update = doc! { "$set": update_doc };
        self.alerts.update_one(filter, update, None).await?;

        Ok(())
    }

    /// Get alerts for a specific symbol (for background processor)
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock symbol
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Alert>)` - List of active alerts for the symbol
    #[instrument(skip(self))]
    pub async fn get_active_alerts_by_symbol(&self, symbol: &str) -> Result<Vec<Alert>> {
        let filter = doc! {
            "symbol": symbol.to_uppercase(),
            "status": "active"
        };

        let mut cursor = self.alerts.find(filter, None).await?;

        let mut alerts = Vec::new();
        while let Some(alert) = cursor.try_next().await? {
            alerts.push(alert);
        }

        Ok(alerts)
    }

    /// Get unique symbols with active alerts (for efficient batch processing)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<String>)` - List of unique symbols with active alerts
    #[instrument(skip(self))]
    pub async fn get_symbols_with_active_alerts(&self) -> Result<Vec<String>> {
        let pipeline = vec![
            doc! { "$match": { "status": "active" } },
            doc! { "$group": { "_id": "$symbol" } },
            doc! { "$sort": { "_id": 1 } },
        ];

        let mut cursor = self
            .alerts
            .aggregate(pipeline, None)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to aggregate symbols: {}", e)))?;

        let mut symbols = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            if let Some(symbol) = doc.get_str("_id").ok() {
                symbols.push(symbol.to_string());
            }
        }

        Ok(symbols)
    }
}

#[cfg(test)]
mod tests {
    // Integration tests require database connection
    // See tests/integration/api/alert_test.rs
}
