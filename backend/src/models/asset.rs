//! Asset model and related DTOs
//!
//! This module defines asset-related structures for stocks, including
//! search results, current prices, and historical data.
//!
//! Note: Phase 1 only supports stocks. Crypto, ETF, and Bond support
//! will be added in Phase 2.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Asset type enumeration
///
/// Supports filtering by asset type: Stock, Crypto, ETF, Bond
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AssetType {
    #[default]
    Stock,
    Crypto,
    Etf,
    Bond,
}

impl AssetType {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            AssetType::Stock => "stock",
            AssetType::Crypto => "crypto",
            AssetType::Etf => "etf",
            AssetType::Bond => "bond",
        }
    }
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for AssetType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "stock" => Ok(AssetType::Stock),
            "crypto" => Ok(AssetType::Crypto),
            "etf" => Ok(AssetType::Etf),
            "bond" => Ok(AssetType::Bond),
            _ => Err(format!(
                "Invalid asset type: '{}'. Valid values: stock, crypto, etf, bond",
                s
            )),
        }
    }
}

/// Asset information with current price
///
/// This is the main structure returned for asset details including
/// current price and 24-hour changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    /// Stock ticker symbol (e.g., "AAPL", "GOOGL")
    pub symbol: String,

    /// Company name (e.g., "Apple Inc.")
    pub name: String,

    /// Asset type (always "stock" in Phase 1)
    pub asset_type: AssetType,

    /// Current price
    pub price: f64,

    /// Price change in last 24 hours (absolute value)
    pub change_24h: f64,

    /// Price change in last 24 hours (percentage)
    pub change_percent_24h: f64,

    /// Trading volume in last 24 hours
    pub volume_24h: Option<i64>,

    /// Market capitalization (if available)
    pub market_cap: Option<i64>,

    /// Previous trading day's closing price
    pub previous_close: Option<f64>,

    /// Today's opening price
    pub open: Option<f64>,

    /// Today's high price
    pub high: Option<f64>,

    /// Today's low price
    pub low: Option<f64>,

    /// 52-week high
    pub high_52_week: Option<f64>,

    /// 52-week low
    pub low_52_week: Option<f64>,

    /// Timestamp of last update
    pub last_updated: DateTime<Utc>,
}

impl Asset {
    /// Create a new Asset with minimal required fields
    pub fn new(symbol: String, name: String, price: f64, change: f64, change_percent: f64) -> Self {
        Self {
            symbol,
            name,
            asset_type: AssetType::Stock,
            price,
            change_24h: change,
            change_percent_24h: change_percent,
            volume_24h: None,
            market_cap: None,
            previous_close: None,
            open: None,
            high: None,
            low: None,
            high_52_week: None,
            low_52_week: None,
            last_updated: Utc::now(),
        }
    }

    /// Check if the price is up compared to previous close
    pub fn is_up(&self) -> bool {
        self.change_24h >= 0.0
    }
}

/// Search result for asset search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Stock ticker symbol
    pub symbol: String,

    /// Company/asset name
    pub name: String,

    /// Asset type
    pub asset_type: AssetType,

    /// Stock exchange/region
    pub region: Option<String>,

    /// Currency
    pub currency: Option<String>,

    /// Match score from search (0.0 - 1.0)
    pub match_score: Option<f64>,
}

impl SearchResult {
    /// Create a new SearchResult
    pub fn new(symbol: String, name: String) -> Self {
        Self {
            symbol,
            name,
            asset_type: AssetType::Stock,
            region: None,
            currency: None,
            match_score: None,
        }
    }
}

/// Search response containing multiple results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// Search query
    pub query: String,

    /// List of matching results
    pub results: Vec<SearchResult>,

    /// Total number of results
    pub total: usize,
}

/// Historical data timeframe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum Timeframe {
    /// 1 Day (intraday data)
    #[serde(rename = "1D")]
    #[default]
    OneDay,

    /// 1 Week
    #[serde(rename = "1W")]
    OneWeek,

    /// 1 Month
    #[serde(rename = "1M")]
    OneMonth,

    /// 3 Months
    #[serde(rename = "3M")]
    ThreeMonths,

    /// 6 Months
    #[serde(rename = "6M")]
    SixMonths,

    /// 1 Year
    #[serde(rename = "1Y")]
    OneYear,

    /// 5 Years
    #[serde(rename = "5Y")]
    FiveYears,

    /// All Time (maximum available history)
    #[serde(rename = "MAX")]
    Max,
}

impl Timeframe {
    /// Convert timeframe to string
    pub fn as_str(&self) -> &'static str {
        match self {
            Timeframe::OneDay => "1D",
            Timeframe::OneWeek => "1W",
            Timeframe::OneMonth => "1M",
            Timeframe::ThreeMonths => "3M",
            Timeframe::SixMonths => "6M",
            Timeframe::OneYear => "1Y",
            Timeframe::FiveYears => "5Y",
            Timeframe::Max => "MAX",
        }
    }

    /// Get the number of days for this timeframe
    ///
    /// For `Max`, returns `i64::MAX` to indicate all available data should be fetched.
    pub fn days(&self) -> i64 {
        match self {
            Timeframe::OneDay => 1,
            Timeframe::OneWeek => 7,
            Timeframe::OneMonth => 30,
            Timeframe::ThreeMonths => 90,
            Timeframe::SixMonths => 180,
            Timeframe::OneYear => 365,
            Timeframe::FiveYears => 1825, // 5 * 365
            Timeframe::Max => i64::MAX,   // All available data
        }
    }

    /// Check if this timeframe requires full output from the API
    ///
    /// Returns true for timeframes longer than 100 days (compact API limit)
    pub fn requires_full_output(&self) -> bool {
        match self {
            Timeframe::OneDay => false,
            Timeframe::OneWeek => false,
            Timeframe::OneMonth => false,
            Timeframe::ThreeMonths => false,
            Timeframe::SixMonths => true,
            Timeframe::OneYear => true,
            Timeframe::FiveYears => true,
            Timeframe::Max => true,
        }
    }
}

impl std::str::FromStr for Timeframe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "1D" => Ok(Timeframe::OneDay),
            "1W" => Ok(Timeframe::OneWeek),
            "1M" => Ok(Timeframe::OneMonth),
            "3M" => Ok(Timeframe::ThreeMonths),
            "6M" => Ok(Timeframe::SixMonths),
            "1Y" => Ok(Timeframe::OneYear),
            "5Y" => Ok(Timeframe::FiveYears),
            "MAX" => Ok(Timeframe::Max),
            _ => Err(format!(
                "Invalid timeframe: {}. Valid values: 1D, 1W, 1M, 3M, 6M, 1Y, 5Y, MAX",
                s
            )),
        }
    }
}

/// Single data point in historical price data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalDataPoint {
    /// Timestamp for this data point
    pub timestamp: DateTime<Utc>,

    /// Opening price
    pub open: f64,

    /// Highest price
    pub high: f64,

    /// Lowest price
    pub low: f64,

    /// Closing price
    pub close: f64,

    /// Trading volume
    pub volume: i64,
}

impl HistoricalDataPoint {
    /// Create a new HistoricalDataPoint
    pub fn new(
        timestamp: DateTime<Utc>,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: i64,
    ) -> Self {
        Self {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}

/// Historical price data for an asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceHistory {
    /// Stock symbol
    pub symbol: String,

    /// Timeframe of the data
    pub timeframe: Timeframe,

    /// Historical data points (sorted by timestamp ascending)
    pub data: Vec<HistoricalDataPoint>,

    /// Number of data points
    pub count: usize,

    /// Timestamp of the data fetch
    pub fetched_at: DateTime<Utc>,
}

impl PriceHistory {
    /// Create a new PriceHistory
    pub fn new(symbol: String, timeframe: Timeframe, data: Vec<HistoricalDataPoint>) -> Self {
        let count = data.len();
        Self {
            symbol,
            timeframe,
            data,
            count,
            fetched_at: Utc::now(),
        }
    }

    /// Get the latest price from history
    pub fn latest_price(&self) -> Option<f64> {
        self.data.last().map(|d| d.close)
    }

    /// Get the oldest price from history
    pub fn oldest_price(&self) -> Option<f64> {
        self.data.first().map(|d| d.close)
    }

    /// Calculate the percentage change over the history period
    pub fn total_change_percent(&self) -> Option<f64> {
        let oldest = self.oldest_price()?;
        let latest = self.latest_price()?;

        if oldest == 0.0 {
            return None;
        }

        Some(((latest - oldest) / oldest) * 100.0)
    }
}

/// Query parameters for asset search
#[derive(Debug, Clone, Deserialize)]
pub struct SearchQuery {
    /// Search query string
    pub q: String,

    /// Optional asset type filter (stock, crypto, etf, bond - case insensitive)
    /// If not provided, returns all asset types
    #[serde(default, rename = "type")]
    pub asset_type: Option<String>,
}

/// Query parameters for historical data
#[derive(Debug, Clone, Deserialize)]
pub struct HistoryQuery {
    /// Timeframe for historical data
    #[serde(default)]
    pub timeframe: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_type_display() {
        assert_eq!(AssetType::Stock.to_string(), "stock");
        assert_eq!(AssetType::Crypto.to_string(), "crypto");
        assert_eq!(AssetType::Etf.to_string(), "etf");
        assert_eq!(AssetType::Bond.to_string(), "bond");
    }

    #[test]
    fn test_asset_type_from_str() {
        // Test lowercase
        assert_eq!("stock".parse::<AssetType>().unwrap(), AssetType::Stock);
        assert_eq!("crypto".parse::<AssetType>().unwrap(), AssetType::Crypto);
        assert_eq!("etf".parse::<AssetType>().unwrap(), AssetType::Etf);
        assert_eq!("bond".parse::<AssetType>().unwrap(), AssetType::Bond);

        // Test uppercase
        assert_eq!("STOCK".parse::<AssetType>().unwrap(), AssetType::Stock);
        assert_eq!("CRYPTO".parse::<AssetType>().unwrap(), AssetType::Crypto);
        assert_eq!("ETF".parse::<AssetType>().unwrap(), AssetType::Etf);
        assert_eq!("BOND".parse::<AssetType>().unwrap(), AssetType::Bond);

        // Test mixed case
        assert_eq!("Stock".parse::<AssetType>().unwrap(), AssetType::Stock);
        assert_eq!("Crypto".parse::<AssetType>().unwrap(), AssetType::Crypto);
        assert_eq!("Etf".parse::<AssetType>().unwrap(), AssetType::Etf);
        assert_eq!("Bond".parse::<AssetType>().unwrap(), AssetType::Bond);

        // Test invalid
        assert!("invalid".parse::<AssetType>().is_err());
        assert!("".parse::<AssetType>().is_err());
    }

    #[test]
    fn test_asset_is_up() {
        let asset = Asset::new(
            "AAPL".to_string(),
            "Apple Inc.".to_string(),
            150.0,
            2.5,
            1.7,
        );
        assert!(asset.is_up());

        let asset = Asset::new(
            "AAPL".to_string(),
            "Apple Inc.".to_string(),
            150.0,
            -2.5,
            -1.7,
        );
        assert!(!asset.is_up());
    }

    #[test]
    fn test_timeframe_from_str() {
        assert_eq!("1D".parse::<Timeframe>().unwrap(), Timeframe::OneDay);
        assert_eq!("1d".parse::<Timeframe>().unwrap(), Timeframe::OneDay);
        assert_eq!("1w".parse::<Timeframe>().unwrap(), Timeframe::OneWeek);
        assert_eq!("1W".parse::<Timeframe>().unwrap(), Timeframe::OneWeek);
        assert_eq!("1M".parse::<Timeframe>().unwrap(), Timeframe::OneMonth);
        assert_eq!("1m".parse::<Timeframe>().unwrap(), Timeframe::OneMonth);
        assert_eq!("3M".parse::<Timeframe>().unwrap(), Timeframe::ThreeMonths);
        assert_eq!("3m".parse::<Timeframe>().unwrap(), Timeframe::ThreeMonths);
        assert_eq!("6M".parse::<Timeframe>().unwrap(), Timeframe::SixMonths);
        assert_eq!("6m".parse::<Timeframe>().unwrap(), Timeframe::SixMonths);
        assert_eq!("1Y".parse::<Timeframe>().unwrap(), Timeframe::OneYear);
        assert_eq!("1y".parse::<Timeframe>().unwrap(), Timeframe::OneYear);
        assert_eq!("5Y".parse::<Timeframe>().unwrap(), Timeframe::FiveYears);
        assert_eq!("5y".parse::<Timeframe>().unwrap(), Timeframe::FiveYears);
        assert_eq!("MAX".parse::<Timeframe>().unwrap(), Timeframe::Max);
        assert_eq!("max".parse::<Timeframe>().unwrap(), Timeframe::Max);
        assert_eq!("Max".parse::<Timeframe>().unwrap(), Timeframe::Max);
        assert!("invalid".parse::<Timeframe>().is_err());
    }

    #[test]
    fn test_timeframe_days() {
        assert_eq!(Timeframe::OneDay.days(), 1);
        assert_eq!(Timeframe::OneWeek.days(), 7);
        assert_eq!(Timeframe::OneMonth.days(), 30);
        assert_eq!(Timeframe::ThreeMonths.days(), 90);
        assert_eq!(Timeframe::SixMonths.days(), 180);
        assert_eq!(Timeframe::OneYear.days(), 365);
        assert_eq!(Timeframe::FiveYears.days(), 1825);
        assert_eq!(Timeframe::Max.days(), i64::MAX);
    }

    #[test]
    fn test_timeframe_as_str() {
        assert_eq!(Timeframe::OneDay.as_str(), "1D");
        assert_eq!(Timeframe::OneWeek.as_str(), "1W");
        assert_eq!(Timeframe::OneMonth.as_str(), "1M");
        assert_eq!(Timeframe::ThreeMonths.as_str(), "3M");
        assert_eq!(Timeframe::SixMonths.as_str(), "6M");
        assert_eq!(Timeframe::OneYear.as_str(), "1Y");
        assert_eq!(Timeframe::FiveYears.as_str(), "5Y");
        assert_eq!(Timeframe::Max.as_str(), "MAX");
    }

    #[test]
    fn test_timeframe_requires_full_output() {
        assert!(!Timeframe::OneDay.requires_full_output());
        assert!(!Timeframe::OneWeek.requires_full_output());
        assert!(!Timeframe::OneMonth.requires_full_output());
        assert!(!Timeframe::ThreeMonths.requires_full_output());
        assert!(Timeframe::SixMonths.requires_full_output());
        assert!(Timeframe::OneYear.requires_full_output());
        assert!(Timeframe::FiveYears.requires_full_output());
        assert!(Timeframe::Max.requires_full_output());
    }

    #[test]
    fn test_price_history_calculations() {
        let data = vec![
            HistoricalDataPoint::new(Utc::now(), 100.0, 105.0, 99.0, 100.0, 1000),
            HistoricalDataPoint::new(Utc::now(), 100.0, 110.0, 100.0, 110.0, 1200),
        ];

        let history = PriceHistory::new("AAPL".to_string(), Timeframe::OneDay, data);

        assert_eq!(history.oldest_price(), Some(100.0));
        assert_eq!(history.latest_price(), Some(110.0));
        assert_eq!(history.total_change_percent(), Some(10.0));
        assert_eq!(history.count, 2);
    }

    #[test]
    fn test_search_result_new() {
        let result = SearchResult::new("AAPL".to_string(), "Apple Inc.".to_string());

        assert_eq!(result.symbol, "AAPL");
        assert_eq!(result.name, "Apple Inc.");
        assert_eq!(result.asset_type, AssetType::Stock);
    }
}
