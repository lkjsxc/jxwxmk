# Crates

Rust workspace crates for the game server.

## Crate Structure

Crates are organized by dependency order (protocol → config → world → systems → persistence → game → net → assets).

## Crates

- [`protocol/`](protocol/): Message types and validation
- [`config/`](config/): Configuration loading
- [`world/`](world/): World state, chunks, entities
- [`systems/`](systems/): Gameplay systems
- [`persistence/`](persistence/): Database persistence
- [`game/`](game/): Game engine and tick loop
- [`net/`](net/): HTTP and WebSocket handlers
- [`assets/`](assets/): Static asset embedding
