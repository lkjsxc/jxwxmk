# Architecture

StarveRS is a small multiplayer survival game server with a minimal web client.

Components:
- Server (Rust + Actix Web): HTTP API, websocket game loop, static file server
- Database (Postgres): persistent player data and world seed
- Frontend (TypeScript): simple canvas renderer and input handling, compiled to static assets served by the server
- Docker Compose: wires database and server together for local dev

Game loop: ticked on the server (Tokio) and sends periodic state snapshots to clients over WebSocket. Clients render simplified top-down 2D view.
