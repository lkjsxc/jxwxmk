# Networking

Handles I/O boundaries.

## Responsibilities

- HTTP endpoints (`/health`, `/session/claim`).
- WebSocket handshake and actor management.
- Static asset serving.
- **Constraint**: Must not mutate `World` directly; enqueues events only.
