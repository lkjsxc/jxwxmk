FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml ./
RUN if [ -f Cargo.lock ]; then cp Cargo.lock ./; else touch Cargo.lock; fi
COPY src/backend/ ./

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/starve_game /usr/local/bin/

EXPOSE 8080

CMD ["starve_game"]