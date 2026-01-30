# Docker Setup

Build and run the single-container runtime.

## Compose (preferred)

```bash
docker compose -f src/runtime/compose/docker-compose.yml up --build
```

## Direct Docker (alternatives)

Build:

```bash
docker build -f src/runtime/Dockerfile -t jxwxmk .
```

Run:

```bash
docker run --rm \
  -p 8080:8080 \
  -v jxwxmk_pgdata:/var/lib/postgresql/data \
  -v ./config:/app/config \
  jxwxmk
```

## Compose Examples

- Build from source: `src/runtime/compose/docker-compose.build.yml`
- Run a prebuilt tag: `src/runtime/compose/docker-compose.image.yml`

## Notes

- PostgreSQL runs inside the same container and is not exposed externally.
- The server reads `/app/config/*.json` at startup.
