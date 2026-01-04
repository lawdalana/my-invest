//! My-Invest Backend API Server
//!
//! A RESTful API server for the My-Invest Dashboard application.
//! Provides endpoints for health checks, assets, watchlists, and user management.

use actix_cors::Cors;
use actix_web::{get, http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod api;
mod config;
mod models;

// ============================================================================
// Configuration
// ============================================================================

/// Application configuration loaded from environment variables
#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub frontend_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a valid number"),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "mongodb://localhost:27017/myinvest".to_string()),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
        }
    }
}

// ============================================================================
// Models (Mock Data)
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub symbol: String,
    pub name: String,
    pub asset_type: String,
    pub current_price: f64,
    pub change_24h: f64,
    pub change_percent_24h: f64,
    pub volume_24h: f64,
    pub market_cap: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Watchlist {
    pub id: String,
    pub name: String,
    pub assets: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub preferences: UserPreferences,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPreferences {
    pub theme: String,
    pub default_timeframe: String,
    pub notifications: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.to_string()),
        }
    }
}

// ============================================================================
// Mock Data Generators
// ============================================================================

fn get_mock_assets() -> Vec<Asset> {
    vec![
        Asset {
            symbol: "AAPL".to_string(),
            name: "Apple Inc.".to_string(),
            asset_type: "stock".to_string(),
            current_price: 178.72,
            change_24h: 2.35,
            change_percent_24h: 1.33,
            volume_24h: 52_345_678.0,
            market_cap: Some(2_800_000_000_000.0),
        },
        Asset {
            symbol: "MSFT".to_string(),
            name: "Microsoft Corporation".to_string(),
            asset_type: "stock".to_string(),
            current_price: 378.91,
            change_24h: -1.23,
            change_percent_24h: -0.32,
            volume_24h: 23_456_789.0,
            market_cap: Some(2_810_000_000_000.0),
        },
        Asset {
            symbol: "BTC-USD".to_string(),
            name: "Bitcoin".to_string(),
            asset_type: "crypto".to_string(),
            current_price: 43_250.00,
            change_24h: 1_250.00,
            change_percent_24h: 2.98,
            volume_24h: 28_500_000_000.0,
            market_cap: Some(848_000_000_000.0),
        },
        Asset {
            symbol: "ETH-USD".to_string(),
            name: "Ethereum".to_string(),
            asset_type: "crypto".to_string(),
            current_price: 2_280.50,
            change_24h: -45.30,
            change_percent_24h: -1.95,
            volume_24h: 12_300_000_000.0,
            market_cap: Some(274_000_000_000.0),
        },
        Asset {
            symbol: "SPY".to_string(),
            name: "SPDR S&P 500 ETF Trust".to_string(),
            asset_type: "etf".to_string(),
            current_price: 476.23,
            change_24h: 3.12,
            change_percent_24h: 0.66,
            volume_24h: 78_234_567.0,
            market_cap: Some(437_000_000_000.0),
        },
        Asset {
            symbol: "GOOGL".to_string(),
            name: "Alphabet Inc.".to_string(),
            asset_type: "stock".to_string(),
            current_price: 141.80,
            change_24h: 0.95,
            change_percent_24h: 0.67,
            volume_24h: 18_765_432.0,
            market_cap: Some(1_780_000_000_000.0),
        },
    ]
}

fn get_mock_watchlists() -> Vec<Watchlist> {
    vec![
        Watchlist {
            id: "wl_001".to_string(),
            name: "Tech Stocks".to_string(),
            assets: vec!["AAPL".to_string(), "MSFT".to_string(), "GOOGL".to_string()],
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-15T10:30:00Z".to_string(),
        },
        Watchlist {
            id: "wl_002".to_string(),
            name: "Crypto Portfolio".to_string(),
            assets: vec!["BTC-USD".to_string(), "ETH-USD".to_string()],
            created_at: "2024-01-05T00:00:00Z".to_string(),
            updated_at: "2024-01-14T15:45:00Z".to_string(),
        },
        Watchlist {
            id: "wl_003".to_string(),
            name: "ETFs".to_string(),
            assets: vec!["SPY".to_string()],
            created_at: "2024-01-10T00:00:00Z".to_string(),
            updated_at: "2024-01-10T00:00:00Z".to_string(),
        },
    ]
}

fn get_mock_user() -> User {
    User {
        id: "user_001".to_string(),
        email: "demo@myinvest.app".to_string(),
        preferences: UserPreferences {
            theme: "dark".to_string(),
            default_timeframe: "1D".to_string(),
            notifications: true,
        },
    }
}

// ============================================================================
// API Handlers
// ============================================================================

/// Health check endpoint
#[get("/api/health")]
async fn health_check() -> impl Responder {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    HttpResponse::Ok().json(response)
}

/// Root endpoint - API info
#[get("/")]
async fn api_info() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "name": "My-Invest API",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Backend API for My-Invest Dashboard",
        "endpoints": {
            "health": "/api/health",
            "assets": "/api/v1/assets",
            "watchlists": "/api/v1/watchlists",
            "user": "/api/v1/user"
        }
    }))
}

/// Get all assets (mock data)
#[get("/api/v1/assets")]
async fn get_assets() -> impl Responder {
    let assets = get_mock_assets();
    HttpResponse::Ok().json(ApiResponse::success(assets))
}

/// Search assets by query
#[get("/api/v1/assets/search")]
async fn search_assets(query: web::Query<SearchQuery>) -> impl Responder {
    let assets = get_mock_assets();
    let search_term = query.q.to_lowercase();

    let filtered: Vec<Asset> = assets
        .into_iter()
        .filter(|asset| {
            asset.symbol.to_lowercase().contains(&search_term)
                || asset.name.to_lowercase().contains(&search_term)
        })
        .filter(|asset| {
            query.asset_type.as_ref().map_or(true, |t| &asset.asset_type == t)
        })
        .collect();

    HttpResponse::Ok().json(ApiResponse::success(filtered))
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(rename = "type")]
    pub asset_type: Option<String>,
}

/// Get single asset by symbol
#[get("/api/v1/assets/{symbol}")]
async fn get_asset(path: web::Path<String>) -> impl Responder {
    let symbol = path.into_inner().to_uppercase();
    let assets = get_mock_assets();

    match assets.into_iter().find(|a| a.symbol == symbol) {
        Some(asset) => HttpResponse::Ok().json(ApiResponse::success(asset)),
        None => HttpResponse::NotFound().json(ApiResponse::<Asset>::error("Asset not found")),
    }
}

/// Get all watchlists (mock data)
#[get("/api/v1/watchlists")]
async fn get_watchlists() -> impl Responder {
    let watchlists = get_mock_watchlists();
    HttpResponse::Ok().json(ApiResponse::success(watchlists))
}

/// Get single watchlist by ID
#[get("/api/v1/watchlists/{id}")]
async fn get_watchlist(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let watchlists = get_mock_watchlists();

    match watchlists.into_iter().find(|w| w.id == id) {
        Some(watchlist) => HttpResponse::Ok().json(ApiResponse::success(watchlist)),
        None => HttpResponse::NotFound().json(ApiResponse::<Watchlist>::error("Watchlist not found")),
    }
}

/// Get current user (mock data)
#[get("/api/v1/user")]
async fn get_user() -> impl Responder {
    let user = get_mock_user();
    HttpResponse::Ok().json(ApiResponse::success(user))
}

/// Get user preferences
#[get("/api/v1/user/preferences")]
async fn get_user_preferences() -> impl Responder {
    let user = get_mock_user();
    HttpResponse::Ok().json(ApiResponse::success(user.preferences))
}

// ============================================================================
// Main Application
// ============================================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing subscriber for logging
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    // Load configuration from environment
    let config = AppConfig::from_env();

    info!(
        "Starting My-Invest API server on {}:{}",
        config.host, config.port
    );
    info!("Frontend URL: {}", config.frontend_url);
    info!("Database URL: {}", config.database_url);
    info!("Redis URL: {}", config.redis_url);

    let frontend_url = config.frontend_url.clone();
    let bind_addr = format!("{}:{}", config.host, config.port);

    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://frontend:80")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            // Middleware
            .wrap(cors)
            .wrap(Logger::default())
            // Routes
            .service(api_info)
            .service(health_check)
            .service(get_assets)
            .service(search_assets)
            .service(get_asset)
            .service(get_watchlists)
            .service(get_watchlist)
            .service(get_user)
            .service(get_user_preferences)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
