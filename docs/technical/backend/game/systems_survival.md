# Survival System

The survival system runs once per tick for each spawned player.

## Hunger

- Hunger decays by `mechanics.hunger_decay / tick_rate` per tick.
- If hunger drops below 0, it is clamped to 0 and health is reduced by `mechanics.starve_dmg / tick_rate`.
- If hunger is above `balance.player.heal_threshold` and health is below max, health heals by `mechanics.heal_rate / tick_rate`.

## Temperature (Cold)

- Player temperature moves toward `balance.player.neutral_temp` at `mechanics.cold_decay / tick_rate`.
- If `cold <= 0`, health is reduced by `mechanics.freeze_dmg / tick_rate`.

## Clamping

- Health is clamped between 0 and `balance.player.max_health` each tick.
