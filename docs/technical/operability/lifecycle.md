# Lifecycle (Startup + Shutdown)

## Startup sequence (canonical)

1. Initialize logging.
2. Load and validate config (`/app/config/*.json`).
3. Start PostgreSQL (runtime entrypoint responsibility).
4. Connect to DB and apply migrations.
5. Construct `GameEngine` (world seed, chunk manager, settlement generation).
6. Start the fixed tick loop.
7. Start HTTP/WS server.

## Shutdown sequence (graceful)

On SIGTERM/SIGINT:

1. Stop accepting new connections.
2. Disconnect WebSocket clients cleanly (best-effort).
3. Stop the tick loop.
4. Flush a final persistence checkpoint (players + dirty chunks/settlements).
5. Shut down HTTP server.

## Failure policy

- If config validation fails: fail fast at startup (do not run with unknown behavior).
- If migrations fail: fail fast at startup.
- If persistence fails during runtime:
  - log and emit metrics
  - continue simulation if safe
  - never corrupt state or panic the tick loop
