//! HTTP handlers for My-Invest Backend
//!
//! This module contains all HTTP request handlers organized by domain.

pub mod alert_handler;
pub mod asset_handler;
pub mod auth_handler;
pub mod watchlist_handler;
pub mod ws_handler;

// Re-export handlers
pub use alert_handler::*;
pub use asset_handler::*;
pub use auth_handler::*;
pub use watchlist_handler::*;
pub use ws_handler::*;
