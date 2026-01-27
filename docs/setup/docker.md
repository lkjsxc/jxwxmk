# Docker Setup

Build and run the single-container runtime.

## Build

```bash
docker build -f src/runtime/Dockerfile -t kkmypk .
```

## Run

```bash
docker run --rm \
  -p 8080:8080 \
  -v kkmypk_pgdata:/var/lib/postgresql/data \
  -v ./config.json:/app/config.json \
  kkmypk
```

## Notes

- PostgreSQL runs inside the same container and is not exposed externally.
- The server reads `/app/config.json` at startup.
