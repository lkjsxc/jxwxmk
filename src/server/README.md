# Server Layer

HTTP/WebSocket hosting, session management, static asset serving, and database persistence.

## Contents

- `mod.rs`: server module wiring.
- `http.rs`: HTTP routes and handlers.
- `ws.rs`: WebSocket route and handshake.
- `session.rs`: `GameSession` actor.
- `registry.rs`: session registry and revoke flow.
- `database.rs`: PostgreSQL pool + persistence helpers.
- `static_assets.rs`: `rust-embed` asset serving.
- `rate_limit.rs`: lightweight rate limiting utilities.
