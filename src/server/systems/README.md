# Systems Crate

Deterministic gameplay systems invoked by the tick loop.

## Purpose

Pure gameplay logic with no network or database dependencies.

## Systems

- `SurvivalSystem` - Hunger, temperature, vital effects
- `MovementSystem` - Player movement application
- `CraftingSystem` - Recipe validation and crafting
- `DeathSystem` - Death detection and respawn
- `BarrierSystem` - Safe zone rules and PvP restrictions
- `AchievementSystem` - Achievement evaluation

## Determinism

All systems operate on:
- Fixed `dt` (time step)
- Typed config values
- World state snapshots

No randomness or external I/O within system logic.
