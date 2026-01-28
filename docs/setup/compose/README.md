# Docker Compose Examples

Compose files for running the single-container runtime locally.

Compose YAML is kept under `src/` (not under `docs/`). See: `src/runtime/compose/README.md`.

## Files (in `src/runtime/compose/`)

- `docker-compose.yml`: regular build-from-source compose file.
- `docker-compose.build.yml`: builds from source and runs the container.
- `docker-compose.image.yml`: uses a prebuilt image tag.
- `docker-compose.rootless.yml`: rootless-friendly build-from-source file using a bind mount for PG data.

The rootless example uses `./.local/pgdata` under the repo root to keep ownership aligned with the host user.
