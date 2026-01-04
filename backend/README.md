# Backend - Rust API Server

RESTful API server with WebSocket support built with Rust.

## Directory Structure

```
backend/
├── src/
│   ├── api/                    # API routes and handlers
│   │   ├── routes/             # Route definitions
│   │   │   ├── auth.rs         # Authentication routes
│   │   │   ├── assets.rs       # Asset routes
│   │   │   ├── watchlists.rs   # Watchlist routes
│   │   │   ├── alerts.rs       # Alert routes
│   │   │   └── mod.rs
│   │   ├── handlers/           # Request handlers
│   │   │   ├── auth_handler.rs
│   │   │   ├── asset_handler.rs
│   │   │   ├── watchlist_handler.rs
│   │   │   ├── alert_handler.rs
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── services/               # Business logic layer
│   │   ├── auth_service.rs     # Authentication logic
│   │   ├── asset_service.rs    # Asset data aggregation
│   │   ├── watchlist_service.rs
│   │   ├── alert_service.rs    # Alert processing
│   │   ├── pattern_detection.rs # Pattern detection algorithms
│   │   ├── websocket_service.rs # WebSocket handling
│   │   ├── data_aggregator.rs  # External API aggregation
│   │   └── mod.rs
│   ├── models/                 # Data models and database schemas
│   │   ├── user.rs
│   │   ├── watchlist.rs
│   │   ├── alert.rs
│   │   ├── asset.rs
│   │   └── mod.rs
│   ├── middleware/             # Middleware components
│   │   ├── auth.rs             # JWT authentication middleware
│   │   ├── rate_limit.rs       # Rate limiting
│   │   ├── cors.rs             # CORS configuration
│   │   ├── logging.rs          # Request logging
│   │   └── mod.rs
│   ├── utils/                  # Utility functions
│   │   ├── jwt.rs              # JWT utilities
│   │   ├── password.rs         # Password hashing (bcrypt/Argon2)
│   │   ├── validation.rs       # Input validation
│   │   ├── error.rs            # Custom error types
│   │   └── mod.rs
│   ├── config/                 # Configuration management
│   │   ├── database.rs         # MongoDB/Redis config
│   │   ├── settings.rs         # App settings
│   │   └── mod.rs
│   ├── main.rs                 # Application entry point
│   └── lib.rs                  # Library root
├── tests/                      # Test files
│   ├── unit/                   # Unit tests
│   │   ├── services/
│   │   └── utils/
│   ├── integration/            # Integration tests
│   │   ├── api/
│   │   └── db/
│   └── common/                 # Test utilities
│       └── mod.rs
├── migrations/                 # Database migrations
├── .env.example                # Environment variables example
├── Cargo.toml                  # Rust dependencies
├── Cargo.lock                  # Dependency lock file
├── rust-toolchain.toml         # Rust version specification
└── Dockerfile                  # Docker configuration
```

## Tech Stack

- Language: Rust (latest stable)
- Web Framework: Actix-web or Axum
- Async Runtime: Tokio
- Database: MongoDB (via mongodb crate)
- Cache: Redis (via redis crate)
- Authentication: JWT (via jsonwebtoken crate)
- Password Hashing: Argon2 or bcrypt
- WebSocket: tokio-tungstenite or framework built-in
- Serialization: serde, serde_json

## Development Commands

```bash
# Build the project
cargo build

# Run the server (development mode)
cargo run

# Run with watch mode (requires cargo-watch)
cargo watch -x run

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Build for production
cargo build --release

# Run benchmarks
cargo bench
```

## Background Services

- **Alert Processor**: Periodic job to check price and pattern alerts
- **Data Aggregator**: Fetch and cache market data from external APIs
- **WebSocket Manager**: Handle real-time price update subscriptions
