# Tech Stack

## Backend (Rust)

- **Language**: Rust (Edition 2021+)
- **Async Runtime**: `tokio`
- **Web Framework**: `actix-web`
- **WebSockets**: `actix-web-actors`
- **Database Driver**: `sqlx` (PostgreSQL)
- **Serialization**: `serde`, `serde_json`

## Frontend (TypeScript)

- **Language**: TypeScript
- **Rendering**: HTML5 Canvas API (2D)
- **Bundler**: `esbuild` (or similar simple build step run by cargo or script)
- **Communication**: Native `WebSocket` API

## Infrastructure

- **Containerization**: Docker, Docker Compose
- **Database**: PostgreSQL 16
