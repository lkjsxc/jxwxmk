# Multi-stage Dockerfile for JXWXMK
# Stage 1: Node - Build client
# Stage 2: Rust - Build server
# Stage 3: Runtime - Debian + PostgreSQL

# Stage 1: Build client
FROM node:20-slim AS client-builder

WORKDIR /app

# Copy client source
COPY src/client/package.json src/client/package.json
COPY src/client/tsconfig.json src/client/tsconfig.json
COPY src/client/index.ts src/client/index.ts
COPY src/client/types src/client/types
COPY src/client/core src/client/core
COPY src/client/input src/client/input
COPY src/client/render src/client/render
COPY src/client/ui src/client/ui
COPY src/static/index.html src/static/index.html
COPY src/static/styles.css src/static/styles.css

# Install dependencies and build
RUN cd src/client && npm install && npm run build

# Stage 2: Build server
FROM rust:1.88-slim AS server-builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy workspace files
COPY src/server/Cargo.toml src/server/Cargo.toml
COPY src/server/src src/server/src
COPY src/server/protocol src/server/protocol
COPY src/server/config src/server/config
COPY src/server/world src/server/world
COPY src/server/systems src/server/systems
COPY src/server/game src/server/game
COPY src/server/persistence src/server/persistence
COPY src/server/net src/server/net
COPY src/server/assets src/server/assets
COPY src/server/jxwxmk-server src/server/jxwxmk-server

# Copy static assets from client builder
COPY --from=client-builder /app/src/static/game.js src/static/game.js
COPY --from=client-builder /app/src/static/index.html src/static/index.html
COPY --from=client-builder /app/src/static/styles.css src/static/styles.css

# Build release binary
RUN cd src/server && cargo build --release

# Stage 3: Runtime
FROM debian:bookworm-slim AS runtime

# Install PostgreSQL and dependencies
RUN apt-get update && apt-get install -y \
    postgresql-15 \
    postgresql-client-15 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy server binary
COPY --from=server-builder /app/src/server/target/release/jxwxmk-server /app/server

# Copy entrypoint script
COPY src/runtime/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

# Create postgres data directory
RUN mkdir -p /var/lib/postgresql/data && \
    chown -R postgres:postgres /var/lib/postgresql

# Expose game server port
EXPOSE 8080

# Set environment variables
ENV DATABASE_URL=postgres://postgres:postgres@localhost:5432/jxwxmk
ENV RUST_LOG=info

# Run entrypoint
ENTRYPOINT ["/app/entrypoint.sh"]
