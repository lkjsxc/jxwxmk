# Network Stability + Lifecycle

## Session Lifecycle

1. Client opens WebSocket `/ws` with optional `token`.
2. Server sends `welcome` with `id`, `token`, `version`, `spawned`.
3. Client sends `{ "type": "spawn" }` if needed.
4. Input loop sends `input` messages every 50ms when in game.

## Disconnect

- On socket close, the client resets local chunk cache and clears its input interval.
- Reconnect occurs on page reload.

## Game Over

- Client enters Game Over when its player entity is missing for too long.
- Respawn requests are sent via `spawn` without resetting the session token.
