# Troubleshooting

Common issues observed during reconstruction and how to diagnose them.

## WebSocket disconnects after idle

Symptoms:

- Connection drops after a short period of inactivity.

Checks:

- Confirm the client is sending `input` keepalives while idle (see: `docs/technical/backend/server/protocol.md`).
- Confirm server idle timeout configuration:
  - `server.limits.ws_idle_timeout_secs`
  - `server.limits.ws_heartbeat_interval_secs`

## World entities do not appear (only UI updates)

Symptoms:

- HUD updates, but trees/rocks/mobs/structures are missing.

Checks:

- In browser DevTools, confirm `chunkAdd` messages arrive and are parsed.
- Confirm `chunkAdd.data.entities.*` are arrays (not maps/objects).
- Confirm the client maintains a chunk cache keyed by `[cx, cy]` and applies `entityDelta` updates to the correct chunk.

## Camera does not follow the player

Symptoms:

- The player can “move” logically (e.g., stats change), but the camera stays fixed.

Checks:

- Confirm `playerUpdate.data` includes `x` and `y` (authoritative local-player position).
- Confirm the camera target updates from `playerUpdate` (not from client-side prediction).

## Input feels unresponsive

Checks:

- Ensure gameplay bindings are disabled when a text input is focused and that pressed-key state is cleared on window blur (see: `docs/technical/frontend/input/keyboard.md`).
- Confirm `aim` is included in `input` when `attack`/`interact` is true.
- Confirm the server rejects invalid `aim` with a structured `error` message (`invalid_aim`) instead of silently ignoring.

## Docker build failures (type mismatches / protocol drift)

Checks:

- Ensure chunk coordinates are serialized as `[i32; 2]` in JSON protocol messages (not a tuple).
- Re-run the docs-driven protocol examples and validate that the client/server agree on:
  - recipe IDs (`snake_case`)
  - `chunkAdd` entity container types (arrays)
  - `playerUpdate` required fields (`x`, `y`)

## Observability quick checks

- `GET /health` returns `OK`.
- `GET /metrics` returns Prometheus text (see: `docs/technical/operability/metrics.md`).
- Browser DevTools → Network → WS shows continuous `input` traffic during gameplay and server→client world replication (`chunkAdd`/`entityDelta`).
