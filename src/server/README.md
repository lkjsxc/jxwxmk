# Server

The backend is a single Rust binary that runs the game simulation and API.

## Responsibilities

- **Simulation**: Fixed-tick game loop.
- **Persistence**: PostgreSQL storage via `sqlx`.
- **Network**: HTTP API and WebSocket game channel.
- **Config**: Loading JSON configuration.

## Modules

- `main.rs`: Entrypoint and server setup.
- `config.rs`: Configuration loading and structs.
- `protocol.rs`: JSON message definitions.
- `handlers.rs`: Actix Web route handlers.
- `persistence.rs`: Database connection and queries.
- `game/`: Core game engine and systems.
