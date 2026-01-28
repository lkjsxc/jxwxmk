# Server Source

Backend application source code.

## Tech Stack

- Rust (Edition 2021)
- Actix Web (HTTP/WebSocket)
- SQLx (PostgreSQL)

## Modules

- **[protocol](protocol/README.md)**: Shared types and validation.
- **[net](net/README.md)**: HTTP and WebSocket handlers.
- **[game](game/README.md)**: Authoritative simulation engine.
- **[persistence](persistence/README.md)**: Database layer.
- **[config](config/README.md)**: Configuration loading.
