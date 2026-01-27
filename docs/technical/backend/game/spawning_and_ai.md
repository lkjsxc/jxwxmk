# Spawning + AI

Spawning is chunk-local with biome-aware tables and respawn timers.

## Chunk Spawn Budgets

- Each chunk has max counts for resources and mobs.
- Budgets scale with biome tier and settlement proximity.
- Respawn timers are stored per node or mob type.

## Resource Spawns

- Resources are seeded deterministically per chunk.
- Depleted nodes enter cooldown and respawn with jittered positions.

## Mob Spawns

- Spawn tables by biome and time of day.
- Elite spawns are globally capped and event-driven.

## Settlement Spawns

- Barrier cores generate settlement NPCs and structures.
- Settlement NPC mix scales with core level and faction.

## AI

- Passive fauna uses low-cost wander logic.
- Predators use aggro + leash ranges and threat tables.
- Bosses use scripted phase logic and telegraphed attacks.
