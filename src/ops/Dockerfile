# Build stage for TypeScript client
FROM node:18 AS ts-build
WORKDIR /app/client
COPY ../src/client/ ./
RUN npm install && npm run build  # Assume build script compiles TS to JS/CSS

# Build stage for Rust server
FROM rust:latest AS rust-build
WORKDIR /app
ENV DATABASE_URL=postgresql://dummy
COPY ../Cargo.toml ./
COPY ../src/ ./src/
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=rust-build /app/target/release/starve-game ./starve-game
COPY --from=ts-build /app/client/dist/* ./static/
EXPOSE 8080
CMD ["./starve-game"]