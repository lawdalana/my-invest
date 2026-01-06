//! Utilities module - contains helper functions and utilities
//!
//! This module includes:
//! - Error handling (error.rs)
//! - JWT utilities (jwt.rs)
//! - Password hashing (password.rs)
//! - Logging configuration (logging.rs)

pub mod error;
pub mod jwt;
pub mod logging;
pub mod password;

// Re-export commonly used types
pub use error::{AppError, Result};
pub use jwt::{extract_bearer_token, Claims, JwtManager, TokenPair, TokenType};
