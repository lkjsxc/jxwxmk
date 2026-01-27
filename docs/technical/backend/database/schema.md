# Database Schema

The current implementation initializes a single `players` table directly on startup (no migration tool yet). Additional tables are a future expansion.

## players (current)

```sql
CREATE TABLE IF NOT EXISTS players (
    id UUID PRIMARY KEY,
    token UUID UNIQUE NOT NULL,
    username TEXT NOT NULL,
    level INT NOT NULL,
    xp BIGINT NOT NULL,
    x DOUBLE PRECISION NOT NULL,
    y DOUBLE PRECISION NOT NULL,
    health DOUBLE PRECISION NOT NULL,
    hunger DOUBLE PRECISION NOT NULL,
    inventory JSONB NOT NULL,
    stats JSONB NOT NULL,
    spawned BOOLEAN NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);
```

## Future expansion (not yet implemented)

- `accounts`
- `player_state`
- `world_state`
