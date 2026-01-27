# Network Stability + Lifecycle

## Session Lifecycle

1. Optional: client requests a new session token via `POST /session/claim` with `player_id`.
2. Client opens WebSocket `/ws` with optional `token`.
3. Server sends `welcome` with `id`, `token`, `version`, `spawned`.
4. Client sends `{ "type": "spawn" }` if needed.
5. Input loop sends `input` messages every 50ms when in game.

## Disconnect

- On socket close, the client resets local chunk cache and clears its input interval.
- Reconnect occurs on page reload.
- If a `sessionRevoked` message arrives, the client clears the stored token and shows a login prompt.

## Game Over

- Client enters Game Over when its player entity is missing for too long.
- Respawn requests are sent via `spawn` without resetting the session token.
