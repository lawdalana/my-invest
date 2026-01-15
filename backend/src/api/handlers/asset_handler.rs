//! Asset HTTP handlers
//!
//! This module contains handlers for asset endpoints:
//! - GET /api/v1/assets/search?q={query}
//! - GET /api/v1/assets/{symbol}
//! - GET /api/v1/assets/{symbol}/history?timeframe={1D|1W|1M|3M|6M|1Y|5Y|MAX}

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use tracing::{info, instrument};

use crate::middleware::auth::RequireAuth;
use crate::models::asset::{AssetType, HistoryQuery, SearchQuery, SearchResponse, Timeframe};
use crate::services::AssetService;
use crate::utils::error::AppError;

/// Handler for asset search
///
/// GET /api/v1/assets/search?q={query}&type={type}
///
/// Searches for assets by symbol or company name.
/// Optionally filters by asset type (stock, crypto, etf, bond - case insensitive).
/// Returns up to 10 matching results.
#[instrument(skip(asset_service))]
pub async fn search_assets(
    State(asset_service): State<AssetService>,
    RequireAuth(_user): RequireAuth,
    Query(query): Query<SearchQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Parse the optional asset type filter
    let asset_type: Option<AssetType> = match &query.asset_type {
        Some(type_str) => Some(type_str.parse().map_err(|e: String| {
            AppError::ValidationError(e)
        })?),
        None => None,
    };

    info!(
        query = %query.q,
        asset_type = ?asset_type,
        "Searching for assets"
    );

    if query.q.is_empty() {
        return Err(AppError::ValidationError(
            "Search query cannot be empty".to_string(),
        ));
    }

    let results = asset_service.search(&query.q, asset_type).await?;

    // Limit to 10 results
    let results: Vec<_> = results.into_iter().take(10).collect();
    let total = results.len();

    let response = SearchResponse {
        query: query.q,
        results,
        total,
    };

    Ok(Json(response))
}

/// Handler for getting current asset price
///
/// GET /api/v1/assets/{symbol}
///
/// Returns current price and 24h change for a stock.
#[instrument(skip(asset_service))]
pub async fn get_asset(
    State(asset_service): State<AssetService>,
    RequireAuth(_user): RequireAuth,
    Path(symbol): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    info!(symbol = %symbol, "Fetching asset details");

    // Validate symbol
    if symbol.is_empty() || symbol.len() > 10 {
        return Err(AppError::ValidationError(
            "Symbol must be between 1 and 10 characters".to_string(),
        ));
    }

    // Validate symbol contains only alphanumeric characters
    if !symbol.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-') {
        return Err(AppError::ValidationError(
            "Symbol contains invalid characters".to_string(),
        ));
    }

    let asset = asset_service.get_quote(&symbol).await?;

    Ok(Json(asset))
}

/// Handler for getting historical price data
///
/// GET /api/v1/assets/{symbol}/history?timeframe={1D|1W|1M|3M|6M|1Y|5Y|MAX}
///
/// Returns historical OHLCV data for a stock.
/// Supported timeframes:
/// - 1D: 1 day
/// - 1W: 1 week
/// - 1M: 1 month
/// - 3M: 3 months
/// - 6M: 6 months
/// - 1Y: 1 year
/// - 5Y: 5 years
/// - MAX: All available history
#[instrument(skip(asset_service))]
pub async fn get_asset_history(
    State(asset_service): State<AssetService>,
    RequireAuth(_user): RequireAuth,
    Path(symbol): Path<String>,
    Query(query): Query<HistoryQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Parse timeframe (default to 1D)
    let timeframe_str = query.timeframe.as_deref().unwrap_or("1D");
    let timeframe: Timeframe = timeframe_str.parse().map_err(|e: String| {
        AppError::ValidationError(e)
    })?;

    info!(
        symbol = %symbol,
        timeframe = timeframe_str,
        "Fetching asset history"
    );

    // Validate symbol
    if symbol.is_empty() || symbol.len() > 10 {
        return Err(AppError::ValidationError(
            "Symbol must be between 1 and 10 characters".to_string(),
        ));
    }

    let history = asset_service.get_history(&symbol, timeframe).await?;

    Ok(Json(history))
}

/// Handler for refreshing asset data (force cache invalidation)
///
/// POST /api/v1/assets/{symbol}/refresh
///
/// Forces a cache refresh for the given symbol.
#[instrument(skip(asset_service))]
pub async fn refresh_asset(
    State(asset_service): State<AssetService>,
    RequireAuth(_user): RequireAuth,
    Path(symbol): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    info!(symbol = %symbol, "Refreshing asset data");

    // Validate symbol
    if symbol.is_empty() || symbol.len() > 10 {
        return Err(AppError::ValidationError(
            "Symbol must be between 1 and 10 characters".to_string(),
        ));
    }

    // Invalidate cache
    asset_service.invalidate_cache(&symbol).await?;

    // Fetch fresh data
    let asset = asset_service.get_quote(&symbol).await?;

    Ok(Json(asset))
}

#[cfg(test)]
mod tests {
    // Handler tests are in the integration test suite
    // See tests/integration/api/asset_test.rs
}
