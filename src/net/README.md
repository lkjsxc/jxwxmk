# Net Module

Handles network I/O, sessions, and protocol orchestration.

## Responsibilities
- HTTP endpoints (Health, Static, Auth).
- WebSocket connection handling.
- Session token management.
- Deserialization and Enqueueing of input.

## Dependencies
- `protocol` (message types).
- `game` (to enqueue events).
- `config` (ports, limits).
