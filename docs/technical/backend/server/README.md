# Server Architecture

## Actor Hierarchy

- **System**:
    - `GameServer`: The singleton manager. Holds the `World`.
    - `DbExecutor`: Pool for async DB writes.
- **Session**:
    - `GameSession`: One per WebSocket connection. Handles packet parsing.

## Game Loop (Tick)

1.  **Input Processing**: Apply queued inputs from `GameSession`s to Player Entities.
2.  **Physics**:
    - Update Positions.
    - Collision Detection (AABB).
    - Resolve Overlaps.
3.  **Game Logic**:
    - Hunger/Cold Tick.
    - Mob AI Tick.
    - Spawning/Despawning.
4.  **Broadcast**:
    - Serialize `WorldState`.
    - Send to all `GameSession`s.
