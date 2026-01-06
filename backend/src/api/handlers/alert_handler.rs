//! Alert HTTP handlers
//!
//! This module contains handlers for alert endpoints:
//! - GET /api/v1/alerts
//! - POST /api/v1/alerts
//! - GET /api/v1/alerts/{id}
//! - PUT /api/v1/alerts/{id}
//! - DELETE /api/v1/alerts/{id}
//! - PATCH /api/v1/alerts/{id}/toggle

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::bson::oid::ObjectId;
use tracing::{info, instrument};
use validator::Validate;

use crate::middleware::auth::RequireAuth;
use crate::models::alert::{CreateAlertRequest, UpdateAlertRequest};
use crate::models::user::MessageResponse;
use crate::services::AlertService;
use crate::utils::error::AppError;

/// Handler for listing all alerts
///
/// GET /api/v1/alerts
///
/// Returns all alerts for the authenticated user.
#[instrument(skip(alert_service))]
pub async fn list_alerts(
    State(alert_service): State<AlertService>,
    RequireAuth(user): RequireAuth,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %user.id, "Listing all alerts");

    let response = alert_service.get_all(&user.id).await?;

    Ok(Json(response))
}

/// Handler for creating an alert
///
/// POST /api/v1/alerts
///
/// Creates a new price alert for the authenticated user.
#[instrument(skip(alert_service, request))]
pub async fn create_alert(
    State(alert_service): State<AlertService>,
    RequireAuth(user): RequireAuth,
    Json(request): Json<CreateAlertRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    request.validate()?;

    info!(
        user_id = %user.id,
        symbol = %request.symbol,
        target_price = request.target_price,
        condition = ?request.condition,
        "Creating alert"
    );

    let response = alert_service.create(&user.id, request).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

/// Handler for getting a specific alert
///
/// GET /api/v1/alerts/{id}
///
/// Returns a specific alert.
#[instrument(skip(alert_service))]
pub async fn get_alert(
    State(alert_service): State<AlertService>,
    RequireAuth(user): RequireAuth,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %user.id, alert_id = %id, "Getting alert");

    let object_id = parse_object_id(&id)?;
    let response = alert_service.get_by_id(&object_id, &user.id).await?;

    Ok(Json(response))
}

/// Handler for updating an alert
///
/// PUT /api/v1/alerts/{id}
///
/// Updates an alert's properties.
#[instrument(skip(alert_service, request))]
pub async fn update_alert(
    State(alert_service): State<AlertService>,
    RequireAuth(user): RequireAuth,
    Path(id): Path<String>,
    Json(request): Json<UpdateAlertRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    request.validate()?;

    info!(
        user_id = %user.id,
        alert_id = %id,
        "Updating alert"
    );

    let object_id = parse_object_id(&id)?;
    let response = alert_service
        .update(&object_id, &user.id, request)
        .await?;

    Ok(Json(response))
}

/// Handler for deleting an alert
///
/// DELETE /api/v1/alerts/{id}
///
/// Deletes an alert.
#[instrument(skip(alert_service))]
pub async fn delete_alert(
    State(alert_service): State<AlertService>,
    RequireAuth(user): RequireAuth,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %user.id, alert_id = %id, "Deleting alert");

    let object_id = parse_object_id(&id)?;
    alert_service.delete(&object_id, &user.id).await?;

    Ok(Json(MessageResponse {
        message: "Alert deleted successfully".to_string(),
    }))
}

/// Handler for toggling an alert's status
///
/// PATCH /api/v1/alerts/{id}/toggle
///
/// Toggles an alert between Active and Disabled status.
#[instrument(skip(alert_service))]
pub async fn toggle_alert(
    State(alert_service): State<AlertService>,
    RequireAuth(user): RequireAuth,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %user.id, alert_id = %id, "Toggling alert status");

    let object_id = parse_object_id(&id)?;
    let response = alert_service.toggle(&object_id, &user.id).await?;

    Ok(Json(response))
}

/// Parse a string as an ObjectId
fn parse_object_id(id: &str) -> Result<ObjectId, AppError> {
    ObjectId::parse_str(id).map_err(|_| {
        AppError::ValidationError(format!("Invalid ID format: {}", id))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_object_id_valid() {
        let id = ObjectId::new();
        let result = parse_object_id(&id.to_hex());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), id);
    }

    #[test]
    fn test_parse_object_id_invalid() {
        let result = parse_object_id("not-a-valid-id");
        assert!(result.is_err());
    }
}
