# syntax=docker/dockerfile:1
# Build stage 1: Node.js for TypeScript client
FROM node:20-slim AS node-builder
WORKDIR /build

# Install dependencies first (better caching)
COPY src/client/package.json ./
RUN npm install

# Copy source (including ui/ subdirectory) and build
COPY src/client/*.ts src/client/*.json ./
COPY src/client/ui/*.ts ./ui/
RUN mkdir -p /static && npm run build

# Copy HTML and CSS to static
COPY src/static/*.html src/static/*.css /static/

# Build stage 2: Rust server
FROM rust:1.88-slim AS rust-builder
WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace Cargo.toml (without lock file initially)
COPY src/server/Cargo.toml ./

# Create directory structure for dependency caching
RUN mkdir -p crates/protocol/src crates/config/src crates/world/src \
    crates/systems/src crates/persistence/src crates/game/src \
    crates/net/src crates/assets/src crates/server/src

# Copy all Cargo.toml files for dependency resolution
COPY src/server/crates/protocol/Cargo.toml ./crates/protocol/
COPY src/server/crates/config/Cargo.toml ./crates/config/
COPY src/server/crates/world/Cargo.toml ./crates/world/
COPY src/server/crates/systems/Cargo.toml ./crates/systems/
COPY src/server/crates/persistence/Cargo.toml ./crates/persistence/
COPY src/server/crates/game/Cargo.toml ./crates/game/
COPY src/server/crates/net/Cargo.toml ./crates/net/
COPY src/server/crates/assets/Cargo.toml ./crates/assets/
COPY src/server/crates/server/Cargo.toml ./crates/server/

# Copy static assets into assets crate (for rust_embed)
COPY --from=node-builder /static ./crates/assets/static

# Create dummy lib.rs files for dependency caching
RUN echo "pub fn init() {}" > crates/protocol/src/lib.rs && \
    echo "pub fn init() {}" > crates/config/src/lib.rs && \
    echo "pub fn init() {}" > crates/world/src/lib.rs && \
    echo "pub fn init() {}" > crates/systems/src/lib.rs && \
    echo "pub fn init() {}" > crates/persistence/src/lib.rs && \
    echo "pub fn init() {}" > crates/game/src/lib.rs && \
    echo "pub fn init() {}" > crates/net/src/lib.rs && \
    echo "pub mod assets { use rust_embed::RustEmbed; #[derive(RustEmbed)] #[folder=\"static\"] pub struct StaticAssets; }" > crates/assets/src/lib.rs && \
    echo "fn main() {}" > crates/server/src/main.rs

# Generate Cargo.lock and build dependencies (cached layer)
RUN cargo generate-lockfile && cargo build --release -p jxwxmk-server 2>/dev/null || true

# Copy actual source code (this will overwrite the dummy files)
COPY src/server/crates ./crates

# Re-copy static assets (they might have been overwritten)
COPY --from=node-builder /static ./crates/assets/static

# Touch all source files to force rebuild with actual implementations
RUN touch crates/protocol/src/lib.rs && \
    touch crates/config/src/lib.rs && \
    touch crates/world/src/lib.rs && \
    touch crates/systems/src/lib.rs && \
    touch crates/persistence/src/lib.rs && \
    touch crates/game/src/lib.rs && \
    touch crates/net/src/lib.rs && \
    touch crates/assets/src/lib.rs && \
    touch crates/server/src/main.rs

# Build release binary
RUN cargo build --release -p jxwxmk-server

# Runtime stage: Debian with PostgreSQL
FROM debian:bookworm-slim

LABEL maintainer="JXWXMK Team"
LABEL description="JXWXMM Game Server"
LABEL version="0.1.0"

# Install PostgreSQL and runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    postgresql-15 \
    postgresql-client-15 \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Setup PostgreSQL
RUN mkdir -p /var/lib/postgresql/data && \
    chown -R postgres:postgres /var/lib/postgresql && \
    chmod 700 /var/lib/postgresql/data

ENV PGDATA=/var/lib/postgresql/data
ENV DATABASE_URL=postgres://postgres@localhost/jxwxmk
ENV RUST_LOG=info

# Copy server binary
COPY --from=rust-builder /build/target/release/jxwxmk-server /app/jxwxmk-server

# Copy entrypoint
COPY src/runtime/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

# Create app directory
WORKDIR /app

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Expose game server port
EXPOSE 8080

ENTRYPOINT ["/app/entrypoint.sh"]
