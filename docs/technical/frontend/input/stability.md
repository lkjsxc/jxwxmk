# Network Stability + Lifecycle

## Session Lifecycle

1. Client opens WebSocket `/ws` with optional `token`.
2. Server sends `welcome` with `id`, `token`, `spawned`.
3. Client sends `{"spawn": true}` if needed.
4. Input loop sends `InputState` every 50ms when in game.

## Disconnect

- On socket close, the client resets local world state and clears its input interval.
- Reconnect occurs on page reload.

## Game Over

- Client enters Game Over state when its player is missing from world updates.
- Respawn closes the socket, clears stored token, and reloads the page.
