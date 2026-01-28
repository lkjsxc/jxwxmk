# Runtime

Docker runtime configuration for the single-container deployment.

## Files

- `Dockerfile` - Multi-stage build (Node → Rust → Runtime)
- `entrypoint.sh` - Container entrypoint (starts PostgreSQL + server)
- `compose/` - Docker Compose examples

## Runtime Contract

The runtime container runs **both** PostgreSQL and the Rust server:

1. PostgreSQL binds to `127.0.0.1:5432` (internal only)
2. Rust server binds to `0.0.0.0:8080` (exposed)
3. Server applies DB migrations on startup
4. Graceful shutdown on SIGTERM

## Environment Variables

- `DATABASE_URL` - PostgreSQL connection string (default: `postgres://postgres:postgres@localhost:5432/jxwxmk`)
- `RUST_LOG` - Logging level (default: `info`)

## Build

```bash
docker build -f src/runtime/Dockerfile -t jxwxmk .
```

## Run

```bash
docker run --rm -p 8080:8080 \
  -v jxwxmk_pgdata:/var/lib/postgresql/data \
  -v ./config:/app/config:ro \
  jxwxmk
```
