# Temperature Mechanics

## Target Temperature

- Each tick, the player's temperature moves toward a target value:
  - `target = survival.neutral_temp + biome.temperature_modifier`
- Convergence uses a stable lerp per second:
  - `temp = lerp(temp, target, survival.temperature_converge_rate * dt)`

## Effects

- Temperature <= 0: apply `survival.freeze_damage` per second (scaled per tick).

## Biome Modifiers (Required)

- Each biome definition provides:
  - `temperature_modifier` (positive = warmer, negative = colder)
  - optional `hunger_modifier` (survival pressure scaling)
- Biome modifiers are read from `config/biomes.json` and applied based on the player’s current chunk biome.

## Optional Extensions

- Day/night cycles (additional modifier)
- Weather (additional modifier)
- Heat sources (campfires; local radius effect)
