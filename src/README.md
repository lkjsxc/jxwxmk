# Source Root

This directory contains the implementation of the game server and client.
It is organized as a Rust workspace with a TypeScript frontend.

## Directory Structure

- **`protocol/`**: Shared types (JSON messages) and validation logic. [Crate]
- **`config/`**: Configuration loading and schemas. [Crate]
- **`world/`**: Spatial index, chunks, and entity data structures. [Crate]
- **`systems/`**: Pure gameplay logic (crafting, survival, combat). [Crate]
- **`persistence/`**: Database adapter (PostgreSQL) and migrations. [Crate]
- **`game/`**: The game engine, tick loop, and event orchestration. [Crate]
- **`net/`**: Network layer (HTTP/WebSocket) and session management. [Crate]
- **`server/`**: The application entrypoint (binary). [Crate]
- **`client/`**: TypeScript source for the browser client.
- **`static/`**: Compiled assets and HTML served by the server.
- **`runtime/`**: Docker build and runtime configuration.

## Build

See `../docs/setup/README.md` for build instructions.
