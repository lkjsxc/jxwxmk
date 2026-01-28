# `quests.json`

Purpose: quest templates and reward tables.

References:
- `docs/technical/backend/game/quests.md`
- `docs/design/mechanics/quests.md`

## Schema (v1)

```json
{
  "version": 1,
  "templates": [
    {
      "id": "caravan_guard",
      "name": "Guard the Caravan",
      "objectives": [
        { "type": "kill", "mob_type": "wolf", "count": 3 }
      ],
      "rewards": { "xp": 50 }
    }
  ]
}
```

## Validation rules

- `id` is `snake_case` and unique.
- Objective `type` is one of the supported objective types.
- `count` values are integers > 0.
- Reward values are non-negative.

## Notes

- Quest objective schema must match the supported types in `docs/technical/backend/game/quests.md`.
