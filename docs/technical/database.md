# Database Schema

PostgreSQL is used for persistent data.

## Tables

### `users`
- `id`: UUID (Primary Key)
- `username`: VARCHAR(50) Unique
- `password_hash`: VARCHAR
- `created_at`: TIMESTAMP

### `scores`
- `id`: UUID
- `user_id`: UUID (FK)
- `score`: INT
- `survival_time`: INTERVAL
- `died_at`: TIMESTAMP

## Data Access

- Uses `sqlx` for type-safe compile-time checked queries.
- Migrations managed via `sqlx-cli` (or manual scripts in startup).
