# Structures

Placeable entities created by players using the attack action with structure items.

## Types

- **Wood Wall**
- **Door**
- **Torch**
- **Workbench**

## Properties

- `x`, `y`: placement location (player position at placement time)
- `health`: configured per type in `config.json`
- `owner_id`: player UUID

## Notes

- Structures are stored in the world and broadcast to clients each tick.
- Doors are not locked; all players can pass or attack them.
