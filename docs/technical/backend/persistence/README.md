# Persistence

Persistence is required for MMORPG-scale continuity.

## Persisted Data

- Accounts and sessions.
- Player progression (levels, inventory, quests, achievements).
- Settlement state (core integrity, NPC inventories, reputation).
- Chunk deltas (structures, depleted nodes, event states).

## Checkpoint Strategy

- Periodic checkpoints for chunks and settlements.
- Player state saved on logout and at fixed intervals.
- Avoid per-tick writes; coalesce changes.

## PostgreSQL Usage

- PostgreSQL runs inside the runtime container.
- Migrations define player, settlement, and chunk tables.
