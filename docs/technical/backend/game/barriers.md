# Barrier System

Barrier cores define safe zones and anchor settlements.

## BarrierCore

- `level` controls effective range.
- `integrity` tracks damage and repair state.
- Range formula: `base_range + (level - 1) * level_multiplier`.

## Generation Rules

- Barrier cores seed settlement generation.
- Core level determines settlement tier and NPC mix.

## Tick Behavior

- Hostile mobs are removed or pushed outside the safe zone.
- PvP is disabled inside the safe-zone radius.

## Configuration

- Parameters live in `config/settlements.json`.
