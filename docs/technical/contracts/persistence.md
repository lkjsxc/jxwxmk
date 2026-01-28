# Persistence Contract (Migrations, Checkpoints)

References:
- `docs/technical/backend/persistence/README.md`
- `docs/technical/backend/database/README.md`
- `docs/technical/backend/database/schema.md`

## Database role

- PostgreSQL is a persistence implementation detail of the server.
- The simulation is not DB-driven per tick; it is tick-driven and checkpoints state.

## Migrations

- Migrations are versioned SQL files.
- Migrations apply at server startup inside the container.
- Migrations must be idempotent where possible (safe to re-run).

## Checkpointing (no per-tick writes)

- Player state is saved:
  - on disconnect
  - on a fixed interval (coalesced)
- Chunks and settlements are saved:
  - on a fixed interval
  - only if dirty
- Per-tick writes are forbidden.

## Serialization format

- JSONB is the canonical format for complex columns (`inventory`, `stats`, chunk `state`).
- Serialization must be versioned (include a `version` field inside JSON blobs) so schema evolution is survivable.

## Failure behavior

If persistence fails:

- the server must log clearly and emit a metric
- the server should degrade safely (continue simulation if possible, but never corrupt state)
- for unrecoverable DB failures, fail fast on startup rather than running without persistence
