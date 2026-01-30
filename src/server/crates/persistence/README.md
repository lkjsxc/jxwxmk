# Persistence Crate

PostgreSQL persistence for players, settlements, and chunks.

## Modules

- `lib.rs`: PersistenceHandle with migrations and CRUD operations

## Tables

- `players`: Player state (id, token, vitals, inventory, stats)
- `settlements`: Settlement data (id, name, core level, bounds)
- `chunks`: Chunk deltas (cx, cy, biome, state)
