FROM rust:1.76-bookworm AS builder

WORKDIR /workspace
COPY src/server ./src/server
RUN cargo build --release --manifest-path src/server/Cargo.toml

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends postgresql \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /workspace/src/server/target/release/kkmypk-server /app/kkmypk-server
COPY src/server/static /app/static
COPY src/runtime/entrypoint.sh /app/entrypoint.sh

RUN chmod +x /app/entrypoint.sh

ENV APP_BIND=0.0.0.0:8080
ENV DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/kkmypk
ENV TICK_HZ=20

WORKDIR /app
EXPOSE 8080

ENTRYPOINT ["/app/entrypoint.sh"]
