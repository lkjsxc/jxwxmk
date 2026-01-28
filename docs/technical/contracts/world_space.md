# World Space + IDs Contract

## World units (wu)

- All simulation coordinates are in **world units (wu)**.
- Rendering uses pixels; conversion is a client concern (see: `docs/design/world/scale_and_chunks.md`).

## Chunking

- Chunk coordinate space is an infinite integer grid `(cx, cy)`.
- Default chunk size target is `128wu × 128wu` (configurable by `world.chunk_size_wu`).
- Conventions:
  - Chunk coords are integers.
  - Entity positions are floats in world units.

## Coordinate validation (server-side)

All inbound coordinates must be validated:

- values must be finite (no NaN/Inf)
- absolute values must be bounded to prevent overflow/precision attacks
- action targets must be within max interaction range of the player

## Stable IDs

Protocol-visible IDs must be stable within their domain:

- Player IDs: UUID.
- Session token: UUID (treated as secret).
- Entity IDs:
  - stable string IDs are acceptable if they remain stable within a chunk
  - do not use hash iteration order as an implicit ID source

## Placement grid

Structures snap to a `2.5wu` grid (see: `docs/design/world/scale_and_chunks.md`).
Placement validity must include collision checks against:

- the player collider
- other structures in the chunk
