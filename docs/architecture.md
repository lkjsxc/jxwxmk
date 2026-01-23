# Architecture

- Backend: Rust + Actix Web (websocket + REST), tokio runtime, `sqlx` for Postgres.
- Frontend: JavaScript compiled for browsers; a small canvas UI and WebSocket client.
- Data: Postgres stores persistent player/account data; game state mostly in-memory for speed.
- Orchestration: Docker Compose spins up Postgres + server. The server serves static client files.

Design notes:
- Use small timestep broadcasting (e.g., 10–20 TPS) for position updates.
- Server keeps authoritative state; clients send input deltas.
- Simple physics: position, velocity, hunger timer, and resource gathering.
