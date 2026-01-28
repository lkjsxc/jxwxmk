# 05 — Game + World (tick loop + chunks + streaming)

Goal: implement the authoritative simulation core with chunked world state and delta streaming.

References:
- `docs/technical/backend/game/engine.md`
- `docs/technical/backend/game/world_state.md`
- `docs/technical/module_map.md` (single writer, event queues)
- `docs/design/world/scale_and_chunks.md`
- `docs/design/world/settlements.md` (villages as anchors)

## A) Tick loop + single-writer ownership

- [ ] Implement a fixed tick loop (20–60Hz) controlled by `server.tick_rate`.
- [ ] Enforce the single-writer rule:
  - network handlers enqueue events
  - only the tick owner mutates `World`
- [ ] Bounded queues for inputs/events (avoid unbounded memory growth).

## B) World state + chunk storage

- [ ] Implement `World` with fields described in `docs/technical/backend/game/world_state.md`:
  - `seed`, `chunks`, `players`, `active_chunks`, `interest_sets`
- [ ] Implement `Chunk` structure and entity storage by kind.
- [ ] Deterministic base generation:
  - biome selection and baseline resources seeded by world seed + chunk coord.
- [ ] Track chunk “dirty” state for checkpointing (persistence integration later).

## C) Chunk lifecycle + interest management

- [ ] Activate/deactivate chunks based on player positions and streaming radii.
- [ ] Maintain per-player interest sets and replicate:
  - `chunkAdd` when entering view radius
  - `chunkRemove` when leaving view radius
- [ ] Freeze far chunks and stop ticking entities outside simulation radius.

## D) Delta building + broadcast

- [ ] Build `entityDelta` per tick per interested player.
- [ ] Cap update sizes per tick (performance docs).

## E) Settlements/villages baseline

- [ ] Generate barrier-core-centered settlements (at minimum: one village).
- [ ] Ensure each settlement has:
  - bounds and safe-zone radius
  - respawn association
  - ≥1 interaction surface (NPC trade, bulletin board, stash, etc.)

## Done when

- [ ] Clients receive chunk add/remove and entity deltas based on movement.
- [ ] Tick loop runs deterministically with bounded memory.
