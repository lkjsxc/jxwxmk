# Deployment

## Strategy
Single runtime container that runs **both** the Rust game server and PostgreSQL.

## Networking
- The game server binds to `0.0.0.0:8080` (exposed).
- PostgreSQL listens on `127.0.0.1:5432` inside the same container (not exposed).
- The server connects via `localhost`.

## Runtime Process Model
- One container process supervisor starts Postgres and the game server.
- The world simulation is authoritative and tick-owned; I/O handlers enqueue events.

## Containers
- **Single container**: Rust binary + embedded static assets + PostgreSQL 16.