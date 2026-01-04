//! Middleware for My-Invest Backend
//!
//! This module contains all middleware implementations:
//! - JWT authentication
//! - Rate limiting
//! - CORS configuration

pub mod auth;
pub mod cors;
pub mod rate_limit;

// Re-export middleware types
pub use auth::{AuthUser, RequireAuth};
pub use cors::create_cors_layer;
pub use rate_limit::RateLimiter;
