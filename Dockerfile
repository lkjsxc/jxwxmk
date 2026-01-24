# Multi-stage build for Rust server

# Build stage
FROM rust:1-slim as builder

WORKDIR /usr/src/app

# Copy Cargo files for dependency caching
COPY src/server/Cargo.toml /usr/src/app/Cargo.toml
COPY src/server/Cargo.lock /usr/src/app/Cargo.lock

# Build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY src/server/src /usr/src/app/src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /usr/src/app

# Install runtime dependencies
RUN apt-get update && apt-get install -y 
    libssl1.1 
    libssl3 
    ca-certificates 
    && rm -rf /var/lib/apt/lists/*

# Copy built binary from builder
COPY --from=builder /usr/src/app/target/release/game-server /usr/local/bin/game-server

# Copy configuration
COPY config /app/config
COPY .env .env

# Copy static assets (will be built separately)
COPY src/assets /app/assets

# Expose ports
EXPOSE 8080 8081

# Set environment variables
ENV RUST_LOG=info
ENV SERVER_ENV=production

# Run the server
CMD ["/usr/local/bin/game-server"]