# `biomes.json`

Purpose: biome definitions and survival modifiers.

References:
- `docs/technical/backend/game/systems_survival.md`
- `docs/design/world/biomes.md`

## Schema (v1)

```json
{
  "version": 1,
  "biomes": [
    {
      "id": "forest",
      "temperature_modifier": 0.0,
      "hunger_modifier": 1.0
    }
  ]
}
```

## Validation rules

- `biomes` is an array with unique `id` values.
- `id` is `snake_case`.
- `temperature_modifier` and `hunger_modifier` must be finite.
- `hunger_modifier` must be > 0.

## Notes

- Additional biome fields (spawn tables, visuals) can be added in later versions, but must be documented here.
