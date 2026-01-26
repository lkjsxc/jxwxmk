# --- Frontend Build ---
FROM node:18-slim AS frontend-builder
WORKDIR /build/client
COPY src/client/package*.json ./
RUN npm install
COPY src/client/ ./
COPY src/static/ ../static/
RUN npm run build

# --- Backend Build ---
FROM rust:1-bookworm AS backend-builder
WORKDIR /build
# Copy the entire src for Rust build
COPY src/ ./
# Copy built frontend assets from previous stage
COPY --from=frontend-builder /build/static/ ./static/
RUN cargo build --release --manifest-path Cargo.toml

# --- Runtime ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    postgresql \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

ENV PATH="/usr/lib/postgresql/15/bin:${PATH}"

WORKDIR /app
COPY --from=backend-builder /build/target/release/kkmypk /app/kkmypk
COPY config.json /app/config.json
COPY src/runtime/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

# PostgreSQL data volume
VOLUME /var/lib/postgresql/data

EXPOSE 8080
ENTRYPOINT ["/app/entrypoint.sh"]
