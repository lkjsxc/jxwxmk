FROM node:20-slim AS frontend
WORKDIR /app/src/client
COPY src/client/package.json package.json
COPY src/client/tsconfig.json tsconfig.json
RUN npm install
COPY src/client .
COPY src/static ../static
RUN npm run build

FROM rust:1.88-slim AS builder
WORKDIR /app/src
COPY src/Cargo.toml Cargo.toml
COPY src/server/Cargo.toml server/Cargo.toml
COPY src/server/src server/src
COPY --from=frontend /app/src/static static
RUN cargo build --release -p jxwxmk

FROM rust:1.88-slim AS test
RUN apt-get update \
  && apt-get install -y --no-install-recommends postgresql-15 ca-certificates \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /app/src
COPY src/Cargo.toml Cargo.toml
COPY src/server/Cargo.toml server/Cargo.toml
COPY src/server/src server/src
COPY src/server/tests server/tests
COPY src/static static
ENV DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/jxwxmk
RUN mkdir -p /var/lib/postgresql/data \
  && chown -R postgres:postgres /var/lib/postgresql/data \
  && runuser -u postgres -- /usr/lib/postgresql/15/bin/initdb -D /var/lib/postgresql/data \
  && runuser -u postgres -- /usr/lib/postgresql/15/bin/pg_ctl -D /var/lib/postgresql/data -w start \
  && runuser -u postgres -- createdb jxwxmk \
  && cargo test -p jxwxmk \
  && runuser -u postgres -- /usr/lib/postgresql/15/bin/pg_ctl -D /var/lib/postgresql/data -m fast stop

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
  && apt-get install -y --no-install-recommends postgresql-15 ca-certificates \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/src/target/release/jxwxmk /app/jxwxmk
COPY src/runtime/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh
ENV DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/jxwxmk
EXPOSE 8080
ENTRYPOINT ["/app/entrypoint.sh"]
