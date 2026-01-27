# Barriers

Barriers are protective zones that eliminate hostile mobs within their range. Each barrier is anchored by a **Barrier Core**.

## Mechanics

- Hostile mobs (Wolf, Bear) are removed if inside a barrier range.
- Rabbits are ignored.
- Barrier range scales by level:
  - `range = base_range + (level - 1) * level_multiplier`

## Placement Rules

- A level-1 core always spawns at world center.
- Additional cores spawn probabilistically; the chance increases near the center.
- The total extra cores are capped by `max_additional_barriers`.

## Configuration

All parameters live under `barriers` in `config.json`.
