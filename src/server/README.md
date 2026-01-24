# Rust Server Code

Planned structure:
- `main.rs`: Entrypoint.
- `handlers.rs`: WebSocket/HTTP routes.
- `world.rs`: Simulation logic.
- `net.rs`: Networking code.

## Main Components
- Actix app with WS.
- Channels for inputs/snapshots.
- Tick loop in tokio task.