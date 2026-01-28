# `survival.json`

Purpose: survival pressure and regeneration parameters.

References:
- `docs/technical/backend/game/systems_survival.md`

## Schema (v1)

```json
{
  "version": 1,
  "hunger_decay": 2.0,
  "starve_damage": 5.0,
  "heal_threshold": 60.0,
  "heal_rate": 2.0,
  "neutral_temp": 50.0,
  "temperature_converge_rate": 0.5,
  "freeze_damage": 5.0,
  "thirst_enabled": false,
  "thirst_decay": 2.0,
  "dehydrate_damage": 5.0
}
```

## Validation rules

- All numeric values must be finite.
- Decays/damages/rates must be >= 0.
- `heal_threshold` must be within `[0, 100]` (recommended).
- Temperatures are expressed on a `0..100` scale (design baseline).

## Notes

- Per-tick math divides per-second rates by `tick_rate` as documented.
