# Survival System

The survival system runs once per tick for each spawned player.

## Hunger

- Hunger decays by `survival.hunger_decay / tick_rate`.
- Biomes may apply a multiplicative modifier: `survival.hunger_decay *= biome.hunger_modifier` (default `1.0`).
- If hunger is 0, health is reduced by `survival.starve_damage / tick_rate`.
- If hunger is above `survival.heal_threshold`, health regenerates by `survival.heal_rate / tick_rate`.

## Temperature

- Temperature moves toward `survival.neutral_temp` with biome modifiers.
- Target temperature:
  - `target = survival.neutral_temp + biome.temperature_modifier`
- Convergence (stable lerp):
  - `temp = lerp(temp, target, survival.temperature_converge_rate * dt)`
- If temperature reaches 0, health is reduced by `survival.freeze_damage / tick_rate`.

## Thirst (Optional)

- Enabled via `survival.thirst_enabled`.
- Thirst decay and dehydration damage mirror hunger rules.

## Clamping

- Health is clamped between 0 and `balance.player.max_health` each tick.
