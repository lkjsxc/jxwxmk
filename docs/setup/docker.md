# Docker Setup

Build and run the single-container runtime.

## Build

```bash
docker build -f src/runtime/Dockerfile -t jxwxmk .
```

## Run

```bash
docker run --rm \
  -p 8080:8080 \
  -v jxwxmk_pgdata:/var/lib/postgresql/data \
  -v ./config:/app/config \
  jxwxmk
```

## Notes

- PostgreSQL runs inside the same container and is not exposed externally.
- The server reads `/app/config/*.json` at startup.
