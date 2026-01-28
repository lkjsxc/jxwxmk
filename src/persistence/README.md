# Persistence Module

Handles database interactions and state checkpointing.

## Responsibilities
- PostgreSQL connection pool (sqlx).
- Database migrations.
- Loading player/world state.
- Saving dirty state (checkpoints).

## Dependencies
- `world` (to map state to DB).
- `protocol` (player data).
- `config` (DB connection string).
