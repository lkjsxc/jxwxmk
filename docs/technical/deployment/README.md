# Deployment

## Contents

- [CI (GitHub Actions)](ci.md)

## Strategy

A single runtime container runs both:

- Rust game server (HTTP + WebSocket + embedded static assets)
- PostgreSQL 15

## Container Responsibilities

- Build pipeline compiles TypeScript assets and embeds them in the Rust binary.
- Runtime starts PostgreSQL locally and then launches the game server.

## Runtime Ports

- Game server: `0.0.0.0:8080` (exposed)
- PostgreSQL: `127.0.0.1:5432` (internal only)

## Build Stages

1. **Frontend build** (Node): `esbuild` bundles `src/client/index.ts` to `src/static/game.js`.
2. **Backend build** (Rust): `cargo build --release` embeds `static/` assets.
3. **Runtime** (Debian): installs PostgreSQL and copies the binary.
