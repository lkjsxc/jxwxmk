# Multi-stage build for static Rust server

# Build stage
FROM rust:1-slim as builder

WORKDIR /usr/src/app

# Install build dependencies for MUSL
RUN apt-get update && apt-get install -y \
    musl-tools \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Add MUSL target
RUN rustup target add x86_64-unknown-linux-musl

# Copy Cargo files for dependency caching
COPY src/server/Cargo.toml /usr/src/app/Cargo.toml
COPY src/server/Cargo.lock /usr/src/app/Cargo.lock

# Build dummy to cache dependencies
# Note: This is slightly tricky with --target, but we'll just copy the source first for simplicity
# since the build time isn't too long here.
COPY src/server/src /usr/src/app/src

# Build the application statically
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage
FROM alpine:latest

WORKDIR /app

# Install CA certificates for TLS (required for rustls)
RUN apk add --no-cache ca-certificates

# Copy built binary from builder
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/game-server /usr/local/bin/game-server

# Copy configuration
COPY config /app/config
COPY .env /app/.env

# Copy static assets
COPY src/assets /app/assets

# Expose ports
EXPOSE 8080 8081

# Set environment variables
ENV RUST_LOG=info
ENV SERVER_ENV=production

# Run the server
CMD ["/usr/local/bin/game-server"]
