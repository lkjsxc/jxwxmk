# Docker Compose Examples

Docker Compose configurations for different deployment scenarios.

## Files

- `docker-compose.yml` - Build-from-source baseline
- `docker-compose.build.yml` - Explicit build variant
- `docker-compose.image.yml` - Prebuilt image variant
- `docker-compose.rootless.yml` - Rootless variant with bind mount

## Usage

```bash
# Build and run from source
docker-compose -f src/runtime/compose/docker-compose.yml up

# Use prebuilt image
docker-compose -f src/runtime/compose/docker-compose.image.yml up

# Rootless (uses local bind mount for data)
docker-compose -f src/runtime/compose/docker-compose.rootless.yml up
```

## Volume Mounts

All compose files mount:
- `./config` → `/app/config` (read-only)
- Named volume or bind mount for PostgreSQL data
