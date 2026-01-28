# Source Tree

This directory contains all source code for the game: Rust server, TypeScript client, static assets, and runtime configuration.

## Structure

- `runtime/` - Docker runtime configuration and entrypoint
- `static/` - Static assets (HTML, CSS, JS) embedded into the server binary
- `client/` - TypeScript client source code (builds to `static/game.js`)
- `server/` - Rust workspace with server crates

## Entry Points

- **Server binary**: `server/src/main.rs`
- **Client bundle**: `client/index.ts` → `static/game.js`
- **Runtime container**: `runtime/Dockerfile` + `runtime/entrypoint.sh`

## Build Flow

1. TypeScript client builds via esbuild to `static/game.js`
2. Rust server embeds `static/` via rust-embed
3. Docker multi-stage build: Node → Rust → Runtime
4. Runtime container starts PostgreSQL + server

See subdirectory READMEs for detailed module documentation.
