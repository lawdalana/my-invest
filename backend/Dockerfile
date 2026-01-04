# Multi-stage build for My-Invest Backend API
# Stage 1: Builder - Compile the Rust application
# Stage 2: Runtime - Minimal production image

# ============================================================================
# Stage 1: Builder
# ============================================================================
FROM rust:latest AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty project for caching dependencies
WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Create dummy source file to build dependencies
RUN mkdir -p src && \
    echo 'fn main() { println!("Dummy"); }' > src/main.rs

# Build dependencies only (this layer will be cached)
RUN cargo build --release && \
    rm -rf src target/release/deps/my_invest_backend*

# Copy actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release

# ============================================================================
# Stage 2: Runtime
# ============================================================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -r -s /bin/false -u 1001 appuser

# Set working directory
WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/my-invest-backend /app/my-invest-backend

# Change ownership to non-root user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose the API port
EXPOSE 8080

# Set environment variables
ENV HOST=0.0.0.0
ENV PORT=8080
ENV RUST_LOG=info

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/health || exit 1

# Run the application
CMD ["./my-invest-backend"]
