# HTTP + WebSocket Endpoints

## HTTP

- `GET /health`
  - Returns `200 OK` with body `OK`.

- `GET /` and `GET /{filename}`
  - Serves embedded static assets from the `static/` directory.

## Default Headers

The server injects basic security headers:

- `Content-Security-Policy` (self-only, allows unsafe-inline for styles and unsafe-eval for bundle)
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`

## WebSocket

- `GET /ws?token=<uuid>`
  - Establishes a WebSocket connection and starts a `GameSession` actor.
  - Optional `token` parameter reattaches to an existing player entity.

### WebSocket Handshake Flow

1. Client connects to `/ws` with optional `token` query.
2. Server sends a `welcome` message containing:
   - `id`: player UUID
   - `token`: session token to store client-side
   - `version`: protocol version
   - `spawned`: whether the player is already in-world
3. Client sends a `spawn` message to enter the world if needed.
