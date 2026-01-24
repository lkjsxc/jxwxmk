# Stage 1: Frontend
FROM node:20-slim as frontend
WORKDIR /app
COPY src/client ./src/client
WORKDIR /app/src/client
# Create output dir
RUN mkdir -p ../static
RUN npm install
RUN npm run build

# Stage 2: Backend
FROM rust:latest as backend
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /app
COPY Cargo.toml ./
# Create dummy main to build deps layer
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy real source
COPY src ./src
# Copy frontend artifacts to where backend expects them (if embedding) or just for next stage
COPY --from=frontend /app/src/static ./src/static

# Build final binary
# Touch main.rs to ensure rebuild
RUN touch src/main.rs
RUN cargo build --release

# Stage 3: Runtime
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=backend /app/target/release/starve_clone .
# Copy static files
COPY --from=frontend /app/src/static ./static
COPY src/client/index.html ./static/index.html
COPY src/client/styles.css ./static/styles.css

ENV RUST_LOG=info
EXPOSE 8080
CMD ["./starve_clone"]
