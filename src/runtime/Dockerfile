# Stage 1: Frontend Build
FROM node:20-slim as frontend
WORKDIR /build
# Create the static directory structure so the build script can write to it
RUN mkdir -p src/static src/client
COPY src/client/package.json ./src/client/
WORKDIR /build/src/client
RUN npm install
COPY src/client ./
# This writes to ../static/game.js -> /build/src/static/game.js
RUN npm run build

# Stage 2: Backend Build
FROM rust:latest as backend
WORKDIR /app
COPY Cargo.toml ./
# Create a dummy main to cache deps? Skipping for now to avoid complexity.
COPY src ./src
# Overwrite the static asset with the built one
COPY --from=frontend /build/src/static/game.js ./src/static/game.js
RUN cargo build --release

# Stage 3: Runtime
FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y postgresql libpq-dev ca-certificates procps && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=backend /app/target/release/kkmypk ./kkmypk
COPY src/runtime/entrypoint.sh ./entrypoint.sh
RUN chmod +x entrypoint.sh

# Ensure config is available if mounted, but we can also copy a default
# COPY config.json ./config.json

EXPOSE 8080
CMD ["./entrypoint.sh"]
