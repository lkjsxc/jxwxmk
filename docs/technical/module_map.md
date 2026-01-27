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
