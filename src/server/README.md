# Server

Rust workspace containing the authoritative game server.

## Crate Structure

Workspace members (in dependency order):

1. **`protocol/`**: Message types and validation (no dependencies)
2. **`config/`**: Config loading and validation → protocol
3. **`world/`**: World state, chunks, entities → config
4. **`systems/`**: Gameplay systems → world, config
5. **`persistence/`**: Database layer → world, protocol, config
6. **`game/`**: Tick loop and engine → systems, world, config, protocol, persistence
7. **`net/`**: HTTP/WebSocket adapters → protocol, game
8. **`assets/`**: Static asset embedding

## Binary

`src/main.rs` wires all crates together.

## Architecture

- Single-writer: Only `game` crate mutates world state
- I/O enqueues events; tick loop processes them
- Fixed tick rate (20-60Hz)
- All gameplay is server-authoritative
