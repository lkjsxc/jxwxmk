# World Crate

World state, chunks, and entity storage.

## Purpose

Contains the authoritative world state structures:
- `World` - Root container with chunk map and player states
- `Chunk` - Spatial partition with entities
- `PlayerState` - Player vitals, inventory, progression
- `Settlement` - Village/settlement data

## Key Types

- `ChunkCoord` - (i32, i32) chunk coordinates
- `PlayerId` - UUID for player identification
- `EntityId` - String ID for world entities

## Chunk Contents

Each chunk contains:
- `resources` - Trees, rocks, ores
- `mobs` - Animals, monsters
- `structures` - Player-built structures
- `npcs` - Villagers, traders
- `cooldowns` - Respawn timers

## Interest Management

`World::update_interest_set()` calculates chunk add/remove for players based on view radius.
