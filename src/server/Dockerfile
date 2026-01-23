FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libpq5 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/starvers_server /usr/local/bin/starvers_server
EXPOSE 8080
CMD ["/usr/local/bin/starvers_server"]
