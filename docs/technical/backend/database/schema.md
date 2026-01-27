# Database Schema (Placeholder)

The server currently does not execute SQL migrations. This document outlines a minimal schema to support persistence when implemented.

## accounts

```sql
CREATE TABLE accounts (
    id UUID PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

## player_state

```sql
CREATE TABLE player_state (
    account_id UUID REFERENCES accounts(id),
    token UUID UNIQUE NOT NULL,
    inventory JSONB NOT NULL DEFAULT '{}',
    stats JSONB NOT NULL DEFAULT '{}',
    quests JSONB NOT NULL DEFAULT '[]',
    x DOUBLE PRECISION NOT NULL,
    y DOUBLE PRECISION NOT NULL,
    spawned BOOLEAN NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

## world_state

```sql
CREATE TABLE world_state (
    id UUID PRIMARY KEY,
    structures JSONB NOT NULL DEFAULT '[]',
    villages JSONB NOT NULL DEFAULT '[]',
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```
