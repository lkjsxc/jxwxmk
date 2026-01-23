# StarveRS — lightweight multiplayer survival

This repository implements a simplified multiplayer survival game inspired by starve.io.

Key points:
- Backend: Rust with Actix Web and Tokio
- Database: PostgreSQL
- Frontend: TypeScript (compiled to static JS) served by the Rust server
- Orchestration: Docker Compose

Project layout (top-level):
- `README.md` — this file
- `LICENSE` — MIT license
- `AGENTS.md` — contributor/agent notes
- `docs/` — documentation and architecture notes
- `src/` — application source (server + frontend source)

Start with Docker Compose:

1. Configure `.env` values in docker-compose or set `DATABASE_URL`.
2. Run `docker compose up --build` (builds the Rust backend and starts Postgres).

The backend serves a tiny single-file frontend at `/` and JSON APIs at `/api/*`.

This repository is intentionally minimal and structured for LLM-driven development.
