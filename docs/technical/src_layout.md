# Source Layout (Keep Human-Maintained Files ≤ 200 Lines)

This repository is **docs-first**: `src/` may be deleted and later reconstructed from `docs/`.
This document records the intended `src/` layout and the conventions used to keep the codebase easy to reconstruct and iterate on with agents.

## File-size rule

- Human-maintained source files should stay **≤ 200 lines**.
  - When a file exceeds that size, split it by responsibility (new modules/files).
- Keep entrypoints (`lib.rs`, `main.rs`, `index.ts`) as thin wiring layers that delegate to submodules.
- Generated artifacts are exempt from the 200-line rule:
  - `src/client/package-lock.json`
  - the bundled client output `src/static/game.js` (built by `esbuild`)

## Observed oversize hotspots (avoid repeating this)

In a prior reconstruction, the following files grew beyond the target size. Treat them as explicit “must-split” hotspots during reconstruction:

- `src/server/protocol/src/lib.rs` (~366 lines): split message types + validation into modules.
- `src/server/config/src/lib.rs` (~468 lines): split loader/error types and per-file schemas.
- `src/server/world/src/lib.rs` (~391 lines): split coords/chunks/entities/interest sets/generation.
- `src/server/game/src/lib.rs` (~384 lines): split tick loop, queues/handles, outbound delta building.
- `src/server/persistence/src/lib.rs` (~207 lines): split migrations/repos/checkpointing.
- `src/client/index.ts` (~587 lines): split net/input/state/render/ui into subdirectories.
- `src/static/styles.css` (~296 lines): split into CSS modules; keep `styles.css` as an index.
- `src/static/game.js` (~444 lines): generated bundle (exempt; do not hand-edit).

## Canonical `src/` tree (high level)

- `src/runtime/`: Docker runtime wiring (single container: Rust server + Postgres).
- `src/server/`: Rust workspace; crates enforce boundaries from `docs/technical/module_map.md`.
- `src/client/`: TypeScript client sources (build-time only).
- `src/static/`: HTML/CSS + bundled JS embedded by the server.

## Server workspace layout (Rust)

Keep each crate’s `src/lib.rs` small and split functionality into submodules.

### `src/server/protocol/` (types + validation only)

Recommended split:

- `src/server/protocol/src/lib.rs` (reexports only)
- `src/server/protocol/src/version.rs`
- `src/server/protocol/src/ids.rs` (`snake_case` identifier helpers)
- `src/server/protocol/src/error.rs` (error codes + shapes)
- `src/server/protocol/src/entity.rs` (entity snapshot/delta shapes)
- `src/server/protocol/src/c2s/` (client→server message structs)
- `src/server/protocol/src/s2c/` (server→client message structs)
- `src/server/protocol/src/validate/` (bounds/schema validation helpers)

### `src/server/config/` (load + validate JSON)

Recommended split:

- `src/server/config/src/lib.rs` (public API)
- `src/server/config/src/loader.rs` (read `/app/config/*.json`)
- `src/server/config/src/error.rs`
- `src/server/config/src/schemas/` (one module per config file, matching docs schemas)

### `src/server/world/` (pure world data structures)

Recommended split:

- `src/server/world/src/lib.rs` (module wiring)
- `src/server/world/src/coords.rs` (world units + chunk coords)
- `src/server/world/src/chunk.rs`
- `src/server/world/src/entities/` (entity storage + helpers)
- `src/server/world/src/interest.rs` (interest sets + streaming radius helpers)
- `src/server/world/src/gen/` (seeded generation helpers)

### `src/server/game/` (single-writer tick owner)

Recommended split:

- `src/server/game/src/lib.rs`
- `src/server/game/src/engine.rs` (tick loop + scheduler)
- `src/server/game/src/events.rs` (inbound events queued by `net`)
- `src/server/game/src/handle.rs` (`GameHandle` enqueue API)
- `src/server/game/src/outbound.rs` (delta building + broadcast)
- `src/server/game/src/metrics.rs`

### `src/server/persistence/` (sqlx + checkpointing)

Recommended split:

- `src/server/persistence/src/lib.rs`
- `src/server/persistence/src/db.rs`
- `src/server/persistence/src/migrations.rs`
- `src/server/persistence/src/repos/players.rs`
- `src/server/persistence/src/repos/chunks.rs`
- `src/server/persistence/src/repos/settlements.rs`
- `src/server/persistence/src/checkpoint.rs`

## Client layout (TypeScript)

Keep `src/client/index.ts` as the bundler entry, but keep it thin:

- `src/client/index.ts` (boot + dependency wiring)
- `src/client/net/` (HTTP session claim + WebSocket)
- `src/client/protocol/` (TypeScript types matching `docs/technical/backend/server/protocol.md`)
- `src/client/state/` (player state store, chunk cache)
- `src/client/input/` (keyboard/touch unified input)
- `src/client/render/` (camera + entity rendering)
- `src/client/ui/` (HUD/inventory/crafting/quests/achievements surfaces)

## Static assets layout

Split CSS by surface and keep an index file:

- `src/static/styles.css` (imports the module files)
- `src/static/styles/base.css`
- `src/static/styles/hud.css`
- `src/static/styles/inventory.css`
- `src/static/styles/crafting.css`
- `src/static/styles/overlays.css`
