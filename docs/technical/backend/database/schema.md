# Database Schema

Schema is managed via migrations. The tables below are the canonical persistence surface for players and world state.

## players

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
    temperature DOUBLE PRECISION NOT NULL,
    inventory JSONB NOT NULL,
    stats JSONB NOT NULL,
    spawned BOOLEAN NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);
```

## settlements

```sql
CREATE TABLE IF NOT EXISTS settlements (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    core_level INT NOT NULL,
    core_integrity DOUBLE PRECISION NOT NULL,
    bounds JSONB NOT NULL,
    state JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);
```

## chunks

```sql
CREATE TABLE IF NOT EXISTS chunks (
    cx INT NOT NULL,
    cy INT NOT NULL,
    biome TEXT NOT NULL,
    state JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (cx, cy)
);
```
