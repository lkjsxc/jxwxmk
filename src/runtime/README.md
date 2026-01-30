# Runtime

Docker runtime configuration and entrypoint scripts.

## Files

- `Dockerfile`: Multi-stage build (Node → Rust → Runtime)
- `entrypoint.sh`: Container startup script (starts Postgres, then server)
- `compose/`: Docker Compose example files
- `migrations/`: SQL migrations applied at startup

## Container Architecture

The runtime container runs both PostgreSQL and the Rust game server:

1. Entrypoint initializes Postgres data directory if empty
2. Starts Postgres bound to 127.0.0.1:5432 (internal only)
3. Applies SQL migrations
4. Starts the Rust game server on 0.0.0.0:8080
5. Handles graceful shutdown on SIGTERM
