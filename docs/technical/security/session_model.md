# Session model

References:
- `docs/technical/backend/server/http_ws.md`
- `docs/technical/backend/database/README.md`

## Identifiers

- `player_id`: UUID (public, user-visible).
- `token`: UUID (secret bearer token; store client-side; rotate on claim).

## Claim flow (HTTP)

- Client calls `POST /session/claim` with `{ "player_id": "<uuid>" }`.
- Server rotates the stored `token` for that player ID.
- Any existing live session for that player is revoked:
  - send `sessionRevoked`
  - disconnect

## Connect flow (WebSocket)

- Client connects to `/ws?token=<uuid>` (token optional if first-time flow is supported).
- Server validates the token against the DB and enforces the single-session rule.
- Server sends `welcome` with the server protocol version and `spawned` flag.
- Client sends `spawn` if needed.

## Revocation semantics

`sessionRevoked` is reserved for security/session lifecycle events:

- login elsewhere (token rotation)
- invalid token (optional; may also be silent disconnect)
- server-side admin action (future)
