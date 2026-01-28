# Module Map (Scalability + Modularity)

This document defines **module boundaries and dependency rules** to keep the codebase scalable as it grows. It is a structural guide for organizing `src/` once implementation/reconstruction happens.

## Goals

- Scale features without adding new runtime services (still **one container**: Rust server + Postgres).
- Keep the server authoritative and deterministic-ish (fixed tick; single world-state writer).
- Make subsystems independently understandable, testable, and replaceable.

## Domain Modules (Conceptual)

These are **domains**, not necessarily exact filesystem paths, but the names should map cleanly to a deep `src/` tree.

- **`protocol`**: WebSocket/HTTP message shapes, versioning, validation, and limits.
- **`net`**: HTTP + WebSocket plumbing, auth/session middleware, rate limiting; **never mutates world state**.
- **`game`**: fixed tick loop + bounded event queues; owns the authoritative `World` and is the **only writer**.
- **`world`**: chunk storage, deterministic generation helpers, entity data structures, spatial queries.
- **`systems`**: deterministic gameplay systems (survival, combat, crafting, spawning, barriers, quests, etc.); invoked by the tick loop.
- **`persistence`**: DB schema/migrations + checkpoint/readback of player + world state; **no per-tick writes**.
- **`config`**: load/validate JSON config; produce typed configs used by `game`/`systems`.
- **`assets`**: build-time client assets + embedding/serving static files.
- **`runtime`**: container entrypoint and operational wiring (start Postgres, run server).

## Enforcement (make boundaries real)

The rules above are only effective if the code structure makes violations hard or impossible.

Recommended enforcement approach:

1. **Separate crates for boundary modules** (preferred).
2. If a single crate is used early, simulate boundaries with:
   - `pub(crate)` visibility
   - “ports” traits in narrow modules
   - no cross-module `use` of implementation types

### Recommended crate graph (Rust)

Organize the Rust code as a workspace under `src/` (do not require root-level Cargo files).

Suggested layout (example):

- `protocol` (no dependencies on any other project crate)
- `config` → `protocol` (for shared ID types/enums only; avoid pulling in net)
- `world` → `config`
- `systems` → `world` + `config`
- `game` → `systems` + `world` + `config` + `protocol` + `persistence` (engine orchestrator)
- `persistence` → `world` + `protocol` + `config` (DB adapter; no `net`)
- `net` → `protocol` + `game` (adapter; no `world`/`systems`/`persistence`)
- `assets` (embedding + static responses; avoid `world`/`systems`)

The binary crate composes adapters + engine but should not become a “god module”.

### Adapter boundaries (ports)

Avoid passing large structs like `World` across module boundaries.
Instead expose **narrow handles**:

- `game` exposes a `GameHandle` for:
  - enqueueing validated events
  - subscribing to outbound deltas/messages
- `persistence` exposes a `PersistenceHandle` for:
  - loading player/world snapshots
  - checkpointing dirty state

This implements the “ports and adapters” contract (see: `docs/technical/contracts/authority.md`).

## Dependency Rules (Hard)

To preserve modularity, treat these as “build-breaking” constraints:

- `net` → `protocol` (decode/validate) → enqueue events into `game`
- `net` must not depend on `world` or `systems` (prevents authority leakage).
- `game` may depend on: `protocol` (types), `world`, `systems`, `config`, `persistence`.
- `systems` must not depend on `net` (pure gameplay; no sockets, no HTTP).
- `persistence` must not depend on `net` (DB is an implementation detail of the server, not the network).
- `assets` is served by the Rust server, but the client is still “dumb”: renderer + input.

## Ownership Model

- **Single writer**: only the tick-owner (`game`) mutates `World`.
- **I/O as messages**: network handlers validate input and enqueue events.
- **Deterministic seams**:
  - `systems` operate on explicit inputs: `dt`, typed config, and world state.
  - randomness affecting simulation uses a seeded RNG stream owned by the tick loop.

## Boundary hygiene rules (modern baseline)

- Do not leak framework types across boundaries:
  - `systems`/`world` must not depend on Actix types, WebSocket types, or SQL client types.
- Centralize validation in `protocol`/`net` so simulation code can assume invariants.
- Use stable IDs (`snake_case` for string IDs; UUIDs for identity) and document them (see: `docs/technical/contracts/world_space.md`).
- Prefer explicit error codes at boundaries (see: `docs/technical/contracts/protocol.md`).

## Scaling Patterns

- Prefer **deep trees** and small files (≤ ~200 LoC) over large modules.
- Add features as **vertical slices**:
  1) protocol type + validation
  2) enqueue + engine event
  3) system logic + tests
  4) persistence (if required) + integration test
  5) minimal client wiring (render + input only)
  6) docs + TOC updates

## Testing Seams (What to Unit-Test)

- `systems/*`: deterministic unit tests (crafting tables, combat resolution, respawns, barrier rules).
- `protocol/*`: validation and bounds tests (reject malformed/abusive inputs).
- `persistence/*`: Dockerized integration tests (schema + checkpoint roundtrips).

## Non-Goals

- Multiple runtime services (no separate Node/JS server).
- Client authority for gameplay outcomes.
