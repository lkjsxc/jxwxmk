# Survival Mechanics

Survival is the primary pressure on the player.

## Spawning
- **Location**: Players spawn randomly within a circular area at the center of the world.
- **Center**: `(WorldWidth / 2, WorldHeight / 2)`.
- **Radius**: Configurable via `spawn_radius` in `config.json`. This ensures new players start in a concentrated zone (e.g., a "safe" or "starting" area).

## Vitals
- **Health (HP)**: 0-100.
- **Hunger**: 0-100.
- **Temperature**: 0-100.

## Consumption
- **Eating**: Players can consume food items (Berry, Meat, Cooked Meat) from their active hotbar slot.
- **Input**: Triggered by the [B] Button (Interact) when a food item is held.
- **Effect**: Increases Hunger by a configured amount. Excess hunger does not overflow 100.