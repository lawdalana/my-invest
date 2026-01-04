//! CORS (Cross-Origin Resource Sharing) configuration
//!
//! This module provides CORS configuration for the API,
//! allowing the frontend to communicate with the backend.

use axum::http::{header, HeaderValue, Method};
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::config::CorsConfig;

/// Create a CORS layer from configuration
///
/// # Arguments
///
/// * `config` - CORS configuration containing allowed origins
///
/// # Returns
///
/// * `CorsLayer` - Configured CORS middleware layer
pub fn create_cors_layer(config: &CorsConfig) -> CorsLayer {
    let origins: Vec<HeaderValue> = config
        .allowed_origins
        .iter()
        .filter_map(|origin| {
            origin.parse::<HeaderValue>().ok().or_else(|| {
                tracing::warn!(origin = origin, "Invalid CORS origin, skipping");
                None
            })
        })
        .collect();

    info!(
        origins = ?config.allowed_origins,
        "Configuring CORS with allowed origins"
    );

    CorsLayer::new()
        // Allow specific origins
        .allow_origin(origins)
        // Allow common HTTP methods
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        // Allow common headers
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::ORIGIN,
        ])
        // Allow credentials (cookies, authorization headers)
        .allow_credentials(true)
        // Cache preflight responses for 1 hour
        .max_age(std::time::Duration::from_secs(3600))
}

/// Create a permissive CORS layer for development
///
/// This allows all origins and is suitable only for development.
#[allow(dead_code)]
pub fn create_permissive_cors_layer() -> CorsLayer {
    info!("Using permissive CORS configuration (development only)");

    CorsLayer::very_permissive()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cors_layer() {
        let config = CorsConfig {
            allowed_origins: vec![
                "http://localhost:3000".to_string(),
                "http://localhost:5173".to_string(),
            ],
        };

        // This should not panic
        let _layer = create_cors_layer(&config);
    }

    #[test]
    fn test_create_cors_layer_with_invalid_origin() {
        let config = CorsConfig {
            allowed_origins: vec![
                "http://localhost:3000".to_string(),
                "not a valid url".to_string(), // This should be skipped
            ],
        };

        // This should not panic even with invalid origins
        let _layer = create_cors_layer(&config);
    }

    #[test]
    fn test_permissive_cors() {
        // This should not panic
        let _layer = create_permissive_cors_layer();
    }
}
