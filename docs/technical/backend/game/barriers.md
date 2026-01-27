# Barrier System

Barrier cores define safe zones that automatically eliminate hostile mobs inside their range.

## BarrierCore

- `level` controls effective range.
- Range formula: `base_range + (level - 1) * level_multiplier`.

## Spawning Rules

- A level-1 core always spawns at world center.
- Additional cores spawn with a probability biased toward the center.
- `max_additional_barriers` caps total extra cores.

## Tick Behavior

Each tick:

- For each hostile mob (Wolf/Bear), if inside any barrier range, the mob is removed.
- Rabbits are ignored (not hostile).

## Village Coupling

Barrier cores act as village anchors. NPCs spawn near cores to provide early-game interaction.
