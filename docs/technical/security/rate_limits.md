# Rate limits + abuse controls

Rate limits are required at multiple layers: HTTP, WebSocket, and gameplay actions.

References:
- `docs/technical/backend/server/http_ws.md`
- `docs/technical/contracts/tick.md` (bounded queues)
- `docs/technical/config/schemas/server.md` (rate limit configuration)

## A) HTTP rate limits

- `POST /session/claim` must be rate-limited per IP.
- Reject or delay requests that exceed the limit.

## B) WebSocket rate limits

The server must enforce:

- max message bytes
- max messages per second
- burst capacity
- max parse depth / max array/map sizes (implementation-specific; do not allow unbounded structures)

On repeated violations:

- send `error` with a stable code (see protocol contract), then disconnect.

## C) Gameplay cooldowns (server-side)

Client cooldowns are advisory only. The server enforces:

- attack cooldown
- interact cooldown
- crafting rate limit
- trade/NPC action rate limit

Cooldown configuration should be in `balance.json` or `server.json` (be explicit; do not hide constants).
