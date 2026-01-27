# Scale + Chunks

The world is effectively infinite by using fixed-size chunks and deterministic generation.

## World Space

- The world is an unbounded plane indexed by chunk coordinates.
- Chunk size is fixed (target 128x128 world units).
- A global world seed drives deterministic terrain, biome, and baseline resource placement per chunk.

## Chunk Layers

Each chunk has layered data so it can be regenerated and partially persisted:

1. **Base terrain + biome** (deterministic from seed + coords).
2. **Resource nodes** (deterministic baseline + dynamic overrides).
3. **Structures + settlements** (authored by generation and player placement).
4. **Dynamic overlay** (damage, depletion, cooldowns, temporary spawns).

## Streaming Rules

- **Simulation radius**: chunks within `R_sim` of a player tick each frame.
- **View radius**: chunks within `R_view` are replicated to the client.
- **Far chunks** are frozen and stored as compact deltas (no per-tick updates).

## Interest Management

- Clients receive only entities inside their interest set.
- Updates are chunk deltas, not full-world snapshots.
- Entity IDs are stable within a chunk; chunk streaming controls add/remove semantics.

## Infinite-World Optimizations

- Deterministic generation for empty chunks (no disk reads when pristine).
- LRU chunk cache with explicit memory caps.
- Bounded background generation queue.
- Chunk-level RNG streams for reproducibility.
- Aggressive culling of physics, AI, and pathing outside `R_sim`.
