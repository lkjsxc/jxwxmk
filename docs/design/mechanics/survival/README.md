# Survival Mechanics

Survival is the primary pressure on the player.

## Spawning
- **Random Location**: Players spawn at a random coordinate `(x, y)` within the world bounds (0,0 to Width,Height) to prevent camping and spread population.

## Vitals
- **Health (HP)**: 0-100.
- **Hunger**: 0-100.
- **Temperature**: 0-100.

## Consumption
- **Eating**: Players can consume food items (Berry, Meat, Cooked Meat) from their active hotbar slot.
- **Input**: Triggered by the [B] Button (Interact) when a food item is held.
- **Effect**: Increases Hunger by a configured amount. Excess hunger does not overflow 100.