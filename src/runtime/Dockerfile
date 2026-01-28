# Stage 1: Frontend Build
FROM node:20-alpine AS frontend
WORKDIR /app
# Install esbuild globally for simplicity in this generated build
RUN npm install -g esbuild
COPY src/client ./src/client
COPY src/static ./src/static
# Bundle the client
RUN esbuild src/client/index.ts --bundle --outfile=src/static/game.js --minify --target=es2020

# Stage 2: Backend Build
FROM rust:bookworm AS backend
WORKDIR /app
COPY Cargo.toml .
# Create a dummy main to cache dependencies
RUN mkdir -p src/server
RUN echo "fn main() {}" > src/server/main.rs
RUN cargo build --release || true
# Now copy the real source
COPY src/server ./src/server
COPY --from=frontend /app/src/static ./src/static
# Touch main.rs to force rebuild
RUN touch src/server/main.rs
RUN cargo build --release

# Stage 3: Runtime
FROM debian:bookworm-slim
# Install Postgres and dependencies
RUN apt-get update && apt-get install -y postgresql postgresql-contrib libssl-dev sudo && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=backend /app/target/release/server /app/server
COPY src/runtime/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

# Environment defaults
ENV DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/jxwxmk
ENV RUST_LOG=info

EXPOSE 8080
ENTRYPOINT ["/app/entrypoint.sh"]
