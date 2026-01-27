# Source Root

Rust server, TypeScript client build pipeline, static assets, and runtime container assets.

## Contents

- `Cargo.toml` / `Cargo.lock`: Rust crate manifest.
- `main.rs`: server entrypoint.
- `config/`: config loading and schema types.
- `protocol/`: WebSocket JSON message types.
- `server/`: HTTP/WebSocket hosting and persistence glue.
- `game/`: authoritative simulation (engine, world, systems).
- `client/`: TypeScript client sources (build-time only).
- `static/`: embedded runtime assets (index.html, styles, game.js).
- `runtime/`: Dockerfile + entrypoint for single-container runtime.
- `tests/`: integration tests (run in Docker).
