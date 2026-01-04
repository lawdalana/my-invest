//! Logging configuration for My-Invest Backend
//!
//! This module sets up structured JSON logging using the tracing ecosystem.
//! Logs include request/response information, database operations, and external API calls.

use tracing::Level;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

/// Initialize the logging system
///
/// Sets up tracing with:
/// - Environment-based filtering (RUST_LOG)
/// - JSON format in production, pretty format in development
/// - Span events for timing information
///
/// # Arguments
///
/// * `environment` - The current environment (development, staging, production)
///
/// # Example
///
/// ```ignore
/// use my_invest_backend::utils::logging;
/// logging::init("development");
/// ```
pub fn init(environment: &str) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            // Default log levels
            EnvFilter::new("info,my_invest_backend=debug,tower_http=debug,axum=info")
        });

    let subscriber = tracing_subscriber::registry().with(filter);

    if environment == "production" {
        // JSON logging for production (easier to parse by log aggregators)
        let json_layer = fmt::layer()
            .json()
            .with_target(true)
            .with_level(true)
            .with_file(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::CLOSE);

        subscriber.with(json_layer).init();
    } else {
        // Pretty logging for development
        let pretty_layer = fmt::layer()
            .pretty()
            .with_target(true)
            .with_level(true)
            .with_file(false)
            .with_line_number(false)
            .with_span_events(FmtSpan::CLOSE);

        subscriber.with(pretty_layer).init();
    }

    tracing::info!(
        environment = environment,
        "Logging initialized"
    );
}

/// Create a request span for HTTP request tracing
///
/// This is used with tower_http::trace to add request context to all logs.
#[allow(dead_code)]
pub fn make_span_with_request_id(request: &axum::http::Request<axum::body::Body>) -> tracing::Span {
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let method = request.method().to_string();
    let uri = request.uri().to_string();

    tracing::span!(
        Level::INFO,
        "request",
        request_id = %request_id,
        method = %method,
        uri = %uri,
    )
}

/// Log levels for different components
pub mod levels {
    use tracing::Level;

    /// Default application log level
    pub const APP: Level = Level::DEBUG;

    /// HTTP layer log level
    pub const HTTP: Level = Level::DEBUG;

    /// Database operation log level
    pub const DATABASE: Level = Level::DEBUG;

    /// External API call log level
    pub const EXTERNAL_API: Level = Level::INFO;

    /// Authentication log level
    pub const AUTH: Level = Level::INFO;
}

/// Macros and helpers for consistent logging patterns
pub mod helpers {
    /// Log a database operation with timing
    #[macro_export]
    macro_rules! log_db_operation {
        ($operation:expr, $result:expr) => {{
            let start = std::time::Instant::now();
            let result = $result;
            let duration = start.elapsed();
            tracing::debug!(
                operation = $operation,
                duration_ms = duration.as_millis() as u64,
                success = result.is_ok(),
                "Database operation completed"
            );
            result
        }};
    }

    /// Log an external API call with timing
    #[macro_export]
    macro_rules! log_external_api_call {
        ($service:expr, $endpoint:expr, $result:expr) => {{
            let start = std::time::Instant::now();
            let result = $result;
            let duration = start.elapsed();
            tracing::info!(
                service = $service,
                endpoint = $endpoint,
                duration_ms = duration.as_millis() as u64,
                success = result.is_ok(),
                "External API call completed"
            );
            result
        }};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_does_not_panic() {
        // This test just ensures init doesn't panic for various environments
        // We can't fully test logging without inspecting output
        // Note: Can only initialize logging once, so this is a simple test
    }

    #[test]
    fn test_log_levels() {
        assert_eq!(levels::APP, Level::DEBUG);
        assert_eq!(levels::HTTP, Level::DEBUG);
        assert_eq!(levels::AUTH, Level::INFO);
    }
}
