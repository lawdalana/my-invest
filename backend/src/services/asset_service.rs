//! Asset service with caching
//!
//! This service provides asset data operations with Redis caching to minimize
//! external API calls to Alpha Vantage (which has strict rate limits).
//!
//! Caching strategy:
//! - Search results: 5 minutes TTL
//! - Current quotes: 5 minutes TTL
//! - Historical data: 1 hour TTL

use serde_json;
use tracing::{debug, info, instrument};

use crate::config::database::RedisDb;
use crate::config::RedisConfig;
use crate::models::asset::{Asset, AssetType, PriceHistory, SearchResult, Timeframe};
use crate::services::alpha_vantage_client::AlphaVantageClient;
use crate::utils::error::Result;

/// Cache key prefixes
const CACHE_PREFIX_SEARCH: &str = "search:";
const CACHE_PREFIX_QUOTE: &str = "quote:";
const CACHE_PREFIX_HISTORY: &str = "history:";

/// Asset service with caching layer
#[derive(Clone)]
pub struct AssetService {
    alpha_vantage: AlphaVantageClient,
    redis: RedisDb,
    cache_ttl: u64,
    history_cache_ttl: u64,
}

impl AssetService {
    /// Create a new AssetService instance
    pub fn new(
        alpha_vantage: AlphaVantageClient,
        redis: RedisDb,
        redis_config: &RedisConfig,
    ) -> Self {
        Self {
            alpha_vantage,
            redis,
            cache_ttl: redis_config.cache_ttl,
            history_cache_ttl: redis_config.history_cache_ttl,
        }
    }

    /// Search for assets by keyword with optional type filter
    ///
    /// Results are cached for 5 minutes to reduce API calls.
    /// The cache key includes the asset type filter for proper cache separation.
    ///
    /// # Arguments
    ///
    /// * `query` - Search keyword
    /// * `asset_type` - Optional filter by asset type (Stock, Crypto, Etf, Bond)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<SearchResult>)` - Matching assets
    #[instrument(skip(self))]
    pub async fn search(
        &self,
        query: &str,
        asset_type: Option<AssetType>,
    ) -> Result<Vec<SearchResult>> {
        // Include asset type in cache key for proper separation
        // Use "||type:" delimiter to prevent collisions (e.g., searching for "apple:stock"
        // would otherwise collide with searching for "apple" with type=stock filter)
        let cache_key = match asset_type {
            Some(ref t) => format!("{}{}||type:{}", CACHE_PREFIX_SEARCH, query.to_lowercase(), t.as_str()),
            None => format!("{}{}||type:all", CACHE_PREFIX_SEARCH, query.to_lowercase()),
        };

        // Try cache first
        if let Ok(Some(cached)) = self.redis.get(&cache_key).await {
            debug!(query = query, asset_type = ?asset_type, "Search cache hit");
            if let Ok(results) = serde_json::from_str::<Vec<SearchResult>>(&cached) {
                return Ok(results);
            }
        }

        // Cache miss - fetch from API
        info!(query = query, asset_type = ?asset_type, "Search cache miss, fetching from API");
        let results = self.alpha_vantage.search(query).await?;

        // Filter by asset type if specified
        let filtered_results: Vec<SearchResult> = match asset_type {
            Some(t) => results.into_iter().filter(|r| r.asset_type == t).collect(),
            None => results,
        };

        // Cache the filtered results
        if let Ok(json) = serde_json::to_string(&filtered_results) {
            let _ = self
                .redis
                .set_with_ttl(&cache_key, &json, self.cache_ttl)
                .await;
        }

        Ok(filtered_results)
    }

    /// Get current price quote for a stock
    ///
    /// Quotes are cached for 5 minutes.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock ticker symbol
    ///
    /// # Returns
    ///
    /// * `Ok(Asset)` - Current stock data
    #[instrument(skip(self))]
    pub async fn get_quote(&self, symbol: &str) -> Result<Asset> {
        let symbol_upper = symbol.to_uppercase();
        let cache_key = format!("{}{}", CACHE_PREFIX_QUOTE, symbol_upper);

        // Try cache first
        if let Ok(Some(cached)) = self.redis.get(&cache_key).await {
            debug!(symbol = symbol, "Quote cache hit");
            if let Ok(asset) = serde_json::from_str::<Asset>(&cached) {
                return Ok(asset);
            }
        }

        // Cache miss - fetch from API
        info!(symbol = symbol, "Quote cache miss, fetching from API");
        let mut asset = self.alpha_vantage.get_quote(&symbol_upper).await?;

        // Try to get the company name from search if we have it cached
        if asset.name == asset.symbol {
            if let Ok(search_results) = self.search(&symbol_upper, None).await {
                if let Some(result) = search_results.iter().find(|r| r.symbol == symbol_upper) {
                    asset.name = result.name.clone();
                }
            }
        }

        // Cache the result
        if let Ok(json) = serde_json::to_string(&asset) {
            let _ = self.redis.set_with_ttl(&cache_key, &json, self.cache_ttl).await;
        }

        Ok(asset)
    }

    /// Get historical price data for a stock
    ///
    /// Historical data is cached for 1 hour.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock ticker symbol
    /// * `timeframe` - Timeframe for historical data
    ///
    /// # Returns
    ///
    /// * `Ok(PriceHistory)` - Historical price data
    #[instrument(skip(self))]
    pub async fn get_history(&self, symbol: &str, timeframe: Timeframe) -> Result<PriceHistory> {
        let symbol_upper = symbol.to_uppercase();
        let cache_key = format!(
            "{}{}:{}",
            CACHE_PREFIX_HISTORY,
            symbol_upper,
            timeframe.as_str()
        );

        // Try cache first
        if let Ok(Some(cached)) = self.redis.get(&cache_key).await {
            debug!(symbol = symbol, timeframe = ?timeframe, "History cache hit");
            if let Ok(history) = serde_json::from_str::<PriceHistory>(&cached) {
                return Ok(history);
            }
        }

        // Cache miss - fetch from API
        info!(
            symbol = symbol,
            timeframe = ?timeframe,
            "History cache miss, fetching from API"
        );

        // Determine number of days to fetch based on timeframe
        // For Max timeframe, use usize::MAX to signal all available data
        let days = if timeframe == Timeframe::Max {
            usize::MAX
        } else {
            timeframe.days() as usize
        };

        let data = self
            .alpha_vantage
            .get_daily_history(&symbol_upper, days)
            .await?;

        let history = PriceHistory::new(symbol_upper, timeframe, data);

        // Cache the result
        if let Ok(json) = serde_json::to_string(&history) {
            let _ = self
                .redis
                .set_with_ttl(&cache_key, &json, self.history_cache_ttl)
                .await;
        }

        Ok(history)
    }

    /// Verify that a stock symbol exists
    ///
    /// Uses the quote endpoint to verify the symbol is valid.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock ticker symbol
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - Symbol exists
    /// * `Ok(false)` - Symbol does not exist
    #[instrument(skip(self))]
    pub async fn verify_symbol(&self, symbol: &str) -> Result<bool> {
        match self.get_quote(symbol).await {
            Ok(_) => Ok(true),
            Err(crate::utils::error::AppError::NotFound(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Invalidate cache for a specific symbol
    ///
    /// Useful for forcing a refresh of data.
    #[instrument(skip(self))]
    pub async fn invalidate_cache(&self, symbol: &str) -> Result<()> {
        let symbol_upper = symbol.to_uppercase();

        // Delete quote cache
        let quote_key = format!("{}{}", CACHE_PREFIX_QUOTE, symbol_upper);
        let _ = self.redis.delete(&quote_key).await;

        // Delete history cache for all timeframes
        for timeframe in ["1D", "1W", "1M", "3M", "6M", "1Y", "5Y", "MAX"] {
            let history_key = format!("{}{}:{}", CACHE_PREFIX_HISTORY, symbol_upper, timeframe);
            let _ = self.redis.delete(&history_key).await;
        }

        info!(symbol = symbol, "Cache invalidated");
        Ok(())
    }

    /// Get multiple quotes at once (batch operation)
    ///
    /// Fetches quotes for multiple symbols, using cache where available.
    ///
    /// # Arguments
    ///
    /// * `symbols` - List of stock ticker symbols
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Asset>)` - Assets for found symbols
    #[instrument(skip(self))]
    pub async fn get_quotes_batch(&self, symbols: &[String]) -> Result<Vec<Asset>> {
        let mut results = Vec::new();

        for symbol in symbols {
            match self.get_quote(symbol).await {
                Ok(asset) => results.push(asset),
                Err(e) => {
                    // Log but continue with other symbols
                    tracing::warn!(
                        symbol = symbol,
                        error = %e,
                        "Failed to fetch quote for symbol"
                    );
                }
            }
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    // Integration tests require Redis connection
    // See tests/integration/api/asset_test.rs
}
