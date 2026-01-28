# `achievements.json`

Purpose: achievement definitions and rewards.

References:
- `docs/technical/backend/game/achievements.md`
- `docs/design/mechanics/achievements/list.md`

## Schema (v1)

```json
{
  "version": 1,
  "achievements": [
    {
      "id": "first_steps",
      "name": "First Steps",
      "description": "Walk 1,000 steps.",
      "requirement": { "type": "steps", "count": 1000 },
      "rewards": { "xp": 25, "stat_bonuses": {} }
    }
  ]
}
```

## Validation rules

- `id` is `snake_case` and unique.
- `requirement.type` must be one of the supported requirement types.
- Counts are integers > 0.
- XP rewards are integers >= 0.

## Notes

- Rewards apply immediately and persist on the player record.
