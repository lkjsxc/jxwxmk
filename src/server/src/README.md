# Server Source

Module layout for the authoritative game server.

## Contents

- `main.rs`: Process bootstrap and Actix HTTP server.
- `lib.rs`: Shared exports for tests.
- `config/`: Config loading + validation.
- `net/`: HTTP routes, WebSocket sessions, protocol types.
- `game/`: World state, engine, systems, entities.
- `persistence/`: PostgreSQL accessors (players table).
- `util/`: Small helpers.
