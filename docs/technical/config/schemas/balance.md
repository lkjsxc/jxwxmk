# `balance.json`

Purpose: core numeric balance values used across systems.

## Schema (v1)

```json
{
  "version": 1,
  "player": {
    "max_health": 100.0,
    "base_speed": 6.0,
    "interaction_range_wu": 4.0,
    "hotbar_slots": 7,
    "inventory_slots": 28
  },
  "resources": {
    "tree_amount": 30.0,
    "rock_amount": 30.0,
    "food_amount": 10.0
  },
  "tools": {
    "rock_mult": 1.5
  }
}
```

## Validation rules

- All numeric values must be finite.
- `player.max_health` > 0.
- `player.base_speed` > 0.
- `player.interaction_range_wu` > 0.
- `player.hotbar_slots` in `[1, 10]` (recommended).
- `player.inventory_slots` >= `player.hotbar_slots`.
- Resource amounts must be > 0.
- `tools.rock_mult` >= 1.0.

## Notes

- This file is intentionally minimal for the initial reconstruction; add new sections as systems grow.
