# Scale + Chunks

The world is effectively infinite by using fixed-size chunks and deterministic generation.

## World Space

- The world is an unbounded plane indexed by chunk coordinates.
- Chunk size is fixed (target 128x128 world units).
- A global world seed drives deterministic terrain, biome, and baseline resource placement per chunk.

## Canonical Scale (World Units)

To keep simulation, rendering, and interaction consistent, all gameplay distances use **world units** (wu).

- **World units (wu)** are the simulation coordinate system (floats are allowed).
- **Pixels (px)** are used only for screen-space UI (HUD, menus, touch controls).
- **Rendering mapping** (recommended):
  - `ppu = 16` pixels per world unit at `zoom = 1.0`.
  - `screen_px = world_wu * ppu * zoom`.
  - This mapping keeps a 128wu chunk “large” (2048px at zoom 1.0) so the camera can frame only a small portion of a chunk at normal zoom.

### Reference Sizes (Targets)

These are **design targets** to avoid “objects too big/small” and keep interactions readable at typical zoom.

- **Player radius**: ~`0.75wu` (~12px at zoom 1.0).
- **Resource node radius** (trees/rocks): ~`1.0–1.25wu` (~16–20px).
- **Interact/target acquire range**: `4.0wu` (~64px).
- **Building snap grid**: `2.5wu` cells (~40px) for walls/stations.

### Density + Spacing (Targets)

Avoid “object soup” by enforcing minimum spacing at generation time.

- **Minimum separation** (colliders): `r1 + r2 + 0.25wu`.
- **Resource baseline spacing**: Poisson-disk style min distance `~6wu` between resource centers (biome-dependent).
- **Spawn budgets** should scale down near settlement cores (keep hubs navigable) and up in wilderness chunks.

### Camera Framing (Targets)

- At default zoom, the camera should show **less than one full chunk** in the short axis (prevents “camera too far away”).
- The player should occupy roughly **~2–4%** of viewport height at typical zoom (readable without hiding the world).

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
