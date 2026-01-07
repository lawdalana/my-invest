//! Alert model and related DTOs
//!
//! This module defines alert-related structures for price monitoring,
//! including alert conditions, types, statuses, and notification DTOs.

use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Alert condition type - determines when an alert triggers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertCondition {
    /// Trigger when price goes above target
    Above,
    /// Trigger when price goes below target
    Below,
    /// Trigger when price crosses target (either direction)
    Crosses,
}

impl AlertCondition {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertCondition::Above => "above",
            AlertCondition::Below => "below",
            AlertCondition::Crosses => "crosses",
        }
    }
}

impl std::fmt::Display for AlertCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Alert type - determines behavior after triggering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AlertType {
    /// Trigger once, then disable
    #[default]
    OneTime,
    /// Can trigger multiple times with cooldown
    Recurring,
}

impl AlertType {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertType::OneTime => "one_time",
            AlertType::Recurring => "recurring",
        }
    }
}

impl std::fmt::Display for AlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Alert status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AlertStatus {
    /// Alert is active and being monitored
    #[default]
    Active,
    /// Alert is disabled by user
    Disabled,
    /// Alert has been triggered (for one-time alerts)
    Triggered,
}

impl AlertStatus {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertStatus::Active => "active",
            AlertStatus::Disabled => "disabled",
            AlertStatus::Triggered => "triggered",
        }
    }
}

impl std::fmt::Display for AlertStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Alert entity stored in MongoDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Unique identifier
    #[serde(rename = "_id")]
    pub id: ObjectId,

    /// Owner's user ID
    pub user_id: ObjectId,

    /// Stock ticker symbol (e.g., "AAPL")
    pub symbol: String,

    /// Target price that triggers the alert
    pub target_price: f64,

    /// Condition for triggering (above, below, crosses)
    pub condition: AlertCondition,

    /// Alert type (one-time or recurring)
    pub alert_type: AlertType,

    /// Current status
    pub status: AlertStatus,

    /// Last price checked for this alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_checked_price: Option<f64>,

    /// Last time the alert was triggered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_triggered_at: Option<DateTime<Utc>>,

    /// Number of times this alert has been triggered
    #[serde(default)]
    pub trigger_count: u32,

    /// Optional user note for the alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    /// Alert creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Alert {
    /// Create a new Alert
    pub fn new(
        user_id: ObjectId,
        symbol: String,
        target_price: f64,
        condition: AlertCondition,
        alert_type: AlertType,
        note: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: ObjectId::new(),
            user_id,
            symbol: symbol.to_uppercase(),
            target_price,
            condition,
            alert_type,
            status: AlertStatus::Active,
            last_checked_price: None,
            last_triggered_at: None,
            trigger_count: 0,
            note,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if this alert should be processed
    pub fn should_process(&self) -> bool {
        self.status == AlertStatus::Active
    }

    /// Check if the alert condition is met
    ///
    /// # Arguments
    ///
    /// * `current_price` - The current price of the asset
    /// * `previous_price` - The previous price (for crosses condition)
    ///
    /// # Returns
    ///
    /// * `true` if the alert should trigger
    pub fn check_condition(&self, current_price: f64, previous_price: Option<f64>) -> bool {
        match self.condition {
            AlertCondition::Above => current_price >= self.target_price,
            AlertCondition::Below => current_price <= self.target_price,
            AlertCondition::Crosses => {
                if let Some(prev) = previous_price {
                    // Check if price crossed the target in either direction
                    let was_above = prev > self.target_price;
                    let is_above = current_price > self.target_price;
                    let was_below = prev < self.target_price;
                    let is_below = current_price < self.target_price;

                    (was_above && is_below) || (was_below && is_above)
                } else {
                    // No previous price, can't determine crossing
                    false
                }
            }
        }
    }

    /// Mark the alert as triggered
    pub fn mark_triggered(&mut self) {
        self.trigger_count += 1;
        self.last_triggered_at = Some(Utc::now());
        self.updated_at = Utc::now();

        if self.alert_type == AlertType::OneTime {
            self.status = AlertStatus::Triggered;
        }
    }

    /// Update the last checked price
    pub fn update_last_checked(&mut self, price: f64) {
        self.last_checked_price = Some(price);
        self.updated_at = Utc::now();
    }

    /// Check if the alert is on cooldown (for recurring alerts)
    pub fn is_on_cooldown(&self, cooldown_seconds: i64) -> bool {
        if self.alert_type != AlertType::Recurring {
            return false;
        }

        if let Some(last_triggered) = self.last_triggered_at {
            let elapsed = Utc::now().signed_duration_since(last_triggered);
            return elapsed.num_seconds() < cooldown_seconds;
        }

        false
    }
}

/// Request DTO for creating an alert
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateAlertRequest {
    /// Stock ticker symbol
    #[validate(length(
        min = 1,
        max = 10,
        message = "Symbol must be between 1 and 10 characters"
    ))]
    pub symbol: String,

    /// Target price for the alert
    #[validate(range(min = 0.0001, message = "Target price must be greater than 0"))]
    pub target_price: f64,

    /// Alert condition (above, below, crosses)
    pub condition: AlertCondition,

    /// Alert type (one_time, recurring) - defaults to one_time
    #[serde(default)]
    pub alert_type: AlertType,

    /// Optional note for the alert
    #[validate(length(max = 500, message = "Note must be at most 500 characters"))]
    pub note: Option<String>,
}

/// Request DTO for updating an alert
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateAlertRequest {
    /// Updated target price
    #[validate(range(min = 0.0001, message = "Target price must be greater than 0"))]
    pub target_price: Option<f64>,

    /// Updated condition
    pub condition: Option<AlertCondition>,

    /// Updated alert type
    pub alert_type: Option<AlertType>,

    /// Updated note
    #[validate(length(max = 500, message = "Note must be at most 500 characters"))]
    pub note: Option<String>,
}

/// Response DTO for an alert
#[derive(Debug, Clone, Serialize)]
pub struct AlertResponse {
    /// Alert ID
    pub id: String,

    /// Stock symbol
    pub symbol: String,

    /// Target price
    pub target_price: f64,

    /// Alert condition
    pub condition: String,

    /// Alert type
    pub alert_type: String,

    /// Alert status
    pub status: String,

    /// Last checked price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_checked_price: Option<f64>,

    /// Last triggered timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_triggered_at: Option<DateTime<Utc>>,

    /// Number of times triggered
    pub trigger_count: u32,

    /// User note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl From<Alert> for AlertResponse {
    fn from(alert: Alert) -> Self {
        Self {
            id: alert.id.to_hex(),
            symbol: alert.symbol,
            target_price: alert.target_price,
            condition: alert.condition.to_string(),
            alert_type: alert.alert_type.to_string(),
            status: alert.status.to_string(),
            last_checked_price: alert.last_checked_price,
            last_triggered_at: alert.last_triggered_at,
            trigger_count: alert.trigger_count,
            note: alert.note,
            created_at: alert.created_at,
            updated_at: alert.updated_at,
        }
    }
}

/// Response DTO for a list of alerts
#[derive(Debug, Clone, Serialize)]
pub struct AlertListResponse {
    /// List of alerts
    pub alerts: Vec<AlertResponse>,

    /// Total count
    pub total: usize,
}

/// Notification DTO sent when an alert triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertNotification {
    /// Alert ID
    pub alert_id: String,

    /// User ID
    pub user_id: String,

    /// Stock symbol
    pub symbol: String,

    /// Target price that was set
    pub target_price: f64,

    /// Current price that triggered the alert
    pub current_price: f64,

    /// Alert condition
    pub condition: String,

    /// User note (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    /// Timestamp when the alert was triggered
    pub triggered_at: DateTime<Utc>,
}

impl AlertNotification {
    /// Create a new AlertNotification
    pub fn new(alert: &Alert, current_price: f64) -> Self {
        Self {
            alert_id: alert.id.to_hex(),
            user_id: alert.user_id.to_hex(),
            symbol: alert.symbol.clone(),
            target_price: alert.target_price,
            current_price,
            condition: alert.condition.to_string(),
            note: alert.note.clone(),
            triggered_at: Utc::now(),
        }
    }

    /// Get a human-readable notification message
    pub fn message(&self) -> String {
        let direction = match self.condition.as_str() {
            "above" => "reached or exceeded",
            "below" => "dropped to or below",
            "crosses" => "crossed",
            _ => "reached",
        };

        format!(
            "{} {} your target price of ${:.2} (current: ${:.2})",
            self.symbol, direction, self.target_price, self.current_price
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_new() {
        let user_id = ObjectId::new();
        let alert = Alert::new(
            user_id,
            "aapl".to_string(),
            150.0,
            AlertCondition::Above,
            AlertType::OneTime,
            Some("Buy signal".to_string()),
        );

        assert_eq!(alert.symbol, "AAPL"); // Should be uppercase
        assert_eq!(alert.target_price, 150.0);
        assert_eq!(alert.condition, AlertCondition::Above);
        assert_eq!(alert.alert_type, AlertType::OneTime);
        assert_eq!(alert.status, AlertStatus::Active);
        assert_eq!(alert.trigger_count, 0);
        assert!(alert.note.is_some());
    }

    #[test]
    fn test_alert_check_condition_above() {
        let alert = Alert::new(
            ObjectId::new(),
            "AAPL".to_string(),
            150.0,
            AlertCondition::Above,
            AlertType::OneTime,
            None,
        );

        assert!(!alert.check_condition(149.99, None));
        assert!(alert.check_condition(150.0, None));
        assert!(alert.check_condition(150.01, None));
    }

    #[test]
    fn test_alert_check_condition_below() {
        let alert = Alert::new(
            ObjectId::new(),
            "AAPL".to_string(),
            150.0,
            AlertCondition::Below,
            AlertType::OneTime,
            None,
        );

        assert!(!alert.check_condition(150.01, None));
        assert!(alert.check_condition(150.0, None));
        assert!(alert.check_condition(149.99, None));
    }

    #[test]
    fn test_alert_check_condition_crosses() {
        let alert = Alert::new(
            ObjectId::new(),
            "AAPL".to_string(),
            150.0,
            AlertCondition::Crosses,
            AlertType::OneTime,
            None,
        );

        // No previous price - shouldn't trigger
        assert!(!alert.check_condition(149.0, None));

        // Crossed from below to above
        assert!(alert.check_condition(151.0, Some(149.0)));

        // Crossed from above to below
        assert!(alert.check_condition(149.0, Some(151.0)));

        // Didn't cross (stayed above)
        assert!(!alert.check_condition(152.0, Some(151.0)));

        // Didn't cross (stayed below)
        assert!(!alert.check_condition(148.0, Some(149.0)));
    }

    #[test]
    fn test_alert_mark_triggered_one_time() {
        let mut alert = Alert::new(
            ObjectId::new(),
            "AAPL".to_string(),
            150.0,
            AlertCondition::Above,
            AlertType::OneTime,
            None,
        );

        alert.mark_triggered();

        assert_eq!(alert.trigger_count, 1);
        assert!(alert.last_triggered_at.is_some());
        assert_eq!(alert.status, AlertStatus::Triggered);
    }

    #[test]
    fn test_alert_mark_triggered_recurring() {
        let mut alert = Alert::new(
            ObjectId::new(),
            "AAPL".to_string(),
            150.0,
            AlertCondition::Above,
            AlertType::Recurring,
            None,
        );

        alert.mark_triggered();

        assert_eq!(alert.trigger_count, 1);
        assert!(alert.last_triggered_at.is_some());
        assert_eq!(alert.status, AlertStatus::Active); // Should stay active
    }

    #[test]
    fn test_alert_response_from() {
        let alert = Alert::new(
            ObjectId::new(),
            "AAPL".to_string(),
            150.0,
            AlertCondition::Above,
            AlertType::OneTime,
            Some("Test note".to_string()),
        );

        let response: AlertResponse = alert.clone().into();

        assert_eq!(response.symbol, "AAPL");
        assert_eq!(response.target_price, 150.0);
        assert_eq!(response.condition, "above");
        assert_eq!(response.alert_type, "one_time");
        assert_eq!(response.status, "active");
        assert_eq!(response.id, alert.id.to_hex());
    }

    #[test]
    fn test_alert_notification_message() {
        let alert = Alert::new(
            ObjectId::new(),
            "AAPL".to_string(),
            150.0,
            AlertCondition::Above,
            AlertType::OneTime,
            None,
        );

        let notification = AlertNotification::new(&alert, 152.50);

        assert!(notification.message().contains("AAPL"));
        assert!(notification.message().contains("150.00"));
        assert!(notification.message().contains("152.50"));
        assert!(notification.message().contains("reached or exceeded"));
    }

    #[test]
    fn test_alert_condition_display() {
        assert_eq!(AlertCondition::Above.to_string(), "above");
        assert_eq!(AlertCondition::Below.to_string(), "below");
        assert_eq!(AlertCondition::Crosses.to_string(), "crosses");
    }

    #[test]
    fn test_alert_type_display() {
        assert_eq!(AlertType::OneTime.to_string(), "one_time");
        assert_eq!(AlertType::Recurring.to_string(), "recurring");
    }

    #[test]
    fn test_alert_status_display() {
        assert_eq!(AlertStatus::Active.to_string(), "active");
        assert_eq!(AlertStatus::Disabled.to_string(), "disabled");
        assert_eq!(AlertStatus::Triggered.to_string(), "triggered");
    }

    #[test]
    fn test_alert_should_process() {
        let mut alert = Alert::new(
            ObjectId::new(),
            "AAPL".to_string(),
            150.0,
            AlertCondition::Above,
            AlertType::OneTime,
            None,
        );

        assert!(alert.should_process());

        alert.status = AlertStatus::Disabled;
        assert!(!alert.should_process());

        alert.status = AlertStatus::Triggered;
        assert!(!alert.should_process());
    }
}
