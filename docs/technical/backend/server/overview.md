# Server Overview

## Process Model

- Single Rust binary built with Actix Web.
- One HTTP server instance with one worker (`workers(1)`), intentionally low memory.
- `GameEngine` is an Actix actor that owns world state and runs the tick loop.
- Each WebSocket connection runs a `GameSession` actor that forwards input to `GameEngine` and receives outbound messages.

## Startup Flow

1. `main` initializes logging and constructs `GameEngine::new()`.
2. `GameEngine` loads `config.json` and initializes the `World`.
3. `GameEngine` starts and begins its fixed-rate tick loop.
4. Actix Web binds `0.0.0.0:8080` and exposes routes:
   - `GET /health`
   - `GET /ws`
   - `GET /` and `GET /{filename}` for static assets

## Actor Responsibilities

- **GameEngine**:
  - Owns `World` state and configuration.
  - Processes input messages (movement, attack, craft, spawn, etc.).
  - Runs the tick loop at `tick_rate`.
  - Broadcasts full world snapshots each tick.

- **GameSession**:
  - Handles JSON message parsing from WebSocket.
  - Sends `Join` on start and `Leave` on stop.
  - Forwards input messages to `GameEngine` without mutating state directly.
