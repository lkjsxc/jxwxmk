# syntax=docker/dockerfile:1

FROM node:20-bullseye AS client-build
WORKDIR /app
COPY src /app
COPY docs/assets/favicon.ico /app/static/favicon.ico
RUN cd /app/client && npm install && npm run build

FROM rust:1.75-bullseye AS rust-build
WORKDIR /app
COPY src /app
COPY --from=client-build /app/static /app/static
RUN cargo build --release

FROM rust:1.75-bullseye AS test
WORKDIR /app
COPY src /app
COPY --from=client-build /app/static /app/static
RUN apt-get update && apt-get install -y postgresql-15 && rm -rf /var/lib/apt/lists/*
ENV DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/jxwxmk
RUN service postgresql start \
  && su - postgres -c "psql -c \"ALTER USER postgres PASSWORD 'postgres';\"" \
  && su - postgres -c "createdb jxwxmk" \
  && cargo test

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y postgresql-15 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=rust-build /app/target/release/jxwxmk /app/server
COPY src/runtime/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh
ENV DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/jxwxmk
EXPOSE 8080
ENTRYPOINT ["/app/entrypoint.sh"]
