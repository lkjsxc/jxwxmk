# Docker Configuration

## Base Dockerfile

```dockerfile
# Multi-stage build

# Stage 1: Build frontend
FROM node:18-alpine as builder
WORKDIR /app
COPY src/client/package*.json ./
RUN npm install
COPY src/client ./
RUN npm run build

# Stage 2: Build Rust server
FROM rust:1.70 as rust-builder
WORKDIR /app
COPY . ./
RUN cargo build --release

# Stage 3: Final image
FROM debian:bullseye-slim
WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ca-certificates

# Copy assets
COPY --from=builder /app/dist /app/static
COPY --from=rust-builder /app/target/release/server /app/server
COPY .env ./
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

USER gameuser
EXPOSE 8080 8081
ENTRYPOINT ["/entrypoint.sh"]
```

## Development Dockerfile

```dockerfile
FROM rust:1.70
WORKDIR /app

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
&& apt-get install -y nodejs

# Copy and build
COPY . ./
WORKDIR /app/src/client
RUN npm install
WORKDIR /app
RUN cd src/client && npm run dev &

EXPOSE 8080 8081 3000
CMD ["cargo", "run"]
```

## Entrypoint Script

```bash
#!/bin/sh
set -e

# Wait for database
until pg_isready -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER"; do
  echo "Waiting for database..."
  sleep 1
done

# Run migrations
# Implement migration runner

# Start server
exec "./server"
```