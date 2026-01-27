# Database Persistence

The persistence layer is implemented using `sqlx` and PostgreSQL.

## Connection

- The server connects to the database URL specified in `DATABASE_URL` env var.
- Default: `postgres://postgres:postgres@127.0.0.1:5432/jxwxmk`

## Initialization

- The server applies SQL migrations at startup.
- Migrations create/upgrade all persistence tables (players + world state).

## Schema

### players

| Column | Type | Description |
|---|---|---|
| id | UUID | Primary Key (Player ID) |
| token | UUID | Unique session token |
| username | TEXT | Display name |
| level | INT | Player level |
| xp | BIGINT | Player experience points |
| x, y | DOUBLE | Position |
| health | DOUBLE | Current health |
| hunger | DOUBLE | Current hunger |
| temperature | DOUBLE | Current temperature |
| inventory | JSONB | Serialized inventory slots |
| stats | JSONB | Serialized stats |
| spawned | BOOLEAN | Whether player is currently in-world |
| updated_at | TIMESTAMPTZ | Last save time |

### settlements

- One row per settlement / barrier core.
- Stores core level/integrity plus evolving NPC/service state.

### chunks

- One row per chunk coordinate.
- Stores the persisted “delta” from deterministic generation:
  - placed structures
  - depleted nodes + cooldown timers
  - event flags/state

## Usage

- **Load**:
  - On startup, load persisted settlements and chunk deltas into the world cache.
  - On `Join`, validate token and load player state.
- **Save**:
  - Player state: on interval and on disconnect.
  - World state: periodic chunk/settlement checkpoints (coalesced; never per-tick).
- **Session Claim**: When a player ID is claimed, the stored token is rotated and existing sessions are revoked.
