# Source Code

The implementation of the game server and client.

## Structure

- `crates/`: Modular Rust workspace members (the bulk of the backend logic).
- `bin/`: The executable binary (entrypoint) that composes the crates.
- `client/`: TypeScript frontend (builds to `static/game.js`).
- `static/`: Assets served by the backend (images, built JS).
- `runtime/`: Docker/deployment artifacts.
