# 09 — Tests (Dockerized unit + integration)

Goal: make reconstruction “done” only when tests pass in Docker/Compose.

References:
- `docs/policy/INSTRUCT.md` (Docker-first tests)
- `docs/technical/module_map.md` (testing seams)
- `docs/technical/testing/README.md`
- `docs/implementation/reconstruction_acceptance.md` (test acceptance)
- `docs/decisions/0006-player-state-message.md` (`playerUpdate`)

## A) Test harness (Dockerized)

- [ ] Provide a Docker command path that runs:
  - unit tests
  - integration tests (including DB + migrations)
- [ ] Prefer running tests using the runtime image + an internal Postgres (single container rule).
- [ ] If an additional test-only container or compose file is needed, keep it under `src/` (not `docs/`).

## B) Unit tests (deterministic)

Add deterministic unit tests for at least:

- [ ] Survival tick math (hunger decay, heal threshold, freeze damage).
- [ ] Crafting (ingredient consumption + output insertion).
- [ ] Placement validation (grid snapping + collision checks).
- [ ] Barrier safe-zone rules (PvP disabled; hostile handling).
- [ ] Death + respawn cooldown rules.

## C) Integration tests (containerized)

- [ ] DB migrations apply successfully.
- [ ] Session claim rotates token and revokes existing session.
- [ ] WebSocket handshake works (`welcome` + `spawn` flow).
- [ ] `playerUpdate` is delivered after spawn and reflects private state changes (inventory, active slot).
- [ ] Config loading works with only `/app/config/*.json`.
- [ ] `/metrics` returns parsable Prometheus text.
- [ ] `/metrics` contains required metric names from `docs/technical/operability/metrics.md`.
- [ ] Invalid WS messages yield a structured `error` (or disconnect if abusive), without mutating world state.
- [ ] Validation errors use stable `error.code` values from `docs/technical/contracts/protocol.md`.

## Done when

- [ ] The Docker test command(s) pass consistently.
