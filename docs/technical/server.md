# Server Architecture

The server is the authority on game state.

## Game Loop

- Runs at a fixed tick rate (e.g., 20 or 60 TPS).
- Updates all entity positions, health, and status.
- Processes input queues from connected players.
- Broadcasts state snapshots to relevant clients (spatial partitioning might be needed later).

## Actor Model

Using `actix` actors for WebSocket connections:
- `GameSession`: Represents a connected player socket.
- `GameServer`: Global actor managing the world state and broadcasting messages.

## State Management

- **World**: Grid-based map.
- **Entities**: HashMap of ID -> Entity (Player, Mob, Item).
- **Physics**: Simple AABB collision detection.
