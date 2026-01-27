# Survival Mechanics

This section reflects the current, implemented survival rules.

## Spawning

- Players spawn within a circle centered on the world.
- Radius is `game.spawn_radius` from `config.json`.

## Vitals

- Health: 0-100
- Hunger: 0-100
- Temperature: 0-100

## Consumption

- Food is consumed with the attack action if a food item is in the active slot.
- Food restores `mechanics.food_value` hunger.
