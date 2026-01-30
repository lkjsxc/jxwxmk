# Reconstruction Report (Evidence Ledger)

This document is the **running evidence ledger** for rebuilding the full `src/` tree from the documentation in `docs/`.

It mirrors the section structure of `docs/implementation/reconstruction_acceptance.md`, but it is **not** a checklist and must not contain “unfinished work” task lists (those live only in `docs/implementation/todo/`).

## Backlog alignment

Canonical backlog (ordered): `docs/implementation/todo/README.md`

Slice mapping (foundations → operability):

- `01-foundation`: repo skeleton + crate boundaries (Acceptance A, K)
- `02-runtime`: Docker/runtime container wiring (Acceptance B)
- `03-config`: JSON configs + validation (Acceptance C)
- `04-backend`: HTTP/WS + protocol + static assets (Acceptance D)
- `05-game`: tick loop + chunk streaming (Acceptance E)
- `06-systems`: survival/crafting/etc. (Acceptance F)
- `07-persistence`: Postgres + sqlx + checkpointing (Acceptance G)
- `08-frontend`: TS client build + runtime + UI (Acceptance H)
- `09-tests`: unit + integration tests in Docker/Compose (Acceptance I)
- `10-ci`: GitHub Actions CI (Acceptance B, I)

## A) Repo + docs invariants

Spec docs:

- `docs/policy/INSTRUCT.md`
- `docs/implementation/reconstruction_acceptance.md`

Evidence (docs changes):

- Fixed protocol/UI naming drift (crafting recipe IDs) so `snake_case` IDs are consistent across protocol, config schema, and UI docs.
- Clarified protocol shapes and client input cadence based on reconstruction findings (see Acceptance D/H notes below).
- Removed stale “completion” artifacts that were not reachable via TOCs and conflicted with the docs-first workflow.
- Linked this report and the traceability matrix from `docs/implementation/README.md` to satisfy TOC reachability.

## B) Runtime container (single container = Rust server + PostgreSQL)

Spec docs:

- `docs/technical/deployment/README.md`
- `docs/setup/docker.md`
- `docs/setup/compose/README.md`

Expected implementation locations:

- `src/runtime/Dockerfile`
- `src/runtime/entrypoint.sh`
- `src/runtime/compose/`

Verification commands:

- `docker build -f src/runtime/Dockerfile -t jxwxmk .`
- `docker compose -f src/runtime/compose/docker-compose.yml up --build`

## C) Configuration (`config/*.json`)

Spec docs:

- `docs/technical/config/README.md`
- `docs/technical/config/files.md`
- `docs/technical/config/schemas/README.md`

Expected implementation locations:

- `config/*.json`
- `src/server/crates/config/`

Verification commands:

- Runtime startup must validate config and fail closed on schema violations.

## D) Backend HTTP + WebSocket (authoritative server)

Spec docs:

- `docs/technical/backend/server/README.md`
- `docs/technical/backend/server/http_ws.md`
- `docs/technical/backend/server/protocol.md`
- `docs/technical/contracts/protocol.md`

Evidence (docs changes):

- Added application-level keepalive and documented idle timeout controls in `server.json`.
- Clarified `playerUpdate` and `chunkAdd` shapes to remove client/server drift vectors.

Expected implementation locations:

- `src/server/crates/net/` (HTTP/WS)
- `src/server/crates/protocol/` (schema + validation)
- `src/server/crates/assets/` (static embed/serve)

Verification commands:

- `curl http://localhost:8080/health`
- `curl http://localhost:8080/metrics`

## E) Game simulation (tick loop + chunked world)

Spec docs:

- `docs/technical/backend/game/engine.md`
- `docs/technical/backend/game/world_state.md`
- `docs/technical/contracts/tick.md`

Evidence (docs changes):

- Expanded chunk streaming requirements: interest set vs loaded set and diff rules.

Expected implementation locations:

- `src/server/crates/game/` (tick owner + event queue)
- `src/server/crates/world/` (chunks/entities)

## F) Gameplay systems (server-authoritative)

Spec docs:

- `docs/technical/backend/game/systems_survival.md`
- `docs/technical/backend/game/systems_interaction.md`
- `docs/technical/backend/game/systems_crafting.md`
- `docs/technical/backend/game/death.md`
- `docs/technical/backend/game/barriers.md`

Expected implementation locations:

- `src/server/crates/systems/`

## G) Persistence (PostgreSQL + sqlx)

Spec docs:

- `docs/technical/backend/persistence/README.md`
- `docs/technical/backend/database/README.md`
- `docs/technical/backend/database/schema.md`

Expected implementation locations:

- `src/runtime/migrations/`
- `src/server/crates/persistence/`

## H) Frontend (Canvas renderer + input + UI)

Spec docs:

- `docs/technical/frontend/README.md`
- `docs/technical/frontend/build.md`
- `docs/technical/frontend/runtime.md`
- `docs/technical/frontend/input/README.md`
- `docs/technical/frontend/ui/README.md`

Evidence (docs changes):

- Fixed the crafting UI doc to use `snake_case` recipe IDs matching `crafting.json` and the `craft` protocol message.
- Clarified idle keepalive behavior and touch “gesture vs joystick” interpretation for mobile parity and stability.

Expected implementation locations:

- `src/client/` (TypeScript)
- `src/static/` (embedded output)

## I) Tests (Dockerized)

Spec docs:

- `docs/technical/testing/README.md`
- `docs/technical/testing/strategy.md`
- `docs/technical/testing/docker_commands.md`

Expected implementation locations:

- `src/server/**/tests` and/or `src/server/**/src/*` unit tests
- `src/runtime/compose/` test runner wiring (if used)

## J) Operability (logs, metrics, lifecycle)

Spec docs:

- `docs/technical/operability/README.md`
- `docs/technical/operability/logging.md`
- `docs/technical/operability/metrics.md`
- `docs/technical/operability/lifecycle.md`

Expected implementation locations:

- `src/server/crates/net/` (HTTP endpoints, metrics)
- `src/server/crates/game/` (tick overrun visibility)

## K) Modularity (enforced boundaries)

Spec docs:

- `docs/technical/module_map.md`
- `docs/technical/contracts/authority.md`

Expected implementation locations:

- Rust crate boundaries under `src/server/crates/` enforce forbidden edges at compile time.
