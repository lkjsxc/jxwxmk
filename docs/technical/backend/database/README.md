# Database Persistence

The persistence layer is implemented using `sqlx` and PostgreSQL.

## Connection

- The server connects to the database URL specified in `DATABASE_URL` env var.
- Default: `postgres://postgres:postgres@127.0.0.1:5432/jxwxmk`

## Initialization

- `src/server/database.rs` contains `init_pool`.
- It executes a raw SQL query to create the `players` table if it does not exist.
- No separate migration tool is required at this stage.

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
| inventory | JSONB | Serialized inventory slots |
| stats | JSONB | Serialized stats |
| spawned | BOOLEAN | Whether player is currently in-world |
| updated_at | TIMESTAMPTZ | Last save time |

## Usage

- **Load**: On `Join`, the `GameEngine` checks for a provided token. If valid, it loads the player state.
- **Save**:
  - Periodically (every 10s) for all spawned players.
  - On `Leave` (disconnect).
  - On creation of a new player.
- **Session Claim**: When a player ID is claimed, the stored token is rotated and existing sessions are revoked.
