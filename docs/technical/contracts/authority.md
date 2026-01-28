# Authority + Ownership Contract

## Server authority (non-negotiable)

- The server is authoritative for all gameplay outcomes.
- The client is a renderer + input device. It may predict visuals, but it must never decide outcomes.
- Any restriction enforced in the client must also be enforced on the server (security parity).

## Single-writer world ownership

- Exactly one component is allowed to mutate the world state: the tick owner (the game engine).
- Network handlers never mutate world state directly; they validate and enqueue events.
- Persistence never mutates world state directly; it provides load/save adapters and is invoked by the engine.

## Ports and adapters (hexagonal layout)

To keep modules replaceable and testable:

- **Core domain**: `world` + `systems` (pure logic; no sockets/DB).
- **Application core**: `game` (tick loop + queues; orchestrates systems; single writer).
- **Adapters**:
  - `net` (HTTP + WS): translates protocol ↔ engine events.
  - `persistence` (Postgres/sqlx): translates domain snapshots ↔ DB rows.
  - `assets` (embedded static): translates embedded bytes ↔ HTTP responses.

Adapters depend inward. The core must not depend outward.

## “Narrow handles” rule

Cross-module interaction must flow through narrow, typed handles:

- `net` holds a `GameHandle` (enqueue + subscribe), not a `&mut World`.
- `game` holds a `PersistenceHandle` (load/save), not raw SQL plumbing.

This prevents accidental authority leakage and makes unit testing possible.
