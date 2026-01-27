# Survival Mechanics

Survival pressure scales with biome, weather, and progression tier.

## Spawning

- Players bind to a settlement core for respawn.
- Spawn points are selected within the settlement safe zone.

## Vitals

- **Health**: 0-100 base, modified by gear and buffs.
- **Hunger**: 0-100, decays continuously.
- **Temperature**: 0-100, driven by biome and weather.
- **Thirst**: optional, enabled per-world in `config/survival.json`.

## Environmental Pressure

- Biomes apply temperature deltas and hazard effects.
- Weather modifies temperature and visibility.
- Cold/heat exposure applies periodic damage when thresholds are crossed.

## Consumption

- Food restores hunger and may provide buffs.
- Cooking improves efficiency and reduces debuffs.

## Configuration

- Rates and thresholds live in `config/survival.json`.
- Base stats and resistances live in `config/balance.json`.
