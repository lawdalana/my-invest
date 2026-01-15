//! WebSocket message types for real-time price updates
//!
//! This module defines the message formats for WebSocket communication:
//! - Client messages: Subscribe/Unsubscribe to symbols
//! - Server messages: Price updates, errors, confirmations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Client-to-server WebSocket message
///
/// Clients send these messages to subscribe/unsubscribe from price updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum WsClientMessage {
    /// Subscribe to price updates for specified symbols
    Subscribe {
        /// List of symbols to subscribe to (e.g., ["AAPL", "BTC-USD"])
        symbols: Vec<String>,
    },

    /// Unsubscribe from price updates for specified symbols
    Unsubscribe {
        /// List of symbols to unsubscribe from
        symbols: Vec<String>,
    },

    /// Ping message for connection health check
    Ping,
}

/// Server-to-client WebSocket message
///
/// Server sends these messages for price updates and notifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsServerMessage {
    /// Real-time price update for a symbol
    PriceUpdate(PriceUpdate),

    /// Subscription confirmation
    Subscribed {
        /// Symbols that were successfully subscribed
        symbols: Vec<String>,
        /// Total number of active subscriptions for this connection
        total_subscriptions: usize,
    },

    /// Unsubscription confirmation
    Unsubscribed {
        /// Symbols that were successfully unsubscribed
        symbols: Vec<String>,
        /// Total number of active subscriptions for this connection
        total_subscriptions: usize,
    },

    /// Pong response to ping
    Pong {
        /// Server timestamp
        timestamp: DateTime<Utc>,
    },

    /// Error message
    Error(WsError),

    /// Connection established confirmation
    Connected {
        /// Message to client
        message: String,
        /// Server timestamp
        timestamp: DateTime<Utc>,
    },
}

/// Price update payload
///
/// Contains the latest price information for a subscribed symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceUpdate {
    /// Stock/asset symbol (e.g., "AAPL", "BTC-USD")
    pub symbol: String,

    /// Current price
    pub price: f64,

    /// Price change from previous close (absolute value)
    pub change: f64,

    /// Price change percentage
    pub change_percent: f64,

    /// Timestamp of this price update
    pub timestamp: DateTime<Utc>,
}

impl PriceUpdate {
    /// Create a new price update
    #[allow(dead_code)]
    pub fn new(symbol: String, price: f64, change: f64, change_percent: f64) -> Self {
        Self {
            symbol,
            price,
            change,
            change_percent,
            timestamp: Utc::now(),
        }
    }

    /// Create a price update from an Asset
    pub fn from_asset(asset: &crate::models::asset::Asset) -> Self {
        Self {
            symbol: asset.symbol.clone(),
            price: asset.price,
            change: asset.change_24h,
            change_percent: asset.change_percent_24h,
            timestamp: asset.last_updated,
        }
    }
}

/// WebSocket error payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsError {
    /// Error code for programmatic handling
    pub code: WsErrorCode,

    /// Human-readable error message
    pub message: String,

    /// Optional additional details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl WsError {
    /// Create a new WebSocket error
    pub fn new(code: WsErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
        }
    }

    /// Create a new WebSocket error with details
    #[allow(dead_code)]
    pub fn with_details(code: WsErrorCode, message: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: Some(details.into()),
        }
    }
}

/// WebSocket error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WsErrorCode {
    /// Invalid message format
    InvalidMessage,

    /// Invalid symbol
    InvalidSymbol,

    /// Symbol not found
    SymbolNotFound,

    /// Subscription limit exceeded
    SubscriptionLimitExceeded,

    /// Authentication required or failed
    AuthenticationFailed,

    /// Rate limit exceeded
    RateLimitExceeded,

    /// Internal server error
    InternalError,
}

/// Maximum number of symbols a single connection can subscribe to
pub const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = 50;

/// Maximum length for a symbol
#[allow(dead_code)]
pub const MAX_SYMBOL_LENGTH: usize = 20;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_message_subscribe_serialization() {
        let msg = WsClientMessage::Subscribe {
            symbols: vec!["AAPL".to_string(), "BTC-USD".to_string()],
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"action\":\"subscribe\""));
        assert!(json.contains("\"symbols\""));

        // Test deserialization
        let deserialized: WsClientMessage = serde_json::from_str(&json).unwrap();
        match deserialized {
            WsClientMessage::Subscribe { symbols } => {
                assert_eq!(symbols.len(), 2);
                assert_eq!(symbols[0], "AAPL");
                assert_eq!(symbols[1], "BTC-USD");
            }
            _ => panic!("Expected Subscribe message"),
        }
    }

    #[test]
    fn test_client_message_unsubscribe_serialization() {
        let msg = WsClientMessage::Unsubscribe {
            symbols: vec!["AAPL".to_string()],
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"action\":\"unsubscribe\""));

        let deserialized: WsClientMessage = serde_json::from_str(&json).unwrap();
        match deserialized {
            WsClientMessage::Unsubscribe { symbols } => {
                assert_eq!(symbols.len(), 1);
                assert_eq!(symbols[0], "AAPL");
            }
            _ => panic!("Expected Unsubscribe message"),
        }
    }

    #[test]
    fn test_client_message_ping_serialization() {
        let msg = WsClientMessage::Ping;
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"action\":\"ping\""));
    }

    #[test]
    fn test_server_message_price_update_serialization() {
        let update = PriceUpdate::new("AAPL".to_string(), 150.25, 1.5, 1.01);
        let msg = WsServerMessage::PriceUpdate(update);

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"price_update\""));
        assert!(json.contains("\"symbol\":\"AAPL\""));
        assert!(json.contains("\"price\":150.25"));
    }

    #[test]
    fn test_server_message_subscribed_serialization() {
        let msg = WsServerMessage::Subscribed {
            symbols: vec!["AAPL".to_string()],
            total_subscriptions: 1,
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"subscribed\""));
        assert!(json.contains("\"total_subscriptions\":1"));
    }

    #[test]
    fn test_server_message_error_serialization() {
        let error = WsError::new(WsErrorCode::InvalidSymbol, "Symbol not valid");
        let msg = WsServerMessage::Error(error);

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"error\""));
        assert!(json.contains("\"code\":\"invalid_symbol\""));
        assert!(json.contains("\"message\":\"Symbol not valid\""));
    }

    #[test]
    fn test_ws_error_with_details() {
        let error = WsError::with_details(
            WsErrorCode::SubscriptionLimitExceeded,
            "Too many subscriptions",
            "Maximum allowed: 50",
        );

        assert_eq!(error.code, WsErrorCode::SubscriptionLimitExceeded);
        assert_eq!(error.message, "Too many subscriptions");
        assert_eq!(error.details, Some("Maximum allowed: 50".to_string()));
    }

    #[test]
    fn test_price_update_new() {
        let update = PriceUpdate::new("GOOGL".to_string(), 140.50, 2.25, 1.63);

        assert_eq!(update.symbol, "GOOGL");
        assert_eq!(update.price, 140.50);
        assert_eq!(update.change, 2.25);
        assert_eq!(update.change_percent, 1.63);
    }

    #[test]
    fn test_deserialize_subscribe_from_json() {
        let json = r#"{"action": "subscribe", "symbols": ["AAPL", "BTC-USD"]}"#;
        let msg: WsClientMessage = serde_json::from_str(json).unwrap();

        match msg {
            WsClientMessage::Subscribe { symbols } => {
                assert_eq!(symbols, vec!["AAPL", "BTC-USD"]);
            }
            _ => panic!("Expected Subscribe message"),
        }
    }

    #[test]
    fn test_deserialize_unsubscribe_from_json() {
        let json = r#"{"action": "unsubscribe", "symbols": ["AAPL"]}"#;
        let msg: WsClientMessage = serde_json::from_str(json).unwrap();

        match msg {
            WsClientMessage::Unsubscribe { symbols } => {
                assert_eq!(symbols, vec!["AAPL"]);
            }
            _ => panic!("Expected Unsubscribe message"),
        }
    }
}
