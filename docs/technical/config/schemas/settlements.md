# `settlements.json`

Purpose: settlement generation parameters, barrier core scaling, and safe-zone rules.

References:
- `docs/technical/backend/game/barriers.md`
- `docs/design/world/settlements.md`

## Schema (v1)

```json
{
  "version": 1,
  "barrier": {
    "base_range_wu": 24.0,
    "level_multiplier_wu": 6.0
  },
  "tiers": [
    { "min_core_level": 1, "name": "outpost" },
    { "min_core_level": 2, "name": "village" },
    { "min_core_level": 4, "name": "town" },
    { "min_core_level": 7, "name": "city" }
  ]
}
```

## Validation rules

- `barrier.base_range_wu` > 0 and finite.
- `barrier.level_multiplier_wu` > 0 and finite.
- `tiers` sorted by `min_core_level` ascending.
- `name` is `snake_case`.

## Notes

- Barrier effective range formula is documented in `docs/technical/backend/game/barriers.md`.
