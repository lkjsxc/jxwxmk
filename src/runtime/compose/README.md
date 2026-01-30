# Docker Compose Examples

Example compose files for different deployment scenarios.

## Files

- `docker-compose.yml`: Default build-from-source
- `docker-compose.build.yml`: Explicit build variant
- `docker-compose.image.yml`: Prebuilt image variant
- `docker-compose.rootless.yml`: Rootless variant with local bind mount
- `docker-compose.test.yml`: Unit + integration test runner (single command)

## Usage

```bash
docker compose -f src/runtime/compose/docker-compose.yml up
```

Run tests:

```bash
docker compose -f src/runtime/compose/docker-compose.test.yml up --build --abort-on-container-exit --exit-code-from test
```
