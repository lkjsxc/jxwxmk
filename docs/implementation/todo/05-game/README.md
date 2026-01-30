# 05 — Game + World (tick loop + chunks + streaming)

Goal: implement the authoritative simulation core with chunked world state and delta streaming.

References:
- `docs/technical/backend/game/engine.md`
- `docs/technical/backend/game/world_state.md`
- `docs/technical/module_map.md` (single writer, event queues)
- `docs/technical/contracts/tick.md` (backpressure contract)
- `docs/technical/operability/metrics.md` (tick/queue metrics)
- `docs/design/world/scale_and_chunks.md`
- `docs/design/world/settlements.md` (villages as anchors)

## A) Tick loop + single-writer ownership

- [x] Implement a fixed tick loop (20–60Hz) controlled by `server.tick_rate`.
- [x] Enforce the single-writer rule:
  - network handlers enqueue events
  - only the tick owner mutates `World`
- [x] Bounded queues for inputs/events (avoid unbounded memory growth).
- [x] Define explicit overflow behavior for bounded queues (drop policy and/or disconnect) and expose it via logs/metrics.

## B) World state + chunk storage

- [x] Implement `World` with fields described in `docs/technical/backend/game/world_state.md`:
  - `seed`, `chunks`, `players`, `active_chunks`, `interest_sets`
- [x] Implement `Chunk` structure and entity storage by kind.
- [x] Deterministic base generation:
  - biome selection and baseline resources seeded by world seed + chunk coord.
- [x] Track chunk "dirty" state for checkpointing (persistence integration later).

## C) Chunk lifecycle + interest management

- [x] Activate/deactivate chunks based on player positions and streaming radii.
- [x] Maintain per-player interest sets and replicate:
  - `chunkAdd` when entering view radius
  - `chunkRemove` when leaving view radius
- [x] Freeze far chunks and stop ticking entities outside simulation radius.

## D) Delta building + broadcast

- [x] Build `entityDelta` per tick per interested player.
- [x] Cap update sizes per tick (performance docs).
- [x] Track and export per-tick metrics (tick duration, bytes sent, queue lengths) via `/metrics`.

## E) Settlements/villages baseline

- [x] Generate barrier-core-centered settlements (at minimum: one village).
- [x] Ensure each settlement has:
  - bounds and safe-zone radius
  - respawn association
  - ≥1 interaction surface (NPC trade, bulletin board, stash, etc.)

## Done when

- [x] Clients receive chunk add/remove and entity deltas based on movement.
- [x] Tick loop runs deterministically with bounded memory.
