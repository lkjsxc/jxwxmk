# Docker Setup

We use Docker Compose to run the entire stack.

## Services

1.  **`app`**: The Rust application.
    - Builds from `Dockerfile`.
    - Exposes port 8080.
    - Depends on `db`.
2.  **`db`**: PostgreSQL database.
    - Uses official `postgres` image.
    - Persists data to a volume.

## Commands

```bash
# Start everything
docker-compose up --build

# Stop everything
docker-compose down
```

## Development

For local dev (outside docker):
1.  Start DB: `docker-compose up db -d`
2.  Run App: `cargo run`
