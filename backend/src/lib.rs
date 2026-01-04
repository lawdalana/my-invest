//! My-Invest Backend Library
//!
//! This crate provides the backend API server for the My-Invest Dashboard,
//! a stock market tracking and analysis application.
//!
//! ## Features
//!
//! - User authentication with JWT
//! - Stock search and price quotes (via Alpha Vantage)
//! - Historical price data with multiple timeframes
//! - Watchlist management
//!
//! ## Architecture
//!
//! The crate is organized into the following modules:
//!
//! - `api` - HTTP handlers and route definitions
//! - `config` - Configuration management and database connections
//! - `middleware` - Authentication, CORS, and rate limiting
//! - `models` - Data models and DTOs
//! - `services` - Business logic services
//! - `utils` - Utilities for error handling, JWT, password hashing, and logging

pub mod api;
pub mod config;
pub mod middleware;
pub mod models;
pub mod services;
pub mod utils;

// Re-export commonly used types
pub use api::routes::{build_router, AppState};
pub use config::Config;
pub use utils::error::{AppError, Result};
