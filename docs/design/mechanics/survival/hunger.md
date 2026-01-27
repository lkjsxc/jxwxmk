# Hunger Mechanics

## Decay

- Hunger decays at `survival.hunger_decay` per second.
- The server divides this by `tick_rate` each tick.

## Effects

- Hunger <= 0: apply `survival.starve_damage` per second (scaled per tick).
- Hunger >= `survival.heal_threshold`: apply `survival.heal_rate` per second (scaled per tick) if health is below max.

## Restoration

- Food items restore hunger (simple baseline uses `survival.food_value`).
