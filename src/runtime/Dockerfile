# Stage 1: Frontend Build
FROM node:18-alpine AS frontend
WORKDIR /app/client
COPY src/client/package.json .
RUN npm install
COPY src/client/ .
RUN npm run build

# Stage 2: Backend Build
FROM rust:1.75-bullseye AS backend
WORKDIR /app
# Copy manifest first for caching (optional optimization, skipping for simplicity in reconstruction)
COPY src/server/Cargo.toml src/server/Cargo.toml
COPY src/server/src src/server/src
# Wait, struct is src/server/main.rs etc. 
# My local struct is src/server/main.rs. 
# I need to be careful with paths.
# The context will be the repo root.
COPY src/server src/server
COPY src/static src/static
# Copy built frontend assets
COPY --from=frontend /app/static/game.js src/static/game.js

WORKDIR /app/src/server
# We need to make sure src/static is accessible to rust-embed.
# If I build from src/server, ../static might be needed?
# Or I can just build from /app/src/server.
RUN cargo build --release

# Stage 3: Runtime
FROM debian:bullseye-slim
WORKDIR /app

# Install PostgreSQL
RUN apt-get update && apt-get install -y postgresql postgresql-contrib && rm -rf /var/lib/apt/lists/*

# Copy artifact
COPY --from=backend /app/src/server/target/release/server /app/server
# Copy config (will be mounted, but maybe copy defaults if any?)
# Configs are expected at /app/config. We don't copy them here as they are volume mounted or copied from source.
# Docs say "The runtime container mounts /app/config". 
# But for the image itself to work without mount (using defaults), we might want to copy them.
# "The server loads all *.json files in that directory at startup."
# I will create the directory.
RUN mkdir -p /app/config

# Copy entrypoint
COPY src/runtime/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

# Environment
ENV DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/jxwxmk
ENV PORT=8080

EXPOSE 8080

CMD ["/app/entrypoint.sh"]
