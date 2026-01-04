//! Watchlist HTTP handlers
//!
//! This module contains handlers for watchlist endpoints:
//! - GET /api/v1/watchlists
//! - POST /api/v1/watchlists
//! - GET /api/v1/watchlists/{id}
//! - PUT /api/v1/watchlists/{id}
//! - DELETE /api/v1/watchlists/{id}
//! - POST /api/v1/watchlists/{id}/assets
//! - DELETE /api/v1/watchlists/{id}/assets/{symbol}

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
use crate::models::watchlist::{
    AddAssetRequest, CreateWatchlistRequest, UpdateWatchlistRequest,
};
use crate::models::user::MessageResponse;
use crate::services::WatchlistService;
use crate::utils::error::AppError;

/// Handler for listing all watchlists
///
/// GET /api/v1/watchlists
///
/// Returns all watchlists for the authenticated user.
#[instrument(skip(watchlist_service))]
pub async fn list_watchlists(
    State(watchlist_service): State<WatchlistService>,
    RequireAuth(user): RequireAuth,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %user.id, "Listing all watchlists");

    let response = watchlist_service.get_all(&user.id).await?;

    Ok(Json(response))
}

/// Handler for creating a watchlist
///
/// POST /api/v1/watchlists
///
/// Creates a new watchlist for the authenticated user.
#[instrument(skip(watchlist_service, request))]
pub async fn create_watchlist(
    State(watchlist_service): State<WatchlistService>,
    RequireAuth(user): RequireAuth,
    Json(request): Json<CreateWatchlistRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    request.validate()?;

    info!(user_id = %user.id, name = %request.name, "Creating watchlist");

    let response = watchlist_service.create(&user.id, request).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

/// Handler for getting a specific watchlist
///
/// GET /api/v1/watchlists/{id}
///
/// Returns a specific watchlist with all its assets.
#[instrument(skip(watchlist_service))]
pub async fn get_watchlist(
    State(watchlist_service): State<WatchlistService>,
    RequireAuth(user): RequireAuth,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %user.id, watchlist_id = %id, "Getting watchlist");

    let object_id = parse_object_id(&id)?;
    let response = watchlist_service.get_by_id(&object_id, &user.id).await?;

    Ok(Json(response))
}

/// Handler for updating a watchlist
///
/// PUT /api/v1/watchlists/{id}
///
/// Updates the name of a watchlist.
#[instrument(skip(watchlist_service, request))]
pub async fn update_watchlist(
    State(watchlist_service): State<WatchlistService>,
    RequireAuth(user): RequireAuth,
    Path(id): Path<String>,
    Json(request): Json<UpdateWatchlistRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    request.validate()?;

    info!(
        user_id = %user.id,
        watchlist_id = %id,
        new_name = %request.name,
        "Updating watchlist"
    );

    let object_id = parse_object_id(&id)?;
    let response = watchlist_service
        .update(&object_id, &user.id, request)
        .await?;

    Ok(Json(response))
}

/// Handler for deleting a watchlist
///
/// DELETE /api/v1/watchlists/{id}
///
/// Deletes a watchlist and all its assets.
#[instrument(skip(watchlist_service))]
pub async fn delete_watchlist(
    State(watchlist_service): State<WatchlistService>,
    RequireAuth(user): RequireAuth,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %user.id, watchlist_id = %id, "Deleting watchlist");

    let object_id = parse_object_id(&id)?;
    watchlist_service.delete(&object_id, &user.id).await?;

    Ok(Json(MessageResponse {
        message: "Watchlist deleted successfully".to_string(),
    }))
}

/// Handler for adding an asset to a watchlist
///
/// POST /api/v1/watchlists/{id}/assets
///
/// Adds a stock to the watchlist after verifying it exists.
#[instrument(skip(watchlist_service, request))]
pub async fn add_asset_to_watchlist(
    State(watchlist_service): State<WatchlistService>,
    RequireAuth(user): RequireAuth,
    Path(id): Path<String>,
    Json(request): Json<AddAssetRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    request.validate()?;

    info!(
        user_id = %user.id,
        watchlist_id = %id,
        symbol = %request.symbol,
        "Adding asset to watchlist"
    );

    let object_id = parse_object_id(&id)?;
    let response = watchlist_service
        .add_asset(&object_id, &user.id, request)
        .await?;

    Ok((StatusCode::CREATED, Json(response)))
}

/// Handler for removing an asset from a watchlist
///
/// DELETE /api/v1/watchlists/{id}/assets/{symbol}
///
/// Removes a stock from the watchlist.
#[instrument(skip(watchlist_service))]
pub async fn remove_asset_from_watchlist(
    State(watchlist_service): State<WatchlistService>,
    RequireAuth(user): RequireAuth,
    Path((id, symbol)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    info!(
        user_id = %user.id,
        watchlist_id = %id,
        symbol = %symbol,
        "Removing asset from watchlist"
    );

    let object_id = parse_object_id(&id)?;
    let response = watchlist_service
        .remove_asset(&object_id, &user.id, &symbol)
        .await?;

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
