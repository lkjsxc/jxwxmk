# StarveRS — a small MMO survival prototype

This repository contains an LLM-driven prototype of a simple survival multiplayer game inspired by starve.io.

- Server: Rust (Actix Web + tokio + sqlx/Postgres)
- Client: TypeScript (browser canvas + WebSocket)
- Orchestration: Docker Compose (Postgres + Server)

Start (Linux):

```
DOCKER_HOST="unix://$XDG_RUNTIME_DIR/docker.sock" docker compose up --build
```

See `docs/` for architecture and play instructions.
