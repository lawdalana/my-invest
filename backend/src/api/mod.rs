//! API module - contains all HTTP handlers and route configurations
//!
//! This module includes:
//! - HTTP handlers for all endpoints (handlers/)
//! - Route definitions and router building (routes/)

pub mod handlers;
pub mod routes;

// Re-export route building
pub use routes::{build_router, AppState};
