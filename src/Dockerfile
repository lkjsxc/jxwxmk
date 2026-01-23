FROM rust:1.72 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /app/target/release/starvers /usr/local/bin/starvers
EXPOSE 8080
CMD ["/usr/local/bin/starvers"]
