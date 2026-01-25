# Database Schema

## Users
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

## GameData (Persistence)
*Note: Currently game is session-based, but this supports saving.*
```sql
CREATE TABLE player_saves (
    user_id UUID REFERENCES users(id),
    inventory JSONB NOT NULL DEFAULT '{}',
    x FLOAT NOT NULL,
    y FLOAT NOT NULL,
    stats JSONB NOT NULL -- hp, hunger, etc
);
```
