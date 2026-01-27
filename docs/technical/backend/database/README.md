# Database

PostgreSQL runs inside the runtime container but is not currently used by the server code.

## Current State

- No tables or migrations are defined in the codebase.
- The runtime entrypoint ensures the `kkmypk` database exists.

## Intended Usage (Future)

- Accounts and sessions
- Player persistence (inventory, stats, quests)
- World structures/villages

## Schema Notes

See [schema.md](schema.md) for a placeholder schema outline.
