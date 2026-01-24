# Database Migrations

## Migration Structure

```
migrations/
├── 001_initial_schema.sql
├── 002_add_player_progress.sql
├── 003_add_inventory_system.sql
└── ...
```

## Migration Format

```sql
-- Up migration (apply changes)
CREATE TABLE players (
    id UUID PRIMARY KEY,
    username VARCHAR(32) UNIQUE NOT NULL,
    -- other fields
);

-- Down migration (rollback changes)
DROP TABLE players;
```

## Migration Process

### Creating Migrations
```bash
# Find next migration number
ls migrations/ | sort -n | tail -1

# Create new migration
cp migrations/template.sql migrations/004_new_feature.sql
```

### Applying Migrations
```bash
# Manual application
psql -U game_user -d game_db -f migrations/001_initial_schema.sql

# Using migration tool
npx pg-migrate up
```

### Rolling Back
```bash
# Rollback specific migration
npx pg-migrate down 003_add_inventory_system

# Rollback all
npx pg-migrate down
```

## Migration Best Practices

### Atomic Changes
- Each migration should be self-contained
- Include both up and down scripts
- Test rollback before deployment

### Data Preservation
- Handle existing data carefully
- Provide data migration paths
- Backup before major changes

### Performance
- Add indexes in separate migrations
- Batch large data changes
- Avoid long-running transactions

## Example Migration

```sql
-- 001_initial_schema.sql

-- Up: Create core tables
CREATE TABLE players (
    id UUID PRIMARY KEY,
    username VARCHAR(32) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_login TIMESTAMP WITH TIME ZONE,
    is_banned BOOLEAN DEFAULT FALSE
);

CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Indexes
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
CREATE INDEX idx_sessions_token_hash ON sessions(token_hash);

-- Down: Drop tables
DROP TABLE sessions;
DROP TABLE players;
```