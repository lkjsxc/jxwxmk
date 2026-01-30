# Docker Compose Examples

Example compose files for different deployment scenarios.

## Files

- `docker-compose.yml`: Default build-from-source
- `docker-compose.build.yml`: Explicit build variant
- `docker-compose.image.yml`: Prebuilt image variant
- `docker-compose.rootless.yml`: Rootless variant with local bind mount

## Usage

```bash
docker compose -f src/runtime/compose/docker-compose.yml up
```
