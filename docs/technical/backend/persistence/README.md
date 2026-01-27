# Persistence

Persistence is required for MMORPG-scale continuity.

## Persisted Data

- Accounts and sessions (single active session per player).
- Player progression (levels, inventory, quests, achievements).
- Settlement state (core integrity, NPC inventories, reputation).
- Chunk deltas (structures, depleted nodes, event states).

## Session Rules

- Each player has at most one active session token.
- Issuing a new token revokes the previous session.
- Session revocation is sent to the client before disconnect.

## Checkpoint Strategy

- Periodic checkpoints for chunks and settlements.
- Player state saved on logout and at fixed intervals.
- Avoid per-tick writes; coalesce changes.

## PostgreSQL Usage

- PostgreSQL runs inside the runtime container.
- Migrations define player, settlement, session, and chunk tables.
