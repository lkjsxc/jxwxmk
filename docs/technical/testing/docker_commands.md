# Docker-first commands

These commands are the canonical way to build, run, and test.

## Build

```bash
docker build -f src/runtime/Dockerfile -t jxwxmk .
```

## Run (single container)

```bash
docker run --rm \
  -p 8080:8080 \
  -v jxwxmk_pgdata:/var/lib/postgresql/data \
  -v ./config:/app/config \
  jxwxmk
```

## Compose (optional convenience)

Compose YAML lives under `src/`:

- `src/runtime/compose/docker-compose.yml`
- `src/runtime/compose/docker-compose.rootless.yml`

Examples:

```bash
docker compose -f src/runtime/compose/docker-compose.yml up --build
```

```bash
docker compose -f src/runtime/compose/docker-compose.rootless.yml up --build
```
