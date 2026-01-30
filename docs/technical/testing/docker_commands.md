# Docker-first commands

These commands are the canonical way to build, run, and test.

## Compose (canonical)

```bash
docker compose -f src/runtime/compose/docker-compose.yml up --build
```

## Single-command test suite (compose)

```bash
docker compose -f src/runtime/compose/docker-compose.test.yml up --build --abort-on-container-exit --exit-code-from test
```

Notes:

- Compose YAML lives under `src/` (not under `docs/`).
- The test compose file exists to keep “run the whole suite” to a single command (agent-cost friendly).

## Direct Docker (alternatives)

Build:

```bash
docker build -f src/runtime/Dockerfile -t jxwxmk .
```

Run (single container):

```bash
docker run --rm \
  -p 8080:8080 \
  -v jxwxmk_pgdata:/var/lib/postgresql/data \
  -v ./config:/app/config \
  jxwxmk
```

## Compose variants

- `src/runtime/compose/docker-compose.yml`
- `src/runtime/compose/docker-compose.rootless.yml`
