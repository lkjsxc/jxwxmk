# Rust Server Design

## Structure
- `main.rs`: Entrypoint; spawn simulation task, start Actix server.
- `handlers/`: HTTP/WebSocket routes (login, asset serving, game WS).
- `world/`: Simulation state (entities, biomes, resources); tick function.
- `net/`: Protocol encoding/decoding, message queues.
- `db/`: Connection pool, queries for persistence.

## Simulation
- Single task owns world state.
- Receives inputs from bounded channels (from WS handlers).
- Advances deterministically: process inputs, update entities, compute deltas.
- Publishes snapshots/deltas to per-client outbound queues.

## Libraries
- `tokio`: Async runtime.
- `actix-web`: Web framework, WS support.
- `tracing`: Structured logs.
- `thiserror`: Errors.
- `serde`: Config/protocol serialization.