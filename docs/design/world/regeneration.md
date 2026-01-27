# Regeneration + Respawns

The world repairs itself to sustain an MMORPG-scale loop.

## Resource Regeneration

- Depleted nodes enter a **cooldown** state with a respawn timer.
- Respawn time scales with node level and biome scarcity.
- Regenerated nodes can shift location within the chunk to avoid static farming.

## Creature Respawns

- Spawn budgets are per-chunk and per-biome.
- Elite and boss spawns use longer cooldowns and global caps.
- Spawn tables consider time-of-day and nearby player population.

## Structure Recovery

- **Player structures** persist but decay slowly if abandoned.
- **Settlement structures** regenerate automatically if destroyed.
- Ruins and abandoned structures can rehydrate over time for exploration.

## Anti-Exhaustion Rules

- Node caps per chunk prevent total depletion.
- Rare nodes use global spawn counters with soft caps.
- Regeneration avoids spawning directly on top of players.
