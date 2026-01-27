# Server Overview

## Process Model

- Single Rust binary built with Actix Web.
- One HTTP server instance with one worker (`workers(1)`), intentionally low memory.
- `GameEngine` owns world state, chunk cache, and tick loop.
- Each WebSocket connection runs a `GameSession` actor that forwards input to `GameEngine`.
- A session registry enforces one active session per player ID.

## Startup Flow

1. `main` initializes logging and constructs `GameEngine::new()`.
2. `GameEngine` loads config files from `config/` and initializes the world seed.
3. `GameEngine` starts the fixed-rate tick loop.
4. Actix Web binds `0.0.0.0:8080` and exposes routes:
   - `GET /health`
   - `GET /ws`
   - `GET /` and `GET /{filename}` for static assets

## Actor Responsibilities

- **GameEngine**:
  - Owns world state and configuration.
  - Processes validated input events.
  - Runs the tick loop at `tick_rate`.
  - Streams chunk deltas to clients.

- **GameSession**:
  - Parses JSON messages.
  - Sends `Join` on start and `Leave` on stop.
  - Enqueues input events without mutating state.
