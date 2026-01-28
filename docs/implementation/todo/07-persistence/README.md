# 07 — Persistence (Postgres + sqlx + checkpointing)

Goal: implement durable continuity: players, sessions, settlements, and chunk deltas in PostgreSQL.

References:
- `docs/technical/backend/persistence/README.md`
- `docs/technical/backend/database/README.md`
- `docs/technical/backend/database/schema.md`

## A) Migrations

- [x] Add SQL migrations for:
  - `players`
  - `settlements`
  - `chunks`
- [x] Ensure migrations apply at server startup inside the container.

## B) Sessions + player state

- [x] `POST /session/claim` rotates the stored token for the player id.
- [x] Old sessions are revoked (live WebSocket receives `sessionRevoked`).
- [x] On WebSocket join:
  - [x] validate token
  - [x] load player state (or create a new row if allowed; decide and document)
- [x] Persist player state:
  - [x] on disconnect
  - [x] on a fixed interval (coalesced; never per tick)

## C) Settlements + chunks

- [ ] Load persisted settlements and chunk deltas on startup.
- [x] Persist settlements/chunks on a fixed interval (checkpoint strategy).
- [x] Never write per-tick; coalesce changes.

## Done when

- [x] A player can reconnect with the same token and resume state.
- [x] The world persists basic deltas (placed structures, depleted nodes, settlement core state).
