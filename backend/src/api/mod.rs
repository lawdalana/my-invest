//! API layer for My-Invest Backend
//!
//! This module contains HTTP handlers and route definitions
//! for all API endpoints.

pub mod handlers;
pub mod routes;

// Re-export the router builder
pub use routes::build_router;
