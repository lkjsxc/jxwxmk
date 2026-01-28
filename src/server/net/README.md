# Net Crate

HTTP + WebSocket handlers and session management.

## Purpose

- HTTP routes: `/health`, `/metrics`, `/session/claim`, static assets
- WebSocket route: `/ws?token=...`
- Session registry for single-session enforcement

## SessionRegistry

Manages active WebSocket connections:
- `register()` - Add new session
- `unregister()` - Remove disconnected session
- `revoke_session()` - Send `sessionRevoked` to existing session

## GameSession

Actor handling individual WebSocket connections:
- Parses JSON messages
- Sends protocol responses
- Handles heartbeat/ping-pong

## Single-Session Enforcement

When a new token is issued via `/session/claim`, the old session receives `SessionRevoked` message and disconnects.
