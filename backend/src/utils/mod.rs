//! Utility modules for My-Invest Backend
//!
//! This module contains common utilities used throughout the application:
//! - Error handling
//! - Logging configuration
//! - Password hashing
//! - JWT token management

pub mod error;
pub mod jwt;
pub mod logging;
pub mod password;

// Re-export commonly used types
pub use error::{AppError, Result};
