//! Middleware module - contains custom Axum middleware
//!
//! This module includes:
//! - Authentication middleware (auth.rs)
//! - Rate limiting middleware (rate_limit.rs)
//! - CORS configuration (cors.rs)

pub mod auth;
pub mod cors;
pub mod rate_limit;

// Re-export commonly used types
pub use auth::{AuthUser, OptionalAuth, RequireAuth};
pub use cors::create_cors_layer;
pub use rate_limit::{RateLimitLayer, RateLimiter};
