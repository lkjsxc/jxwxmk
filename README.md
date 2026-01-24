# Starve.io-like Multiplayer Survival Game

## Overview
This project builds a starve.io-like multiplayer survival game as a single integrated solution using Rust (server + static file serving), PostgreSQL (persistence), and TypeScript (build-time client compilation). No long-running Node service; Docker Compose runs only Rust and Postgres containers.

Key features:
- Server-authoritative simulation with fixed-rate tick loop (20-60 Hz).
- Real-time multiplayer via WebSocket + optional HTTP endpoints.
- Survival mechanics: gathering, crafting, hunger/thirst, combat, biomes.
- Simple graphics: Canvas 2D with primitives/shapes for "rich content" via systems.
- Persistence: accounts, inventory, equipment; minimal transient state.

## Tech Stack
- **Server**: Rust (Tokio, Actix Web, WebSockets).
- **Database**: PostgreSQL (migrations, schema).
- **Client**: TypeScript (compiled to JS/CSS, served by Rust).
- **Ops**: Docker Compose (build-time TS compilation).

## Runtime Constraints
- `docker compose up` starts: 1 Rust service, 1 Postgres.
- No extras (no Redis, no separate frontend).

## Development
See AGENTS.md for rules and conventions. Commit often; no backward compatibility.

## Build and Run
- `docker compose up --build` to run the game.
- Client at http://localhost:8080/static/index.html
- WS at ws://localhost:8080/ws

## Directories
- `docs/`: Documentation (architecture, protocol, gameplay, operations, decisions).
- `src/`: Source code (server, client, assets, db, ops).