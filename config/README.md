# Configuration

JSON configuration files loaded by the server at startup.

## Files

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

## Loading

Server loads all `*.json` files from `/app/config` at startup.
Missing files use documented defaults.
Unknown fields are rejected.
