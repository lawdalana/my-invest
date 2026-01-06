//! Notification sender service
//!
//! This module provides abstractions for sending alert notifications.
//! It includes a trait for different notification backends and implementations
//! for Redis queue-based and logging-based notification sending.

use async_trait::async_trait;
use serde_json;
use tracing::{info, instrument, warn};

use crate::config::database::RedisDb;
use crate::models::alert::AlertNotification;
use crate::utils::error::{AppError, Result};

/// Redis key for the notification queue
const NOTIFICATION_QUEUE_KEY: &str = "alert_notifications";

/// Trait for notification sending backends
#[async_trait]
pub trait NotificationSender: Send + Sync {
    /// Send a notification
    ///
    /// # Arguments
    ///
    /// * `notification` - The alert notification to send
    ///
    /// # Returns
    ///
    /// * `Ok(())` - On successful queuing/sending
    /// * `Err(AppError)` - If sending fails
    async fn send(&self, notification: AlertNotification) -> Result<()>;

    /// Send multiple notifications
    ///
    /// # Arguments
    ///
    /// * `notifications` - List of notifications to send
    ///
    /// # Returns
    ///
    /// * `Ok(())` - On successful queuing/sending
    /// * `Err(AppError)` - If sending fails
    async fn send_batch(&self, notifications: Vec<AlertNotification>) -> Result<()> {
        for notification in notifications {
            self.send(notification).await?;
        }
        Ok(())
    }
}

/// Redis queue-based notification sender
///
/// This implementation pushes notifications to a Redis list where they can be
/// picked up by a separate notification worker process (e.g., for sending emails,
/// push notifications, WebSocket messages, etc.)
#[derive(Clone)]
pub struct QueuedNotificationSender {
    redis: RedisDb,
    queue_key: String,
}

impl QueuedNotificationSender {
    /// Create a new QueuedNotificationSender instance
    pub fn new(redis: RedisDb) -> Self {
        Self {
            redis,
            queue_key: NOTIFICATION_QUEUE_KEY.to_string(),
        }
    }

    /// Create with a custom queue key
    pub fn with_queue_key(redis: RedisDb, queue_key: String) -> Self {
        Self { redis, queue_key }
    }

    /// Get the current queue length
    pub async fn queue_length(&self) -> Result<i64> {
        let mut conn = self.redis.connection();
        let length: i64 = redis::cmd("LLEN")
            .arg(&self.queue_key)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::CacheError(format!("Failed to get queue length: {}", e)))?;
        Ok(length)
    }

    /// Pop a notification from the queue (for worker processes)
    pub async fn pop(&self) -> Result<Option<AlertNotification>> {
        let mut conn = self.redis.connection();
        let result: Option<String> = redis::cmd("LPOP")
            .arg(&self.queue_key)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::CacheError(format!("Failed to pop from queue: {}", e)))?;

        match result {
            Some(json) => {
                let notification: AlertNotification = serde_json::from_str(&json)
                    .map_err(|e| AppError::InternalError(format!("Failed to parse notification: {}", e)))?;
                Ok(Some(notification))
            }
            None => Ok(None),
        }
    }

    /// Pop multiple notifications from the queue
    pub async fn pop_batch(&self, count: usize) -> Result<Vec<AlertNotification>> {
        let mut notifications = Vec::with_capacity(count);
        for _ in 0..count {
            match self.pop().await? {
                Some(notification) => notifications.push(notification),
                None => break,
            }
        }
        Ok(notifications)
    }
}

#[async_trait]
impl NotificationSender for QueuedNotificationSender {
    #[instrument(skip(self, notification), fields(alert_id = %notification.alert_id))]
    async fn send(&self, notification: AlertNotification) -> Result<()> {
        let json = serde_json::to_string(&notification)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize notification: {}", e)))?;

        let mut conn = self.redis.connection();
        redis::cmd("RPUSH")
            .arg(&self.queue_key)
            .arg(&json)
            .query_async::<_, i64>(&mut conn)
            .await
            .map_err(|e| AppError::CacheError(format!("Failed to push to queue: {}", e)))?;

        info!(
            alert_id = %notification.alert_id,
            symbol = %notification.symbol,
            "Notification queued successfully"
        );

        Ok(())
    }
}

/// Logging-based notification sender for development/testing
///
/// This implementation simply logs notifications without actually sending them.
/// Useful for development and testing environments.
#[derive(Clone, Default)]
pub struct LoggingNotificationSender;

impl LoggingNotificationSender {
    /// Create a new LoggingNotificationSender instance
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl NotificationSender for LoggingNotificationSender {
    #[instrument(skip(self, notification), fields(alert_id = %notification.alert_id))]
    async fn send(&self, notification: AlertNotification) -> Result<()> {
        info!(
            alert_id = %notification.alert_id,
            user_id = %notification.user_id,
            symbol = %notification.symbol,
            target_price = notification.target_price,
            current_price = notification.current_price,
            condition = %notification.condition,
            message = %notification.message(),
            "[DEV] Alert notification triggered"
        );

        Ok(())
    }
}

/// Composite notification sender that sends to multiple backends
#[derive(Clone)]
pub struct CompositeNotificationSender {
    senders: Vec<std::sync::Arc<dyn NotificationSender>>,
}

impl CompositeNotificationSender {
    /// Create a new CompositeNotificationSender
    pub fn new(senders: Vec<std::sync::Arc<dyn NotificationSender>>) -> Self {
        Self { senders }
    }

    /// Add a sender
    pub fn add_sender(&mut self, sender: std::sync::Arc<dyn NotificationSender>) {
        self.senders.push(sender);
    }
}

#[async_trait]
impl NotificationSender for CompositeNotificationSender {
    async fn send(&self, notification: AlertNotification) -> Result<()> {
        for sender in &self.senders {
            if let Err(e) = sender.send(notification.clone()).await {
                warn!(error = %e, "Failed to send notification via one of the backends");
            }
        }
        Ok(())
    }
}

/// No-op notification sender (useful for testing)
#[derive(Clone, Default)]
pub struct NoOpNotificationSender;

impl NoOpNotificationSender {
    /// Create a new NoOpNotificationSender
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl NotificationSender for NoOpNotificationSender {
    async fn send(&self, _notification: AlertNotification) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_notification() -> AlertNotification {
        AlertNotification {
            alert_id: "test_alert_id".to_string(),
            user_id: "test_user_id".to_string(),
            symbol: "AAPL".to_string(),
            target_price: 150.0,
            current_price: 151.50,
            condition: "above".to_string(),
            note: Some("Test note".to_string()),
            triggered_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_logging_sender() {
        let sender = LoggingNotificationSender::new();
        let notification = create_test_notification();

        let result = sender.send(notification).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_noop_sender() {
        let sender = NoOpNotificationSender::new();
        let notification = create_test_notification();

        let result = sender.send(notification).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_composite_sender_with_logging() {
        let logging_sender = std::sync::Arc::new(LoggingNotificationSender::new());
        let noop_sender = std::sync::Arc::new(NoOpNotificationSender::new());

        let composite = CompositeNotificationSender::new(vec![logging_sender, noop_sender]);
        let notification = create_test_notification();

        let result = composite.send(notification).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_batch_send() {
        let sender = LoggingNotificationSender::new();
        let notifications = vec![
            create_test_notification(),
            create_test_notification(),
            create_test_notification(),
        ];

        let result = sender.send_batch(notifications).await;
        assert!(result.is_ok());
    }
}
