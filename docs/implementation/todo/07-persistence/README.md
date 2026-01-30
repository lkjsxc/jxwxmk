# 07 — Persistence (Postgres + sqlx + checkpointing)

Goal: implement durable continuity: players, sessions, settlements, and chunk deltas in PostgreSQL.

References:
- `docs/technical/backend/persistence/README.md`
- `docs/technical/backend/database/README.md`
- `docs/technical/backend/database/schema.md`

## A) Migrations

- [ ] Add SQL migrations for:
  - `players`
  - `settlements`
  - `chunks`
- [ ] Ensure migrations apply at server startup inside the container.

## B) Sessions + player state

- [ ] `POST /session/claim` rotates the stored token for the player id.
- [ ] Old sessions are revoked (live WebSocket receives `sessionRevoked`).
- [ ] On WebSocket join:
  - [ ] validate token
  - [ ] load player state (or create a new row if allowed)
- [ ] Persist player state:
  - [ ] on disconnect
  - [ ] on a fixed interval (coalesced; never per tick)

## C) Settlements + chunks

- [ ] Load persisted settlements and chunk deltas on startup.
- [ ] Persist settlements/chunks on a fixed interval (checkpoint strategy).
- [ ] Never write per-tick; coalesce changes.

## Done when

- [ ] A player can reconnect with the same token and resume state.
- [ ] The world persists basic deltas (placed structures, depleted nodes, settlement core state).
