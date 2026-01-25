# Stage 1: Frontend
FROM node:20-slim as frontend
WORKDIR /app
# Copy client source
COPY src/client ./src/client
WORKDIR /app/src/client
# Create output directory for esbuild (it writes to ../../static)
RUN mkdir -p ../../static
RUN npm install
RUN npm run build

# Stage 2: Backend Builder
FROM rust:latest as backend
# Install dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /app

# Build dependencies - this is the caching layer
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY src ./src

# Copy static assets from host (index.html, styles.css)
COPY static ./static

# Copy built assets from frontend (game.js)
COPY --from=frontend /app/static/game.js ./static/game.js

# Build final binary with embedded assets
# Touch main.rs to force rebuild
RUN touch src/main.rs
RUN cargo build --release

# Stage 2b: Test Runner
FROM backend as tester
COPY src ./src
# Need to rebuild/test with potentially different profile or just run tests
CMD ["cargo", "test"]

# Stage 3: Runtime
FROM debian:bookworm-slim as runtime
WORKDIR /app
# Install Postgres and runtime deps
RUN apt-get update && apt-get install -y libssl-dev ca-certificates postgresql sudo && rm -rf /var/lib/apt/lists/*

COPY --from=backend /app/target/release/kkmypk .
COPY config.json .
COPY .infra/start.sh .

ENV RUST_LOG=info
ENV DATABASE_URL=postgres://user:password@localhost:5432/kkmypk_db
EXPOSE 8080
CMD ["./start.sh"]
