# Docker Setup

We run a **single container** that includes both the Rust server and PostgreSQL.

## Build

```bash
docker build -f src/runtime/Dockerfile -t kkmypk .
```

## Run

```bash
docker run --rm \
    -p 8080:8080 \
    -e DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/kkmypk \
    -e APP_BIND=0.0.0.0:8080 \
    -e TICK_HZ=20 \
    -v kkmypk_pgdata:/var/lib/postgresql/data \
    kkmypk
```

## Notes

- PostgreSQL is **not** exposed to the host.
- The server connects to the local DB via `127.0.0.1`.
