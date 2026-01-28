# 04 — Backend (HTTP/WS + protocol + assets)

Goal: implement the Actix server, the documented endpoints, and strict protocol handling.

References:
- `docs/technical/backend/server/overview.md`
- `docs/technical/backend/server/http_ws.md`
- `docs/technical/backend/server/protocol.md`
- `docs/technical/backend/server/static_assets.md`
- `docs/decisions/0002-input-aim.md`
- `docs/technical/contracts/protocol.md`
- `docs/technical/operability/metrics.md`

## A) HTTP routes

- [x] `GET /health` returns `200 OK` with body `OK`.
- [x] `GET /metrics` returns `200 OK` with Prometheus text format.
- [ ] `POST /session/claim`:
  - accepts `{ "player_id": "<uuid>" }`
  - rotates the session token for that player
  - enforces the single-session rule (revokes any live session)
  - rate-limits to prevent brute force
- [x] `GET /` and `GET /{filename}` serve embedded assets from `src/static/`.

## B) Default security headers

- [ ] Inject headers as documented in `docs/technical/backend/server/http_ws.md`:
  - `Content-Security-Policy` (self-only; allow unsafe-inline styles and unsafe-eval for bundle)
  - `X-Content-Type-Options: nosniff`
  - `X-Frame-Options: DENY`

## C) WebSocket route + handshake

- [x] `GET /ws?token=<uuid>` establishes a WebSocket connection.
- [ ] Token handling:
  - no token: new session is created (or claimed flow required; decide and document if unclear)
  - token present: validate and reattach to existing player state
- [x] Server sends `welcome` exactly as documented:
  - `{ "type": "welcome", "id": "...", "token": "...", "version": 3, "spawned": false }`
- [ ] Single-session enforcement:
  - when a new token is issued, the old session receives `sessionRevoked` then disconnects

## D) Protocol types + validation

- [x] Implement all client→server messages from `docs/technical/backend/server/protocol.md`:
  - `input` (includes `aim` when action booleans are true)
  - `spawn`, `craft`, `trade`, `npcAction`, `acceptQuest`, `slot`, `swapSlots`, `name`
- [x] Implement all server→client messages:
  - `welcome`, `sessionRevoked`, `chunkAdd`, `chunkRemove`, `entityDelta`, `achievement`, `notification`, `error`, `npcInteraction`, `questUpdate`
- [x] Strict inbound validation:
  - reject unknown message types
  - reject missing/invalid fields
  - enforce numeric bounds and cooldown/rate limits server-side
- [x] Rejected inputs yield structured errors (`error.code` uses the baseline contract set; `error.message` is user-facing).
- [x] Identifier convention is enforced:
  - all protocol IDs (recipes, items, achievements, quests, subtypes) are `snake_case`.

## E) Static assets (rust-embed)

- [x] Embed `src/static/` into the Rust binary with `rust-embed`.
- [x] Implement `serve_index` (`/`) and `serve_asset` (`/{filename}`) per `docs/technical/backend/server/static_assets.md`.
- [x] Use `mime_guess` for MIME type inference.

## Done when

- [x] A browser can load `/` and establish `/ws`.
- [x] Handshake and spawn flow works end-to-end with strict validation.
