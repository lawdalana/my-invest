//! WebSocket service for managing real-time price updates
//!
//! This service handles:
//! - Managing WebSocket connections and their subscriptions
//! - Broadcasting price updates to subscribed clients
//! - Background task for fetching and distributing price updates

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use chrono::Utc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, info, instrument, warn};

use crate::models::websocket::{
    PriceUpdate, WsError, WsErrorCode, WsServerMessage, MAX_SUBSCRIPTIONS_PER_CONNECTION,
};

/// Channel capacity for price updates (unused but reserved for future use)
#[allow(dead_code)]
use crate::services::AssetService;

/// Channel capacity for price updates
const BROADCAST_CHANNEL_CAPACITY: usize = 1000;

/// Internal command for managing connections
#[derive(Debug)]
pub enum WsCommand {
    /// Register a new connection
    Connect {
        connection_id: String,
        sender: mpsc::Sender<WsServerMessage>,
    },

    /// Remove a connection
    Disconnect {
        connection_id: String,
    },

    /// Subscribe to symbols
    Subscribe {
        connection_id: String,
        symbols: Vec<String>,
    },

    /// Unsubscribe from symbols
    Unsubscribe {
        connection_id: String,
        symbols: Vec<String>,
    },
}

/// Connection state tracking
#[derive(Debug)]
struct ConnectionState {
    /// Sender channel to this connection
    sender: mpsc::Sender<WsServerMessage>,

    /// Set of subscribed symbols for this connection
    subscriptions: HashSet<String>,
}

/// WebSocket service for managing connections and broadcasting updates
#[derive(Clone)]
pub struct WsService {
    /// Command channel for connection management
    command_tx: mpsc::Sender<WsCommand>,

    /// Shared state: connection ID -> connection state
    connections: Arc<RwLock<HashMap<String, ConnectionState>>>,

    /// Shared state: symbol -> set of subscribed connection IDs
    subscriptions: Arc<RwLock<HashMap<String, HashSet<String>>>>,

    /// Asset service for fetching prices
    asset_service: AssetService,

    /// Update interval in seconds
    update_interval_seconds: u64,
}

impl WsService {
    /// Create a new WebSocket service
    pub fn new(asset_service: AssetService, update_interval_seconds: u64) -> Self {
        let (command_tx, command_rx) = mpsc::channel(100);

        let connections = Arc::new(RwLock::new(HashMap::new()));
        let subscriptions = Arc::new(RwLock::new(HashMap::new()));

        let service = Self {
            command_tx,
            connections: connections.clone(),
            subscriptions: subscriptions.clone(),
            asset_service,
            update_interval_seconds,
        };

        // Spawn the command processor
        let service_clone = service.clone();
        tokio::spawn(async move {
            service_clone.process_commands(command_rx).await;
        });

        service
    }

    /// Process commands from the command channel
    async fn process_commands(&self, mut command_rx: mpsc::Receiver<WsCommand>) {
        info!("WebSocket command processor started");

        while let Some(command) = command_rx.recv().await {
            match command {
                WsCommand::Connect { connection_id, sender } => {
                    self.handle_connect(connection_id, sender).await;
                }
                WsCommand::Disconnect { connection_id } => {
                    self.handle_disconnect(connection_id).await;
                }
                WsCommand::Subscribe { connection_id, symbols } => {
                    self.handle_subscribe(connection_id, symbols).await;
                }
                WsCommand::Unsubscribe { connection_id, symbols } => {
                    self.handle_unsubscribe(connection_id, symbols).await;
                }
            }
        }

        info!("WebSocket command processor stopped");
    }

    /// Handle new connection
    async fn handle_connect(&self, connection_id: String, sender: mpsc::Sender<WsServerMessage>) {
        debug!(connection_id = %connection_id, "New WebSocket connection");

        let state = ConnectionState {
            sender: sender.clone(),
            subscriptions: HashSet::new(),
        };

        self.connections.write().await.insert(connection_id.clone(), state);

        // Send welcome message
        let welcome = WsServerMessage::Connected {
            message: "Connected to My-Invest real-time price updates".to_string(),
            timestamp: Utc::now(),
        };

        if let Err(e) = sender.send(welcome).await {
            warn!(connection_id = %connection_id, error = %e, "Failed to send welcome message");
        }
    }

    /// Handle connection disconnect
    async fn handle_disconnect(&self, connection_id: String) {
        debug!(connection_id = %connection_id, "WebSocket connection disconnected");

        // Remove from connections and get subscriptions
        let subscribed_symbols = {
            let mut connections = self.connections.write().await;
            if let Some(state) = connections.remove(&connection_id) {
                state.subscriptions
            } else {
                return;
            }
        };

        // Remove from subscription mappings
        let mut subscriptions = self.subscriptions.write().await;
        for symbol in subscribed_symbols {
            if let Some(connections) = subscriptions.get_mut(&symbol) {
                connections.remove(&connection_id);
                if connections.is_empty() {
                    subscriptions.remove(&symbol);
                }
            }
        }
    }

    /// Handle subscription request
    async fn handle_subscribe(&self, connection_id: String, symbols: Vec<String>) {
        let mut connections = self.connections.write().await;
        let mut subscriptions = self.subscriptions.write().await;

        let state = match connections.get_mut(&connection_id) {
            Some(state) => state,
            None => {
                warn!(connection_id = %connection_id, "Subscribe request for unknown connection");
                return;
            }
        };

        // Validate and filter symbols
        let mut added_symbols = Vec::new();
        let mut errors = Vec::new();

        for symbol in symbols {
            // Normalize symbol to uppercase
            let symbol = symbol.to_uppercase();

            // Check subscription limit
            if state.subscriptions.len() >= MAX_SUBSCRIPTIONS_PER_CONNECTION {
                errors.push((symbol.clone(), "Subscription limit exceeded"));
                continue;
            }

            // Skip if already subscribed
            if state.subscriptions.contains(&symbol) {
                continue;
            }

            // Validate symbol format (basic validation)
            if symbol.is_empty() || symbol.len() > 20 {
                errors.push((symbol.clone(), "Invalid symbol format"));
                continue;
            }

            // Add to connection's subscriptions
            state.subscriptions.insert(symbol.clone());

            // Add to global subscription mapping
            subscriptions
                .entry(symbol.clone())
                .or_insert_with(HashSet::new)
                .insert(connection_id.clone());

            added_symbols.push(symbol);
        }

        // Send confirmation
        if !added_symbols.is_empty() {
            let msg = WsServerMessage::Subscribed {
                symbols: added_symbols.clone(),
                total_subscriptions: state.subscriptions.len(),
            };

            if let Err(e) = state.sender.send(msg).await {
                warn!(connection_id = %connection_id, error = %e, "Failed to send subscription confirmation");
            }

            debug!(
                connection_id = %connection_id,
                symbols = ?added_symbols,
                "Subscribed to symbols"
            );
        }

        // Send errors if any
        for (symbol, reason) in errors {
            let error = WsError::new(
                WsErrorCode::InvalidSymbol,
                format!("Failed to subscribe to {}: {}", symbol, reason),
            );
            let msg = WsServerMessage::Error(error);

            if let Err(e) = state.sender.send(msg).await {
                warn!(connection_id = %connection_id, error = %e, "Failed to send error message");
            }
        }
    }

    /// Handle unsubscription request
    async fn handle_unsubscribe(&self, connection_id: String, symbols: Vec<String>) {
        let mut connections = self.connections.write().await;
        let mut subscriptions = self.subscriptions.write().await;

        let state = match connections.get_mut(&connection_id) {
            Some(state) => state,
            None => {
                warn!(connection_id = %connection_id, "Unsubscribe request for unknown connection");
                return;
            }
        };

        let mut removed_symbols = Vec::new();

        for symbol in symbols {
            let symbol = symbol.to_uppercase();

            // Remove from connection's subscriptions
            if state.subscriptions.remove(&symbol) {
                removed_symbols.push(symbol.clone());

                // Remove from global subscription mapping
                if let Some(connections) = subscriptions.get_mut(&symbol) {
                    connections.remove(&connection_id);
                    if connections.is_empty() {
                        subscriptions.remove(&symbol);
                    }
                }
            }
        }

        // Send confirmation
        if !removed_symbols.is_empty() {
            let msg = WsServerMessage::Unsubscribed {
                symbols: removed_symbols.clone(),
                total_subscriptions: state.subscriptions.len(),
            };

            if let Err(e) = state.sender.send(msg).await {
                warn!(connection_id = %connection_id, error = %e, "Failed to send unsubscription confirmation");
            }

            debug!(
                connection_id = %connection_id,
                symbols = ?removed_symbols,
                "Unsubscribed from symbols"
            );
        }
    }

    /// Register a new connection
    pub async fn connect(&self, connection_id: String, sender: mpsc::Sender<WsServerMessage>) {
        let _ = self
            .command_tx
            .send(WsCommand::Connect { connection_id, sender })
            .await;
    }

    /// Disconnect a connection
    pub async fn disconnect(&self, connection_id: &str) {
        let _ = self
            .command_tx
            .send(WsCommand::Disconnect {
                connection_id: connection_id.to_string(),
            })
            .await;
    }

    /// Subscribe to symbols
    pub async fn subscribe(&self, connection_id: &str, symbols: Vec<String>) {
        let _ = self
            .command_tx
            .send(WsCommand::Subscribe {
                connection_id: connection_id.to_string(),
                symbols,
            })
            .await;
    }

    /// Unsubscribe from symbols
    pub async fn unsubscribe(&self, connection_id: &str, symbols: Vec<String>) {
        let _ = self
            .command_tx
            .send(WsCommand::Unsubscribe {
                connection_id: connection_id.to_string(),
                symbols,
            })
            .await;
    }

    /// Get all symbols that have at least one subscriber
    pub async fn get_subscribed_symbols(&self) -> Vec<String> {
        self.subscriptions.read().await.keys().cloned().collect()
    }

    /// Get the number of active connections
    #[allow(dead_code)]
    pub async fn connection_count(&self) -> usize {
        self.connections.read().await.len()
    }

    /// Get the number of unique subscribed symbols
    #[allow(dead_code)]
    pub async fn subscription_count(&self) -> usize {
        self.subscriptions.read().await.len()
    }

    /// Broadcast a price update to all subscribed connections
    #[instrument(skip(self))]
    pub async fn broadcast_price_update(&self, update: PriceUpdate) {
        let symbol = &update.symbol;

        // Get connections subscribed to this symbol
        let connection_ids: Vec<String> = {
            let subscriptions = self.subscriptions.read().await;
            match subscriptions.get(symbol) {
                Some(ids) => ids.iter().cloned().collect(),
                None => return, // No subscribers
            }
        };

        if connection_ids.is_empty() {
            return;
        }

        // Send to all subscribed connections
        let connections = self.connections.read().await;
        let msg = WsServerMessage::PriceUpdate(update.clone());

        for connection_id in connection_ids {
            if let Some(state) = connections.get(&connection_id) {
                if let Err(e) = state.sender.send(msg.clone()).await {
                    warn!(
                        connection_id = %connection_id,
                        symbol = %symbol,
                        error = %e,
                        "Failed to send price update"
                    );
                }
            }
        }
    }

    /// Start the background price update task
    ///
    /// This task periodically fetches prices for all subscribed symbols
    /// and broadcasts updates to subscribed clients.
    pub async fn run_price_updater(self: Arc<Self>) {
        info!(
            interval_seconds = self.update_interval_seconds,
            "Starting WebSocket price updater"
        );

        let mut interval = tokio::time::interval(
            tokio::time::Duration::from_secs(self.update_interval_seconds)
        );

        loop {
            interval.tick().await;

            // Get all subscribed symbols
            let symbols = self.get_subscribed_symbols().await;

            if symbols.is_empty() {
                debug!("No symbols subscribed, skipping price update");
                continue;
            }

            debug!(
                symbol_count = symbols.len(),
                "Fetching prices for subscribed symbols"
            );

            // Fetch prices for all subscribed symbols
            for symbol in symbols {
                match self.asset_service.get_quote(&symbol).await {
                    Ok(asset) => {
                        let update = PriceUpdate::from_asset(&asset);
                        self.broadcast_price_update(update).await;
                    }
                    Err(e) => {
                        // Log but don't fail - continue with other symbols
                        warn!(
                            symbol = %symbol,
                            error = %e,
                            "Failed to fetch price for symbol"
                        );
                    }
                }
            }
        }
    }
}

/// Configuration for the WebSocket service
#[derive(Debug, Clone)]
pub struct WsConfig {
    /// Interval between price updates in seconds
    pub update_interval_seconds: u64,

    /// Maximum subscriptions per connection
    #[allow(dead_code)]
    pub max_subscriptions_per_connection: usize,
}

impl Default for WsConfig {
    fn default() -> Self {
        Self {
            update_interval_seconds: 30,
            max_subscriptions_per_connection: MAX_SUBSCRIPTIONS_PER_CONNECTION,
        }
    }
}

impl WsConfig {
    /// Load configuration from environment
    pub fn from_env() -> Self {
        Self {
            update_interval_seconds: std::env::var("WS_UPDATE_INTERVAL_SECONDS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            max_subscriptions_per_connection: std::env::var("WS_MAX_SUBSCRIPTIONS_PER_CONNECTION")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(MAX_SUBSCRIPTIONS_PER_CONNECTION),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    // Note: Full tests require a mocked AssetService
    // These are basic unit tests for the data structures

    #[test]
    fn test_ws_config_default() {
        let config = WsConfig::default();
        assert_eq!(config.update_interval_seconds, 30);
        assert_eq!(config.max_subscriptions_per_connection, MAX_SUBSCRIPTIONS_PER_CONNECTION);
    }

    #[tokio::test]
    async fn test_connection_state() {
        let (tx, _rx) = mpsc::channel(10);
        let state = ConnectionState {
            sender: tx,
            subscriptions: HashSet::new(),
        };

        assert!(state.subscriptions.is_empty());
    }
}
