//! Alpha Vantage API client
//!
//! This module provides an HTTP client for interacting with the Alpha Vantage
//! stock market data API. It handles:
//! - Symbol search (SYMBOL_SEARCH)
//! - Current price quotes (GLOBAL_QUOTE)
//! - Historical daily data (TIME_SERIES_DAILY)
//!
//! Rate limiting is handled through aggressive caching to stay within
//! the free tier limits (5 requests/minute, 500/day).

use chrono::{NaiveDate, TimeZone, Utc};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use tracing::{debug, info, instrument, warn};

use crate::config::AlphaVantageConfig;
use crate::models::asset::{Asset, AssetType, HistoricalDataPoint, SearchResult};
use crate::utils::error::{AppError, Result};

/// Alpha Vantage API client
#[derive(Clone)]
pub struct AlphaVantageClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl AlphaVantageClient {
    /// Create a new Alpha Vantage client
    pub fn new(config: &AlphaVantageConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_key: config.api_key.clone(),
            base_url: config.base_url.clone(),
        }
    }

    /// Search for stocks by keyword
    ///
    /// Uses the SYMBOL_SEARCH endpoint to find matching stocks.
    ///
    /// # Arguments
    ///
    /// * `query` - Search keyword (symbol or company name)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<SearchResult>)` - Matching stocks
    /// * `Err(AppError)` - If the API call fails
    #[instrument(skip(self))]
    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        info!(query = query, "Searching for stocks");

        let url = format!(
            "{}?function=SYMBOL_SEARCH&keywords={}&apikey={}",
            self.base_url, query, self.api_key
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::ExternalApiError(format!(
                "Alpha Vantage API error: {}",
                response.status()
            )));
        }

        let body: SearchResponse = response.json().await.map_err(|e| {
            AppError::ExternalApiError(format!("Failed to parse search response: {}", e))
        })?;

        // Check for API error messages
        if let Some(note) = body.note {
            if note.contains("API call frequency") {
                warn!("Alpha Vantage rate limit hit");
                return Err(AppError::RateLimitExceeded);
            }
        }

        // Filter to only US stocks (Phase 1)
        let results: Vec<SearchResult> = body
            .best_matches
            .unwrap_or_default()
            .into_iter()
            .filter(|m| {
                // Filter for US stocks only (NYSE, NASDAQ)
                let region = m.region.as_deref().unwrap_or("");
                region == "United States" || region.is_empty()
            })
            .map(|m| SearchResult {
                symbol: m.symbol,
                name: m.name,
                asset_type: AssetType::Stock,
                region: m.region,
                currency: m.currency,
                match_score: m.match_score.and_then(|s| s.parse().ok()),
            })
            .collect();

        debug!(count = results.len(), "Search completed");
        Ok(results)
    }

    /// Get current price quote for a stock
    ///
    /// Uses the GLOBAL_QUOTE endpoint to get real-time price data.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock ticker symbol (e.g., "AAPL")
    ///
    /// # Returns
    ///
    /// * `Ok(Asset)` - Current stock data
    /// * `Err(AppError)` - If the API call fails
    #[instrument(skip(self))]
    pub async fn get_quote(&self, symbol: &str) -> Result<Asset> {
        info!(symbol = symbol, "Fetching stock quote");

        let url = format!(
            "{}?function=GLOBAL_QUOTE&symbol={}&apikey={}",
            self.base_url, symbol, self.api_key
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::ExternalApiError(format!(
                "Alpha Vantage API error: {}",
                response.status()
            )));
        }

        let body: QuoteResponse = response.json().await.map_err(|e| {
            AppError::ExternalApiError(format!("Failed to parse quote response: {}", e))
        })?;

        // Check for API error messages
        if let Some(note) = body.note {
            if note.contains("API call frequency") {
                warn!("Alpha Vantage rate limit hit");
                return Err(AppError::RateLimitExceeded);
            }
        }

        let quote = body.global_quote.ok_or_else(|| {
            AppError::NotFound(format!("Stock not found: {}", symbol))
        })?;

        // Parse price values
        let price = quote
            .price
            .parse::<f64>()
            .map_err(|_| AppError::ExternalApiError("Invalid price format".to_string()))?;

        let change = quote
            .change
            .parse::<f64>()
            .unwrap_or(0.0);

        let change_percent = quote
            .change_percent
            .replace('%', "")
            .parse::<f64>()
            .unwrap_or(0.0);

        let volume = quote
            .volume
            .parse::<i64>()
            .ok();

        let previous_close = quote
            .previous_close
            .parse::<f64>()
            .ok();

        let open = quote
            .open
            .parse::<f64>()
            .ok();

        let high = quote
            .high
            .parse::<f64>()
            .ok();

        let low = quote
            .low
            .parse::<f64>()
            .ok();

        let asset = Asset {
            symbol: symbol.to_uppercase(),
            name: symbol.to_uppercase(), // Alpha Vantage GLOBAL_QUOTE doesn't return name
            asset_type: AssetType::Stock,
            price,
            change_24h: change,
            change_percent_24h: change_percent,
            volume_24h: volume,
            market_cap: None, // Not available in GLOBAL_QUOTE
            previous_close,
            open,
            high,
            low,
            high_52_week: None, // Not available in GLOBAL_QUOTE
            low_52_week: None,
            last_updated: Utc::now(),
        };

        debug!(symbol = symbol, price = price, "Quote fetched successfully");
        Ok(asset)
    }

    /// Get historical daily price data for a stock
    ///
    /// Uses the TIME_SERIES_DAILY endpoint to get historical OHLCV data.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock ticker symbol
    /// * `days` - Number of days of history to return (use `usize::MAX` for all available data)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<HistoricalDataPoint>)` - Historical price data
    /// * `Err(AppError)` - If the API call fails
    #[instrument(skip(self))]
    pub async fn get_daily_history(
        &self,
        symbol: &str,
        days: usize,
    ) -> Result<Vec<HistoricalDataPoint>> {
        info!(symbol = symbol, days = days, "Fetching daily history");

        // Use compact output for up to 100 days, full for more
        // Also use full for usize::MAX (MAX timeframe - all available data)
        let output_size = if days <= 100 { "compact" } else { "full" };

        let url = format!(
            "{}?function=TIME_SERIES_DAILY&symbol={}&outputsize={}&apikey={}",
            self.base_url, symbol, output_size, self.api_key
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::ExternalApiError(format!(
                "Alpha Vantage API error: {}",
                response.status()
            )));
        }

        let body: TimeSeriesResponse = response.json().await.map_err(|e| {
            AppError::ExternalApiError(format!("Failed to parse time series response: {}", e))
        })?;

        // Check for API error messages
        if let Some(note) = body.note {
            if note.contains("API call frequency") {
                warn!("Alpha Vantage rate limit hit");
                return Err(AppError::RateLimitExceeded);
            }
        }

        if let Some(error) = body.error_message {
            return Err(AppError::NotFound(format!(
                "Stock not found or invalid: {}",
                error
            )));
        }

        let time_series = body.time_series_daily.ok_or_else(|| {
            AppError::NotFound(format!("No historical data for: {}", symbol))
        })?;

        // Parse and sort data points
        let mut data_points: Vec<HistoricalDataPoint> = time_series
            .into_iter()
            .filter_map(|(date_str, values)| {
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok()?;
                let timestamp = Utc.from_utc_datetime(
                    &date.and_hms_opt(16, 0, 0)? // Market close time
                );

                let open = values.open.parse::<f64>().ok()?;
                let high = values.high.parse::<f64>().ok()?;
                let low = values.low.parse::<f64>().ok()?;
                let close = values.close.parse::<f64>().ok()?;
                let volume = values.volume.parse::<i64>().ok()?;

                Some(HistoricalDataPoint::new(timestamp, open, high, low, close, volume))
            })
            .collect();

        // Sort by timestamp ascending
        data_points.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // Limit to requested days (skip limiting for MAX timeframe - usize::MAX)
        let result = if days == usize::MAX {
            // Return all available data for MAX timeframe
            data_points
        } else {
            let start_index = data_points.len().saturating_sub(days);
            data_points[start_index..].to_vec()
        };

        debug!(
            symbol = symbol,
            count = result.len(),
            "History fetched successfully"
        );

        Ok(result)
    }

    /// Check if the API is available (simple health check)
    pub async fn health_check(&self) -> Result<bool> {
        // Use a simple search to test API connectivity
        let url = format!(
            "{}?function=SYMBOL_SEARCH&keywords=IBM&apikey={}",
            self.base_url, self.api_key
        );

        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

// Alpha Vantage API response structures

#[derive(Debug, Deserialize)]
struct SearchResponse {
    #[serde(rename = "bestMatches")]
    best_matches: Option<Vec<SearchMatch>>,
    #[serde(rename = "Note")]
    note: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SearchMatch {
    #[serde(rename = "1. symbol")]
    symbol: String,
    #[serde(rename = "2. name")]
    name: String,
    #[serde(rename = "3. type")]
    #[allow(dead_code)]
    asset_type: Option<String>,
    #[serde(rename = "4. region")]
    region: Option<String>,
    #[serde(rename = "8. currency")]
    currency: Option<String>,
    #[serde(rename = "9. matchScore")]
    match_score: Option<String>,
}

#[derive(Debug, Deserialize)]
struct QuoteResponse {
    #[serde(rename = "Global Quote")]
    global_quote: Option<GlobalQuote>,
    #[serde(rename = "Note")]
    note: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GlobalQuote {
    #[serde(rename = "01. symbol")]
    _symbol: String,
    #[serde(rename = "02. open")]
    open: String,
    #[serde(rename = "03. high")]
    high: String,
    #[serde(rename = "04. low")]
    low: String,
    #[serde(rename = "05. price")]
    price: String,
    #[serde(rename = "06. volume")]
    volume: String,
    #[serde(rename = "08. previous close")]
    previous_close: String,
    #[serde(rename = "09. change")]
    change: String,
    #[serde(rename = "10. change percent")]
    change_percent: String,
}

#[derive(Debug, Deserialize)]
struct TimeSeriesResponse {
    #[serde(rename = "Time Series (Daily)")]
    time_series_daily: Option<HashMap<String, DailyData>>,
    #[serde(rename = "Note")]
    note: Option<String>,
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DailyData {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = AlphaVantageConfig {
            api_key: "test-key".to_string(),
            base_url: "https://www.alphavantage.co/query".to_string(),
            timeout_seconds: 30,
        };

        let client = AlphaVantageClient::new(&config);
        assert_eq!(client.api_key, "test-key");
    }

    // Integration tests with actual API calls are in the integration test suite
    // We use wiremock to mock the API responses
}
