# 02 — Runtime (Docker + entrypoint + compose)

Goal: deliver the single-container runtime contract: Rust server + PostgreSQL in one container.

References:
- `docs/policy/INSTRUCT.md` (single runtime container; Docker-first)
- `docs/technical/deployment/README.md` (build stages)
- `docs/setup/docker.md` (run command)
- `docs/technical/backend/server/overview.md` (bind `0.0.0.0:8080`)
- `docs/technical/backend/database/README.md` (default DB URL)
- `docs/technical/operability/lifecycle.md` (startup/shutdown contract)

## A) Multi-stage Docker build

- [ ] Implement `src/runtime/Dockerfile` with three stages:
  - Node stage: run `npm ci` + `npm run build` in `src/client/` → outputs `src/static/game.js`.
  - Rust stage: `cargo build --release` for the server crate (located under `src/`).
  - Runtime stage: Debian slim + PostgreSQL 15 installed + server binary copied in.
- [ ] Ensure no runtime Node process exists (Node is build-time only).
- [ ] Ensure the Rust build embeds `src/static/` via `rust-embed`.

## B) Runtime entrypoint (Postgres inside the same container)

- [ ] Implement `src/runtime/entrypoint.sh` that:
  - initializes the Postgres data directory when empty
  - starts Postgres bound to `127.0.0.1:5432`
  - launches the Rust server (which applies migrations on startup)
- [ ] Ensure the entrypoint handles SIGTERM/SIGINT:
  - forwards the signal to the Rust server
  - waits for graceful shutdown (final checkpoint)
  - stops Postgres cleanly
- [ ] Ensure Postgres is not exposed externally (no `0.0.0.0:5432`).
- [ ] Document runtime environment variables supported (at minimum: `DATABASE_URL`).

## C) docker-compose examples (live in `src/`, not `docs/`)

Policy note: compose YAML must live under `src/` (not under `docs/`).

- [ ] Create `src/runtime/compose/docker-compose.yml` (build-from-source baseline).
- [ ] Create `src/runtime/compose/docker-compose.build.yml` (explicit build variant).
- [ ] Create `src/runtime/compose/docker-compose.image.yml` (prebuilt image tag variant).
- [ ] Create `src/runtime/compose/docker-compose.rootless.yml`:
  - uses a bind mount for PG data at `./.local/pgdata` under repo root
  - does not rely on a named Docker volume
- [ ] Ensure compose files mount `./config` into `/app/config` (read-only is preferred).

## Done when

- [ ] `docker build -f src/runtime/Dockerfile -t jxwxmk .` succeeds.
- [ ] Running the container and hitting `/health` works (see: `docs/implementation/reconstruction_acceptance.md`).
- [ ] Running the container and hitting `/metrics` works (Prometheus text).
