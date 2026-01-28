# `spawning.json`

Purpose: chunk-local spawn budgets and respawn timers.

References:
- `docs/technical/backend/game/spawning_and_ai.md`

## Schema (v1)

```json
{
  "version": 1,
  "resource_respawn_seconds": {
    "tree": 120,
    "rock": 180,
    "food": 60
  },
  "mob_respawn_seconds": {
    "wolf": 180
  },
  "chunk_budgets": {
    "forest": {
      "resources": { "tree": 25, "rock": 12, "food": 10 },
      "mobs": { "wolf": 4 }
    }
  }
}
```

## Validation rules

- Respawn seconds are integers >= 0.
- Budgets are integers >= 0.
- Biome IDs are `snake_case` and must exist in `biomes.json`.

## Notes

- The server may enforce additional global caps for elites/bosses later; initial schema keeps it simple.
