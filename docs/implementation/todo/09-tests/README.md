# 09 — Tests (Dockerized unit + integration)

Goal: make reconstruction "done" only when tests pass in Docker/Compose.

References:
- `docs/policy/INSTRUCT.md` (Docker-first tests)
- `docs/technical/module_map.md` (testing seams)
- `docs/technical/testing/README.md`
- `docs/implementation/reconstruction_acceptance.md` (test acceptance)
- `docs/technical/backend/server/protocol.md` (`playerUpdate`)

## A) Test harness (Dockerized)

- [x] Provide a Docker command path that runs:
  - unit tests
  - integration tests (including DB + migrations)
- [x] Provide a **single-command** test runner via Docker Compose:
  - `docker compose -f src/runtime/compose/docker-compose.test.yml up --build --abort-on-container-exit --exit-code-from test`
- [x] Prefer running tests using the runtime image + an internal Postgres (single container rule).
- [x] If an additional test-only container or compose file is needed, keep it under `src/` (not `docs/`).

## B) Unit tests (deterministic)

Add deterministic unit tests for at least:

- [x] Survival tick math (hunger decay, heal threshold, freeze damage).
- [x] Crafting (ingredient consumption + output insertion).
- [x] Placement validation (grid snapping + collision checks).
- [x] Barrier safe-zone rules (PvP disabled; hostile handling).
- [x] Death + respawn cooldown rules.

## C) Integration tests (containerized)

- [x] DB migrations apply successfully.
- [x] Session claim rotates token and revokes existing session.
- [x] WebSocket handshake works (`welcome` + `spawn` flow).
- [x] `playerUpdate` is delivered after spawn and reflects private state changes (inventory, active slot).
- [x] Config loading works with only `/app/config/*.json`.
- [x] `/metrics` returns parsable Prometheus text.
- [x] `/metrics` contains required metric names from `docs/technical/operability/metrics.md`.
- [x] Invalid WS messages yield a structured `error` (or disconnect if abusive), without mutating world state.
- [x] Validation errors use stable `error.code` values from `docs/technical/contracts/protocol.md`.

## Done when

- [x] The Docker test command(s) pass consistently.
