# Net Crate

HTTP and WebSocket server with session management.

## Modules

- `server.rs`: HTTP routes, Actix server setup
- `session.rs`: WebSocket session actor
- `metrics.rs`: Prometheus metrics
- `handlers.rs`: Protocol message handlers

## Routes

- `GET /health`: Health check
- `GET /metrics`: Prometheus metrics
- `POST /session/claim`: Token rotation
- `GET /ws`: WebSocket endpoint
- `GET /`: Static assets
