# =============================================================================
# My-Invest Backend - Production Dockerfile
# =============================================================================
# Multi-stage build for optimized production image
# =============================================================================

# -----------------------------------------------------------------------------
# Stage 1: Build the application
# -----------------------------------------------------------------------------
FROM rust:1.75-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty project
WORKDIR /app

# Copy Cargo files first for dependency caching
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only (this layer will be cached)
RUN cargo build --release && rm -rf src

# Copy the actual source code
COPY src ./src

# Touch main.rs to ensure it's rebuilt
RUN touch src/main.rs

# Build the application
RUN cargo build --release --bin my-invest-backend

# -----------------------------------------------------------------------------
# Stage 2: Create the runtime image
# -----------------------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -r -s /bin/false myinvest

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/my-invest-backend /app/my-invest-backend

# Set ownership
RUN chown -R myinvest:myinvest /app

# Switch to non-root user
USER myinvest

# Expose the port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Set environment variables
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080
ENV RUST_LOG=info,my_invest_backend=info

# Run the application
CMD ["/app/my-invest-backend"]
