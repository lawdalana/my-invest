# Backend Development Setup Guide

This guide walks you through setting up the My-Invest backend for local development.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (1.75 or later) - [Install Rust](https://rustup.rs/)
- **Docker** and **Docker Compose** - [Install Docker](https://docs.docker.com/get-docker/)
- **Alpha Vantage API Key** - [Get free API key](https://www.alphavantage.co/support/#api-key)

Optional but recommended:
- **cargo-watch** for hot reloading: `cargo install cargo-watch`
- **MongoDB Compass** for database GUI
- **Redis Insight** for Redis GUI

## Quick Start with Docker

The fastest way to get started is using Docker Compose:

```bash
# Clone the repository
git clone <repository-url>
cd my-invest

# Create environment file
cp backend/.env.example backend/.env

# Edit .env and add your Alpha Vantage API key
# ALPHA_VANTAGE_API_KEY=your-actual-api-key

# Start all services
docker-compose -f docker-compose.dev.yml up
```

The backend will be available at `http://localhost:8080`.

## Manual Setup (Without Docker)

### 1. Install Dependencies

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### 2. Start MongoDB and Redis

You can run MongoDB and Redis using Docker:

```bash
# Start MongoDB
docker run -d --name mongodb -p 27017:27017 mongo:7.0

# Start Redis
docker run -d --name redis -p 6379:6379 redis:7.2-alpine
```

Or install them locally:
- [MongoDB Installation](https://www.mongodb.com/docs/manual/installation/)
- [Redis Installation](https://redis.io/docs/getting-started/installation/)

### 3. Configure Environment

```bash
cd backend

# Copy environment template
cp .env.example .env

# Edit .env with your settings
# At minimum, set:
# - JWT_SECRET (use a secure random string)
# - ALPHA_VANTAGE_API_KEY (your API key)
```

### 4. Build and Run

```bash
# Navigate to backend directory
cd backend

# Build the project
cargo build

# Run the server
cargo run
```

The server will start at `http://localhost:8080`.

### 5. Verify Setup

Test the health endpoint:

```bash
curl http://localhost:8080/health
```

Expected response:
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "service": "my-invest-backend"
}
```

## Development Workflow

### Hot Reloading

For automatic rebuilds during development:

```bash
# Install cargo-watch
cargo install cargo-watch

# Run with hot reload
cargo watch -x run
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test unit::utils::password_test

# Run tests with output
cargo test -- --nocapture

# Run only unit tests
cargo test --lib

# Run integration tests (requires MongoDB and Redis)
cargo test --test integration
```

### Linting and Formatting

```bash
# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt

# Run linter
cargo clippy

# Fix linting issues
cargo clippy --fix
```

### Building for Production

```bash
# Build optimized release
cargo build --release

# The binary will be at: target/release/my-invest-backend
```

## Project Structure

```
backend/
├── Cargo.toml              # Dependencies and project metadata
├── Cargo.lock              # Locked dependency versions
├── .env.example            # Environment template
├── Dockerfile              # Production container
├── Dockerfile.dev          # Development container
├── src/
│   ├── main.rs             # Application entry point
│   ├── lib.rs              # Library exports
│   ├── api/
│   │   ├── mod.rs
│   │   ├── handlers/       # HTTP request handlers
│   │   └── routes/         # Route definitions
│   ├── config/
│   │   ├── mod.rs          # Configuration structs
│   │   └── database.rs     # Database connections
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── auth.rs         # JWT authentication
│   │   ├── cors.rs         # CORS configuration
│   │   └── rate_limit.rs   # Rate limiting
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs         # User model and DTOs
│   │   ├── asset.rs        # Asset model and DTOs
│   │   └── watchlist.rs    # Watchlist model and DTOs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── auth_service.rs
│   │   ├── alpha_vantage_client.rs
│   │   ├── asset_service.rs
│   │   └── watchlist_service.rs
│   └── utils/
│       ├── mod.rs
│       ├── error.rs        # Error handling
│       ├── jwt.rs          # JWT utilities
│       ├── logging.rs      # Logging setup
│       └── password.rs     # Password hashing
├── tests/
│   ├── common/             # Shared test utilities
│   ├── unit/               # Unit tests
│   └── integration/        # Integration tests
└── migrations/             # Database migrations (future)
```

## Common Tasks

### Adding a New Endpoint

1. Create or update handler in `src/api/handlers/`
2. Add route in `src/api/routes/mod.rs`
3. Add tests in `tests/`

### Adding a New Model

1. Create model file in `src/models/`
2. Export from `src/models/mod.rs`
3. Add any necessary validation with `validator` crate

### Working with the Database

```rust
// Example: Query MongoDB
let user = self.users
    .find_one(doc! { "email": email }, None)
    .await?;

// Example: Insert document
self.users.insert_one(&user, None).await?;
```

### Working with Cache

```rust
// Set with TTL
redis.set_with_ttl("key", "value", 300).await?;

// Get value
let value: Option<String> = redis.get("key").await?;

// Delete key
redis.delete("key").await?;
```

## Troubleshooting

### MongoDB Connection Issues

```bash
# Check if MongoDB is running
docker ps | grep mongodb

# View MongoDB logs
docker logs mongodb
```

### Redis Connection Issues

```bash
# Check if Redis is running
docker ps | grep redis

# Test Redis connection
redis-cli ping
```

### Build Errors

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build
```

### Port Already in Use

```bash
# Find process using port 8080
lsof -i :8080

# Kill the process
kill -9 <PID>
```

## Next Steps

- Read the [API Documentation](../api/phase1-endpoints.md)
- Review [Environment Variables](./environment-variables.md)
- Check the [Architecture Overview](../architecture/backend-phase1.md)
