# Persistence

The current implementation does not persist game state to PostgreSQL. Player state is held in memory and survives only as long as the server process runs.

## Session Tokens

- Each player is assigned a UUID token on first connection.
- The client stores the token in local storage and presents it on reconnect.
- The token reattaches the session to the existing player entity (if present in memory).

## Implications

- Server restart resets the world and all player progress.
- PostgreSQL is started in the runtime container but is not used yet.

## Future Persistence Targets (Not Implemented)

- Accounts and sessions
- Player inventory, stats, and quest state
- World structures and village state
