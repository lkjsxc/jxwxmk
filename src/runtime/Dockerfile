# Stage 1: Frontend
FROM node:20-slim as frontend
WORKDIR /app/src/client
COPY src/client/package.json ./
RUN npm install
COPY src/client ./
RUN mkdir -p ../static
RUN npm run build

# Stage 2: Backend
FROM rust:1.84-slim-bookworm as backend
WORKDIR /app
COPY src/Cargo.toml ./src/
COPY src/crates ./src/crates/
COPY src/bin ./src/bin/
COPY src/static ./src/static/
# Copy built frontend assets
COPY --from=frontend /app/src/static/game.js ./src/static/
COPY config ./config

WORKDIR /app/src
RUN cargo build --release --bin server

# Stage 3: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y postgresql-15 postgresql-client-15 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=backend /app/config ./config
COPY --from=backend /app/src/target/release/server ./server
COPY src/runtime/entrypoint.sh ./entrypoint.sh
RUN chmod +x ./entrypoint.sh

ENV DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/jxwxmk
ENV RUST_LOG=info

EXPOSE 8080

CMD ["./entrypoint.sh"]
