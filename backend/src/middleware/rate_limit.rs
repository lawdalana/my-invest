//! Rate limiting middleware
//!
//! This module provides rate limiting using the Governor crate
//! to prevent abuse and ensure fair usage of the API.

use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use governor::{
    clock::DefaultClock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter as GovernorRateLimiter,
};
use serde::Serialize;
use std::{
    net::SocketAddr,
    num::NonZeroU32,
    sync::Arc,
    time::Duration,
};
use tower::{Layer, Service};
use tracing::{debug, warn};

use crate::config::RateLimitConfig;

/// Rate limiter wrapper
#[derive(Clone)]
pub struct RateLimiter {
    limiter: Arc<GovernorRateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>,
}

impl RateLimiter {
    /// Create a new rate limiter from configuration
    pub fn new(config: &RateLimitConfig) -> Self {
        let requests = NonZeroU32::new(config.requests).unwrap_or(NonZeroU32::new(100).unwrap());
        let period = Duration::from_secs(config.window_seconds);

        let quota = Quota::with_period(period)
            .expect("Invalid rate limit period")
            .allow_burst(requests);

        let limiter = Arc::new(GovernorRateLimiter::direct(quota));

        Self { limiter }
    }

    /// Check if a request should be allowed
    pub fn check(&self) -> bool {
        self.limiter.check().is_ok()
    }

    /// Get the rate limiter for use in middleware
    pub fn limiter(
        &self,
    ) -> Arc<GovernorRateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>> {
        self.limiter.clone()
    }
}

/// Rate limit error response
#[derive(Debug, Serialize)]
struct RateLimitError {
    error: String,
    status: u16,
    retry_after: Option<u64>,
}

/// Rate limiting layer for Tower middleware stack
#[derive(Clone)]
pub struct RateLimitLayer {
    limiter: Arc<GovernorRateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>,
}

impl RateLimitLayer {
    /// Create a new rate limit layer
    pub fn new(config: &RateLimitConfig) -> Self {
        let rate_limiter = RateLimiter::new(config);
        Self {
            limiter: rate_limiter.limiter(),
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, service: S) -> Self::Service {
        RateLimitService {
            inner: service,
            limiter: self.limiter.clone(),
        }
    }
}

/// Rate limiting service
#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    limiter: Arc<GovernorRateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>,
}

impl<S> Service<Request<Body>> for RateLimitService<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send,
{
    type Response = Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let limiter = self.limiter.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Check rate limit
            match limiter.check() {
                Ok(_) => {
                    debug!("Rate limit check passed");
                    inner.call(request).await
                }
                Err(not_until) => {
                    let retry_after = not_until.wait_time_from(governor::clock::Clock::now(
                        &governor::clock::DefaultClock::default(),
                    ));

                    let ip = request
                        .extensions()
                        .get::<ConnectInfo<SocketAddr>>()
                        .map(|ci| ci.0.to_string())
                        .unwrap_or_else(|| "unknown".to_string());

                    warn!(
                        ip = ip,
                        retry_after_secs = retry_after.as_secs(),
                        "Rate limit exceeded"
                    );

                    let error = RateLimitError {
                        error: "Too many requests. Please slow down.".to_string(),
                        status: 429,
                        retry_after: Some(retry_after.as_secs()),
                    };

                    let response = (
                        StatusCode::TOO_MANY_REQUESTS,
                        [
                            (
                                axum::http::header::RETRY_AFTER,
                                retry_after.as_secs().to_string(),
                            ),
                        ],
                        Json(error),
                    )
                        .into_response();

                    Ok(response)
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_creation() {
        let config = RateLimitConfig {
            requests: 100,
            window_seconds: 60,
        };

        let limiter = RateLimiter::new(&config);

        // First request should pass
        assert!(limiter.check());
    }

    #[test]
    fn test_rate_limiter_burst() {
        let config = RateLimitConfig {
            requests: 5,
            window_seconds: 60,
        };

        let limiter = RateLimiter::new(&config);

        // All burst requests should pass
        for _ in 0..5 {
            assert!(limiter.check());
        }

        // Next request should fail (exceeded burst)
        assert!(!limiter.check());
    }

    #[test]
    fn test_rate_limit_layer_creation() {
        let config = RateLimitConfig {
            requests: 100,
            window_seconds: 60,
        };

        // Should not panic
        let _layer = RateLimitLayer::new(&config);
    }
}
