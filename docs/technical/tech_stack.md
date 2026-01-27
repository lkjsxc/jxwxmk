# Tech Stack

## Backend (Rust)

- **Language**: Rust (Edition 2021)
- **Web Framework**: `actix-web`
- **Actor Model**: `actix`
- **WebSockets**: `actix-web-actors`
- **Serialization**: `serde`, `serde_json`
- **IDs**: `uuid`
- **Randomness**: `rand`
- **Static Assets**: `rust-embed`, `mime_guess`
- **Logging**: `log`, `env_logger`

## Frontend (TypeScript)

- **Language**: TypeScript (ES2020 target)
- **Bundler**: `esbuild` (build-time only)
- **Rendering**: HTML5 Canvas 2D
- **Networking**: Native `WebSocket`

## Infrastructure

- **Containerization**: Docker + Docker Compose
- **Runtime Base**: Debian slim
- **Database**: PostgreSQL 15 (running inside the same container)
