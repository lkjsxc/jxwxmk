# Barrier Cores

Barrier cores define safe zones by removing hostile mobs within their range.

## Properties

- `id`: Unique identifier
- `x`, `y`: Position
- `level`: Barrier level (1-3 in current spawn logic)
- `base_range`: Base range for level 1

## Range

`range = base_range + (level - 1) * level_multiplier`

## Spawning

- A level-1 core always spawns at world center.
- Additional cores spawn probabilistically near the center.
