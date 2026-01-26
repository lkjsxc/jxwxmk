# Player Persistence

Players remain in the world after disconnection and can rejoin.

## Mechanics

### Disconnect
- **State**: When a WebSocket closes, the `Player` entity is **NOT** removed from the `World`.
- **Status**: The entity is marked as `Offline` (optional, for visualization).
- **Vulnerable**: The body stays in the world and can take damage/die while offline.

### Reconnect
- **Token System**:
    - Upon first join, the server issues a unique `secret_token` (UUID).
    - The client saves this token (localStorage).
- **Possession**:
    - When connecting, the client sends the `secret_token`.
    - If a matching Player entity exists, the connection "possesses" that entity.
    - If no match, a new Player is created.

### Death
- If `HP <= 0`, the player entity is marked as dead (`spawned = false`) and inventory is cleared.
- Rejoining with a dead player's token possess the existing entity, prompts a "Game Over" screen, and requires a respawn (fresh start for that entity).
