# Network Stability & Session Lifecycle

Procedures for handling connection errors, latency, and session resets.

## Session Lifecycle

### 1. Connection Handshake
- Client requests `ws://` with optional `token`.
- Server responds with `welcome` packet.
- Client enters `InGame` state.

### 2. Active Session
- **Input Loop**: 20Hz (50ms).
- **Update Loop**: Server broadcasts world state.
- **Heartbeat**: Standard WebSocket ping/pong managed by Actix.

### 3. Termination
- **Intentional (Logout)**: User closes tab or returns to menu. Socket is closed gracefully.
- **Unintentional (Drop)**: Client detects `onclose` and returns to `StartScreen`.
- **Game Over (Death)**: Server removes entity. Client transitions to `GameOver` state.

## Respawn & Cleanup
To avoid network flooding or redundant reloads:
1. Set `respawnRequest = false` immediately.
2. Gracefully close the WebSocket.
3. Clear all active intervals (`sendInput`).
4. Invoke `location.reload()`.
