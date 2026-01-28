# Config File Map

All files are JSON under `config/`.

- `server.json`: HTTP/WebSocket ports, tick rate, rate limits. ([schema](schemas/server.md))
- `world.json`: chunk size, streaming radii, world seed. ([schema](schemas/world.md))
- `balance.json`: player/mob stats, item scaling, base damage. ([schema](schemas/balance.md))
- `survival.json`: hunger, temperature, thirst, regen rates. ([schema](schemas/survival.md))
- `crafting.json`: recipe tables and station tiers. ([schema](schemas/crafting.md))
- `spawning.json`: spawn budgets, respawn timers, density caps. ([schema](schemas/spawning.md))
- `biomes.json`: biome definitions and modifiers. ([schema](schemas/biomes.md))
- `settlements.json`: barrier cores, settlement tiers, guard rules. ([schema](schemas/settlements.md))
- `economy.json`: vendor pricing, taxes, currency sinks. ([schema](schemas/economy.md))
- `quests.json`: quest templates and reward tables. ([schema](schemas/quests.md))
- `achievements.json`: achievement definitions and bonuses. ([schema](schemas/achievements.md))
