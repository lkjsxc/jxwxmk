# Config Schemas

This directory defines the authoritative schemas for `config/*.json`.

## Global rules

- All config files are JSON objects.
- Every file includes `version` (integer) so schema evolution is explicit.
- Unknown fields are rejected (recommended).
- Identifiers are `snake_case` strings unless explicitly noted.

## Files

- [`server.json`](server.md)
- [`world.json`](world.md)
- [`balance.json`](balance.md)
- [`survival.json`](survival.md)
- [`crafting.json`](crafting.md)
- [`spawning.json`](spawning.md)
- [`biomes.json`](biomes.md)
- [`settlements.json`](settlements.md)
- [`economy.json`](economy.md)
- [`quests.json`](quests.md)
- [`achievements.json`](achievements.md)
