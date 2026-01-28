# Dockerfile

FROM node:20-slim AS frontend-builder
WORKDIR /app/client
COPY src/client/package*.json ./
RUN npm install
COPY src/client/ .
# Expects build script to output to ../static/game.js
RUN npm run build

FROM rustlang/rust:nightly AS backend-builder
WORKDIR /app
COPY . .
# Copy compiled frontend assets into the source tree for embedding
COPY --from=frontend-builder /app/static/game.js src/static/game.js
WORKDIR /app/src
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Install PostgreSQL 15 and runtime dependencies
RUN apt-get update && \
    apt-get install -y postgresql-15 postgresql-client-15 libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Setup PostgreSQL
ENV PGDATA=/var/lib/postgresql/data
RUN mkdir -p "$PGDATA" && chown -R postgres:postgres "$PGDATA"

# Copy binary
COPY --from=backend-builder /app/src/target/release/server /app/server
# Copy config (defaults)
COPY config /app/config
# Copy entrypoint
COPY src/runtime/entrypoint.sh /app/entrypoint.sh

RUN chmod +x /app/entrypoint.sh

# Expose HTTP/WS port
EXPOSE 8080

ENTRYPOINT ["/app/entrypoint.sh"]