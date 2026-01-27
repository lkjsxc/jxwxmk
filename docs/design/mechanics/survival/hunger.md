# Hunger Mechanics

## Decay

- Hunger decays at `mechanics.hunger_decay` per second.
- The server divides this by `tick_rate` each tick.

## Effects

- Hunger <= 0: apply `mechanics.starve_dmg` per second (scaled per tick).
- Hunger >= `balance.player.heal_threshold`: apply `mechanics.heal_rate` per second (scaled per tick) if health is below max.

## Restoration

- Food items restore a fixed amount: `mechanics.food_value`.
