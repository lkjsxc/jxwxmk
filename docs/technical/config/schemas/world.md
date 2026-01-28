# `world.json`

Purpose: world generation and streaming parameters.

## Schema (v1)

```json
{
  "version": 1,
  "seed": "123456789",
  "chunk_size_wu": 128,
  "view_radius_chunks": 3,
  "sim_radius_chunks": 2,
  "max_active_chunks": 512
}
```

## Validation rules

- `seed`: string containing an unsigned integer (base-10) in `u64` range.
  - If omitted, the server may generate a seed, but it must log it.
- `chunk_size_wu`: integer > 0 (recommended: 128).
- `view_radius_chunks`: integer >= 1.
- `sim_radius_chunks`: integer >= 0 and `<= view_radius_chunks`.
- `max_active_chunks`: integer > 0 (LRU cap).

## Notes

- Distances are in world units (`wu`) (see: `docs/technical/contracts/world_space.md`).
