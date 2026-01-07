//! Alert processor service
//!
//! This service runs in the background and periodically checks all active alerts
//! against current market prices. When an alert condition is met, it triggers
//! the appropriate notification.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;
use tracing::{debug, error, info, instrument, warn};

use crate::models::alert::{Alert, AlertNotification};
use crate::services::alert_service::AlertService;
use crate::services::asset_service::AssetService;
use crate::services::notification_sender::NotificationSender;
use crate::utils::error::Result;

/// Default check interval in seconds
const DEFAULT_CHECK_INTERVAL_SECONDS: u64 = 30;

/// Default cooldown for recurring alerts in seconds
const DEFAULT_RECURRING_COOLDOWN_SECONDS: i64 = 300;

/// Configuration for the alert processor
#[derive(Debug, Clone)]
pub struct AlertProcessorConfig {
    /// Interval between alert checks in seconds
    pub check_interval_seconds: u64,
    /// Cooldown period for recurring alerts in seconds
    pub recurring_cooldown_seconds: i64,
    /// Maximum number of alerts to process in a single batch
    pub batch_size: usize,
}

impl Default for AlertProcessorConfig {
    fn default() -> Self {
        Self {
            check_interval_seconds: DEFAULT_CHECK_INTERVAL_SECONDS,
            recurring_cooldown_seconds: DEFAULT_RECURRING_COOLDOWN_SECONDS,
            batch_size: 100,
        }
    }
}

impl AlertProcessorConfig {
    /// Create config from environment variables
    pub fn from_env() -> Self {
        let check_interval = std::env::var("ALERT_CHECK_INTERVAL_SECONDS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_CHECK_INTERVAL_SECONDS);

        let recurring_cooldown = std::env::var("ALERT_RECURRING_COOLDOWN_SECONDS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_RECURRING_COOLDOWN_SECONDS);

        let batch_size = std::env::var("ALERT_BATCH_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(100);

        Self {
            check_interval_seconds: check_interval,
            recurring_cooldown_seconds: recurring_cooldown,
            batch_size,
        }
    }
}

/// Alert processor that runs in the background
pub struct AlertProcessor {
    alert_service: AlertService,
    asset_service: AssetService,
    notification_sender: Arc<dyn NotificationSender>,
    config: AlertProcessorConfig,
    /// Track last known prices for "crosses" condition detection
    last_prices: Arc<RwLock<HashMap<String, f64>>>,
    /// Flag to stop the processor
    running: Arc<RwLock<bool>>,
}

impl AlertProcessor {
    /// Create a new AlertProcessor instance
    pub fn new(
        alert_service: AlertService,
        asset_service: AssetService,
        notification_sender: Arc<dyn NotificationSender>,
        config: AlertProcessorConfig,
    ) -> Self {
        Self {
            alert_service,
            asset_service,
            notification_sender,
            config,
            last_prices: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Start the background processor
    ///
    /// This method spawns a tokio task that runs the alert checking loop.
    /// Returns a handle that can be used to stop the processor.
    pub fn start(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        let processor = self.clone();

        tokio::spawn(async move {
            processor.run().await;
        })
    }

    /// Run the main processing loop
    #[instrument(skip(self), name = "alert_processor_run")]
    pub async fn run(&self) {
        info!(
            check_interval = self.config.check_interval_seconds,
            "Starting alert processor"
        );

        {
            let mut running = self.running.write().await;
            *running = true;
        }

        let mut interval = time::interval(Duration::from_secs(self.config.check_interval_seconds));

        loop {
            interval.tick().await;

            // Check if we should stop
            {
                let running = self.running.read().await;
                if !*running {
                    info!("Alert processor stopping");
                    break;
                }
            }

            // Process a batch of alerts
            if let Err(e) = self.process_batch().await {
                error!(error = %e, "Error processing alert batch");
            }
        }
    }

    /// Stop the processor
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
        info!("Alert processor stop requested");
    }

    /// Check if the processor is running
    pub async fn is_running(&self) -> bool {
        let running = self.running.read().await;
        *running
    }

    /// Process a batch of alerts
    #[instrument(skip(self), name = "process_alert_batch")]
    pub async fn process_batch(&self) -> Result<()> {
        debug!("Processing alert batch");

        // Get unique symbols with active alerts
        let symbols = self.alert_service.get_symbols_with_active_alerts().await?;

        if symbols.is_empty() {
            debug!("No active alerts to process");
            return Ok(());
        }

        debug!(symbol_count = symbols.len(), "Processing alerts for symbols");

        // Fetch current prices for all symbols
        let mut current_prices: HashMap<String, f64> = HashMap::new();
        for symbol in &symbols {
            match self.asset_service.get_quote(symbol).await {
                Ok(asset) => {
                    current_prices.insert(symbol.clone(), asset.price);
                }
                Err(e) => {
                    warn!(symbol = %symbol, error = %e, "Failed to fetch price for symbol");
                }
            }
        }

        // Get all active alerts
        let alerts = self.alert_service.get_active_alerts().await?;

        // Process each alert
        let mut triggered_count = 0;
        for alert in alerts {
            if let Some(&current_price) = current_prices.get(&alert.symbol) {
                match self.check_and_process_alert(&alert, current_price).await {
                    Ok(triggered) => {
                        if triggered {
                            triggered_count += 1;
                        }
                    }
                    Err(e) => {
                        warn!(
                            alert_id = %alert.id,
                            symbol = %alert.symbol,
                            error = %e,
                            "Failed to process alert"
                        );
                    }
                }
            }
        }

        // Update last prices cache
        {
            let mut last_prices = self.last_prices.write().await;
            for (symbol, price) in current_prices {
                last_prices.insert(symbol, price);
            }
        }

        if triggered_count > 0 {
            info!(triggered_count = triggered_count, "Alerts triggered in batch");
        }

        Ok(())
    }

    /// Check a single alert and process it if triggered
    #[instrument(skip(self, alert), fields(alert_id = %alert.id, symbol = %alert.symbol))]
    async fn check_and_process_alert(&self, alert: &Alert, current_price: f64) -> Result<bool> {
        // Check if alert is on cooldown (for recurring alerts)
        if alert.is_on_cooldown(self.config.recurring_cooldown_seconds) {
            debug!(
                alert_id = %alert.id,
                cooldown_seconds = self.config.recurring_cooldown_seconds,
                "Alert is on cooldown"
            );
            return Ok(false);
        }

        // Get previous price for "crosses" condition
        let previous_price = {
            let last_prices = self.last_prices.read().await;
            last_prices.get(&alert.symbol).copied()
        };

        // Use last_checked_price from DB if we don't have it in memory
        let previous_price = previous_price.or(alert.last_checked_price);

        // Check if the condition is met
        let triggered = alert.check_condition(current_price, previous_price);

        // Update the alert's last checked price
        self.alert_service
            .update_after_check(&alert.id, current_price, triggered)
            .await?;

        if triggered {
            self.handle_triggered_alert(alert, current_price).await?;
        }

        Ok(triggered)
    }

    /// Handle a triggered alert
    #[instrument(skip(self, alert), fields(alert_id = %alert.id, symbol = %alert.symbol))]
    async fn handle_triggered_alert(&self, alert: &Alert, current_price: f64) -> Result<()> {
        info!(
            alert_id = %alert.id,
            symbol = %alert.symbol,
            target_price = alert.target_price,
            current_price = current_price,
            condition = ?alert.condition,
            "Alert triggered!"
        );

        // Create notification
        let notification = AlertNotification::new(alert, current_price);

        // Send notification
        self.notification_sender.send(notification).await?;

        Ok(())
    }

    /// Process a single symbol's alerts (useful for testing or manual processing)
    #[instrument(skip(self), fields(symbol = %symbol))]
    pub async fn process_symbol(&self, symbol: &str) -> Result<Vec<Alert>> {
        let alerts = self.alert_service.get_active_alerts_by_symbol(symbol).await?;

        if alerts.is_empty() {
            return Ok(vec![]);
        }

        // Get current price
        let asset = self.asset_service.get_quote(symbol).await?;
        let current_price = asset.price;

        let mut triggered_alerts = Vec::new();

        for alert in alerts {
            match self.check_and_process_alert(&alert, current_price).await {
                Ok(triggered) => {
                    if triggered {
                        triggered_alerts.push(alert);
                    }
                }
                Err(e) => {
                    warn!(
                        alert_id = %alert.id,
                        error = %e,
                        "Failed to process alert"
                    );
                }
            }
        }

        // Update last price cache
        {
            let mut last_prices = self.last_prices.write().await;
            last_prices.insert(symbol.to_uppercase(), current_price);
        }

        Ok(triggered_alerts)
    }

    /// Get the current price cache (for debugging/monitoring)
    pub async fn get_price_cache(&self) -> HashMap<String, f64> {
        let last_prices = self.last_prices.read().await;
        last_prices.clone()
    }

    /// Clear the price cache
    pub async fn clear_price_cache(&self) {
        let mut last_prices = self.last_prices.write().await;
        last_prices.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_processor_config_default() {
        let config = AlertProcessorConfig::default();
        assert_eq!(config.check_interval_seconds, DEFAULT_CHECK_INTERVAL_SECONDS);
        assert_eq!(config.recurring_cooldown_seconds, DEFAULT_RECURRING_COOLDOWN_SECONDS);
        assert_eq!(config.batch_size, 100);
    }

    #[test]
    fn test_alert_processor_config_from_env() {
        // This test verifies that from_env uses default values when env vars aren't set
        let config = AlertProcessorConfig::from_env();
        assert!(config.check_interval_seconds > 0);
        assert!(config.recurring_cooldown_seconds > 0);
        assert!(config.batch_size > 0);
    }
}
