# Source Tree

Root of the reconstructed runtime and build artifacts.

## Contents

- `server/`: Rust game server crate (authoritative simulation + HTTP/WebSocket + persistence).
- `client/`: TypeScript client sources (build-time only).
- `static/`: Embedded runtime assets served by the Rust server.
- `runtime/`: Dockerfile and entrypoint for the single-container runtime.
- `Cargo.toml`: Workspace manifest (server crate only).
