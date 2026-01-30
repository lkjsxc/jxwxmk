# Source Tree

The `src/` directory contains all implementation code for the game.

## Structure

- [`runtime/`](runtime/): Docker runtime and entrypoint
- [`static/`](static/): Static assets embedded in the Rust binary
- [`client/`](client/): TypeScript client (build-time only)
- [`server/`](server/): Rust workspace with server crates

## Build Flow

1. TypeScript client builds to `static/game.js`
2. Rust server embeds `static/` via rust-embed
3. Docker container includes only the compiled binary
