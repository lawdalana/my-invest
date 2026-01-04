//! Route definitions for My-Invest Backend
//!
//! This module defines all API routes and builds the main router.

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::api::handlers::{
    // Auth handlers
    confirm_password_reset, get_current_user, login, logout, refresh_token, register,
    request_password_reset,
    // Asset handlers
    get_asset, get_asset_history, refresh_asset, search_assets,
    // Watchlist handlers
    add_asset_to_watchlist, create_watchlist, delete_watchlist, get_watchlist, list_watchlists,
    remove_asset_from_watchlist, update_watchlist,
};
use crate::middleware::{create_cors_layer, RateLimiter};
use crate::services::{AssetService, AuthService, WatchlistService};
use crate::utils::jwt::JwtManager;
use crate::config::Config;

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    pub asset_service: AssetService,
    pub watchlist_service: WatchlistService,
    pub jwt_manager: Arc<JwtManager>,
}

/// Build the main application router
///
/// # Arguments
///
/// * `state` - Application state containing all services
/// * `config` - Application configuration
///
/// # Returns
///
/// * `Router` - Configured Axum router
pub fn build_router(state: AppState, config: &Config) -> Router {
    // Create middleware layers
    let cors_layer = create_cors_layer(&config.cors);
    let rate_limiter = RateLimiter::new(&config.rate_limit);

    // Build auth routes (public)
    let auth_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh_token))
        .route("/reset-password", post(request_password_reset))
        .route("/reset-password/confirm", post(confirm_password_reset))
        .route("/me", get(get_current_user))
        .with_state(state.auth_service.clone());

    // Build asset routes (protected)
    let asset_routes = Router::new()
        .route("/search", get(search_assets))
        .route("/:symbol", get(get_asset))
        .route("/:symbol/history", get(get_asset_history))
        .route("/:symbol/refresh", post(refresh_asset))
        .with_state(state.asset_service.clone());

    // Build watchlist routes (protected)
    let watchlist_routes = Router::new()
        .route("/", get(list_watchlists))
        .route("/", post(create_watchlist))
        .route("/:id", get(get_watchlist))
        .route("/:id", put(update_watchlist))
        .route("/:id", delete(delete_watchlist))
        .route("/:id/assets", post(add_asset_to_watchlist))
        .route("/:id/assets/:symbol", delete(remove_asset_from_watchlist))
        .with_state(state.watchlist_service.clone());

    // Combine all routes under /api/v1
    let api_routes = Router::new()
        .nest("/auth", auth_routes)
        .nest("/assets", asset_routes)
        .nest("/watchlists", watchlist_routes);

    // Build health check route
    let health_route = Router::new()
        .route("/health", get(health_check))
        .route("/", get(root));

    // Combine all routes
    Router::new()
        .nest("/api/v1", api_routes)
        .merge(health_route)
        // Add JWT manager to request extensions for auth middleware
        .layer(axum::Extension(state.jwt_manager))
        // Add tracing layer
        .layer(TraceLayer::new_for_http())
        // Add CORS layer
        .layer(cors_layer)
        // Note: Rate limiting is applied per-route or globally as needed
        // For simplicity in Phase 1, we use a simple check
        .layer(axum::Extension(Arc::new(rate_limiter)))
}

/// Root endpoint
///
/// GET /
async fn root() -> &'static str {
    "My-Invest API v1.0.0"
}

/// Health check endpoint
///
/// GET /health
async fn health_check() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "service": "my-invest-backend"
    }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_health_check_response() {
        // This is tested in integration tests
        // Just a placeholder for now
    }
}
