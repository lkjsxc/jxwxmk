# Reconstruction Acceptance Criteria

This document defines what “**complete**” means for a full `src/` reconstruction from the documentation in `docs/`.
It exists to prevent partial implementations that “mostly compile” but omit entire documented subsystems.

Implementation tasks for reaching this state are tracked in:

- `docs/implementation/todo/README.md`

## Scope anchor

The reconstruction scope is defined by: `docs/implementation/reconstruction_scope.md`.

Anything explicitly marked **Planned / Out of scope (for initial reconstruction)** in that scope document is **not required** to satisfy this checklist.

## Definition of done (all boxes must be checked)

### A) Repo + docs invariants

- [ ] Obey `docs/policy/INSTRUCT.md` (root allowlist, 1 README per directory, Docker-first, etc.).
- [ ] Every directory under `docs/`, `config/`, and `src/` contains **exactly one** `README.md`.
- [ ] No placeholder markers in committed docs/code:
  - `TODO`, `TBD`, `stub`, `not implemented`, or placeholder headings
  - Exception: `docs/implementation/todo/` is the only allowed location for unfinished work.
- [ ] All documentation leaf files are reachable via TOCs (recursive README discipline).

### B) Runtime container (single container = Rust server + PostgreSQL)

References:
- `docs/technical/deployment/README.md`
- `docs/setup/docker.md`

- [ ] Multi-stage Docker build exists: Node (esbuild) → Rust build → Debian runtime.
- [ ] Runtime container starts PostgreSQL **inside the same container** and then launches the Rust server.
- [ ] PostgreSQL is not exposed externally (bind to `127.0.0.1:5432` inside container).
- [ ] The following commands succeed:
  - `docker compose -f src/runtime/compose/docker-compose.yml up --build`
  - `docker build -f src/runtime/Dockerfile -t jxwxmk .` (alternative path)
- [ ] `GET /health` returns `200 OK` with body `OK`.

### C) Configuration (`config/*.json`)

References:
- `docs/technical/config/README.md`
- `docs/technical/config/files.md`
- `docs/technical/config/schemas/README.md`

- [ ] `config/` exists and includes all files listed in `docs/technical/config/files.md`.
- [ ] Server loads all `*.json` at startup, validates them, and applies defaults for optional fields.
- [ ] Missing config files fall back to documented defaults (no crash-on-missing unless explicitly required).
- [ ] Every config file conforms to its documented schema under `docs/technical/config/schemas/`.
- [ ] Unknown fields are rejected (prevents silent typos and config drift).
- [ ] Config values are actually used by the server systems (not loaded-and-ignored).

### D) Backend HTTP + WebSocket (authoritative server)

References:
- `docs/technical/backend/server/README.md`
- `docs/technical/backend/server/http_ws.md`
- `docs/technical/backend/server/protocol.md`
- `docs/technical/backend/server/static_assets.md`
- `docs/technical/contracts/protocol.md`

- [ ] HTTP routes exist and match docs: `/health`, `/metrics`, `/session/claim`, `/` + `/{filename}`, `/ws?token=...`.
- [ ] Single-session enforcement works:
  - new `/session/claim` rotates token and revokes an existing live session (`sessionRevoked` then disconnect).
- [ ] Security headers are present as documented.
- [ ] All protocol messages listed in `docs/technical/backend/server/protocol.md` are implemented with strict JSON validation.
- [ ] Structured protocol errors are implemented:
  - invalid/rejected messages result in an `error` message (or a disconnect if abusive).
- [ ] Private player state is synchronized via `playerUpdate` (inventory, vitals, quests, achievements) and is never broadcast via public `entityDelta`.
- [ ] `input` targeting semantics are implemented as documented (`aim` required for actions; server validates and uses it).
- [ ] Rust server embeds `src/static/` with `rust-embed` and serves assets from memory.

### E) Game simulation (tick loop + chunked world)

References:
- `docs/technical/backend/game/README.md`
- `docs/technical/backend/game/engine.md`
- `docs/technical/backend/game/world_state.md`
- `docs/technical/module_map.md`
- `docs/technical/contracts/tick.md`
- Design anchors: `docs/design/world/README.md`, `docs/design/core_loop.md`

- [ ] Fixed tick loop exists (target 20–60Hz; configured by `server.tick_rate`).
- [ ] Single-writer rule is enforced: only the tick owner mutates world state; I/O only enqueues events.
- [ ] Tick backpressure is implemented (bounded queues; defined overflow behavior; visible via logs/metrics).
- [ ] Chunk streaming works end-to-end:
  - server maintains interest sets and sends `chunkAdd`/`chunkRemove`
  - server sends `entityDelta` updates scoped to chunk coords
  - client maintains a local chunk cache and applies deltas
- [ ] Villages/settlements exist as first-class world structures:
  - barrier core bounds/safe-zone
  - spawn/respawn association
  - ≥ 1 interaction surface (NPC trade, bulletin board, stash, station, etc.)

### F) Gameplay systems (server-authoritative)

References:
- `docs/technical/backend/game/systems_survival.md`
- `docs/technical/backend/game/systems_interaction.md`
- `docs/technical/backend/game/systems_crafting.md`
- `docs/technical/backend/game/spawning_and_ai.md`
- `docs/technical/backend/game/death.md`
- `docs/technical/backend/game/barriers.md`
- `docs/technical/backend/game/achievements.md`
- `docs/technical/backend/game/quests.md`

- [ ] Survival runs per tick for spawned players (hunger + temperature; thirst if enabled).
- [ ] Movement is applied per tick; stats update for achievements.
- [ ] Gather / combat / interaction paths exist and are validated server-side.
- [ ] Consume and structure placement are implemented (inventory consumption, `aim` targeting, and placement grid snapping/collision validation).
- [ ] Crafting consumes inventory materials and produces output items.
- [ ] Spawning keeps chunk-local budgets and respawn timers.
- [ ] Barrier safe-zone rules are enforced server-side (no PvP, hostile mobs handled, etc.).
- [ ] Death + respawn flow works (health <= 0 → unspawned → respawn at bound settlement).
- [ ] Achievements system is data-driven and grants XP/bonuses as configured.
- [ ] Quest system supports accept + progress + server-driven `questUpdate`.

### G) Persistence (PostgreSQL + sqlx)

References:
- `docs/technical/backend/persistence/README.md`
- `docs/technical/backend/database/README.md`
- `docs/technical/backend/database/schema.md`

- [ ] SQL migrations exist and match the canonical tables (players, settlements, chunks).
- [ ] Migrations apply at server startup inside the container.
- [ ] Player state loads on join and saves on disconnect + at interval.
- [ ] Chunk/settlement deltas checkpoint on an interval (never per tick).

### H) Frontend (Canvas renderer + input + UI)

References:
- `docs/technical/frontend/README.md`
- `docs/technical/frontend/build.md`
- `docs/technical/frontend/runtime.md`
- `docs/technical/frontend/input/README.md`
- `docs/technical/frontend/rendering/README.md`
- `docs/technical/frontend/ui/README.md`

- [ ] `src/client/` TypeScript sources exist and build via `esbuild` to `src/static/game.js`.
- [ ] Client connects to `/ws`, handles `welcome`, and performs the spawn flow.
- [ ] Client handles `playerUpdate` and uses it as the authoritative source for HUD/hotbar/inventory/profile/quests/achievements.
- [ ] Client maintains chunk cache and applies entity deltas.
- [ ] Canvas render loop works (camera + entity rendering).
- [ ] UI is present at minimum:
  - HUD (HP/hunger/temp)
  - hotbar slot selection
  - inventory view
  - crafting menu wired to `craft` messages
  - quests + achievements surfaces
  - notifications/toasts
  - session revoked overlay / login flow

### I) Tests (Dockerized)

References:
- `docs/policy/INSTRUCT.md` (Docker-first)
- `docs/technical/module_map.md` (testing seams)
- `docs/technical/testing/README.md`

- [ ] Unit tests cover deterministic logic (survival tick math, crafting consumption/outputs, barrier rules, respawn rules, etc.).
- [ ] Integration tests cover at minimum:
  - DB migrations apply successfully
  - session claim/token rotation + single-session enforcement
  - protocol handshake (welcome/spawn) roundtrip
- [ ] Integration tests cover:
  - structured error behavior for invalid messages
  - `/metrics` returns parsable Prometheus text
- [ ] Tests run in Docker or Docker Compose (no host-only test path).
- [ ] A single-command Docker Compose test runner exists and succeeds:
  - `docker compose -f src/runtime/compose/docker-compose.test.yml up --build --abort-on-container-exit --exit-code-from test`

### J) Operability (logs, metrics, lifecycle)

References:
- `docs/technical/operability/README.md`

- [ ] Logs are structured and include key context for debugging (player/session, error code, tick overruns).
- [ ] `/metrics` exports bounded, low-cardinality metrics (no UUID labels) and includes the required metric names from `docs/technical/operability/metrics.md`.
- [ ] Server startup and shutdown follow the documented lifecycle (migrations at startup; graceful shutdown flushes checkpoints).

### K) Modularity (enforced boundaries)

References:
- `docs/technical/module_map.md`
- `docs/technical/contracts/authority.md`

- [ ] Dependency rules are enforced by structure (preferred: crate boundaries) so forbidden edges do not compile (e.g., `net` cannot import `world`).
- [ ] Framework types do not leak into core domain modules (`world`/`systems` are free of Actix/DB types).

## Required evidence in an agent final report

If you are an agent performing reconstruction, your final report must include:

- A checklist copy of the sections above with every box checked.
- A traceability summary:
  - which docs were implemented in which code locations
  - which tests cover which subsystems
