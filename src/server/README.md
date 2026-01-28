# Server Module (Binary)

The application entrypoint.

## Responsibilities
- Parse command line args.
- Initialize logging/metrics.
- Load config.
- Start Database pool.
- Start Game Engine loop (background task).
- Start HTTP/WS Server (main task).

## Dependencies
- All other crates.
