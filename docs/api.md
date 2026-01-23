# API

HTTP endpoints (served by Actix Web):

- `GET /` - serves frontend
- `GET /api/health` - simple health check
- `POST /api/login` - create or fetch player, returns a player token
- `GET /ws` - websocket upgrade for real-time game

WebSocket messages are JSON lines. The protocol is intentionally small for now.
