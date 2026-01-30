# Config Crate

Configuration file loading and validation.

## Modules

- `lib.rs`: Config loader with defaults
- `schemas.rs`: Typed config structures for all 11 config files

## Supported Config Files

- `server.json`: HTTP/WS ports, tick rate, limits
- `world.json`: World seed, chunk size, streaming radii
- `balance.json`: Player stats, item scaling
- `survival.json`: Hunger, temperature, regen rates
- `crafting.json`: Recipe definitions
- `spawning.json`: Spawn budgets, respawn timers
- `biomes.json`: Biome definitions
- `settlements.json`: Barrier cores, settlement tiers
- `economy.json`: Pricing, taxes
- `quests.json`: Quest templates
- `achievements.json`: Achievement definitions
