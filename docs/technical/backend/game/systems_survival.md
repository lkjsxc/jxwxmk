# Survival System

The survival system runs once per tick for each spawned player.

## Hunger

- Hunger decays by `survival.hunger_decay / tick_rate`.
- If hunger is 0, health is reduced by `survival.starve_damage / tick_rate`.
- If hunger is above `survival.heal_threshold`, health regenerates by `survival.heal_rate / tick_rate`.

## Temperature

- Temperature moves toward `survival.neutral_temp` with biome modifiers.
- If temperature reaches 0, health is reduced by `survival.freeze_damage / tick_rate`.

## Thirst (Optional)

- Enabled via `survival.thirst_enabled`.
- Thirst decay and dehydration damage mirror hunger rules.

## Clamping

- Health is clamped between 0 and `balance.player.max_health` each tick.
