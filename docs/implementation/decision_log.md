# Decision Log

This repository is docs-first and agent-driven. When documentation is ambiguous or contradictory, agents must not stall.

This file is the single place to record **minimal, blocking decisions** made during reconstruction or refactors.

## Rules

- Prefer updating canonical specs directly when the docs are simply incomplete.
- Use this log when you must choose between reasonable options or reconcile conflicts.
- Entries must be small, explicit, and traceable to doc sections.
- **Most recent relevant entry wins** if multiple entries touch the same topic.
- After writing an entry, apply it to canonical spec docs so this log is not the only place the contract exists:
  - `docs/technical/backend/server/protocol.md`
  - `docs/technical/contracts/*`
  - `docs/technical/config/schemas/*`
  - `docs/implementation/reconstruction_acceptance.md` and `docs/implementation/todo/` when needed

## Entry format (copy/paste)

## DL-0001 — <short title>

### Context

- What was unclear / conflicting?
- Why does it block implementation?

### References

- `docs/...` (file paths + the relevant section headings)

### Options

1. Option A
2. Option B

### Decision

- Choose the minimal, doc-consistent option that unblocks progress.

### Impact

- Docs: what files were updated (or must be updated)
- Code: what areas/modules are affected
- Tests: what must prove the behavior

---

## DL-0001 — `playerUpdate` includes authoritative local-player position (`x`, `y`)

### Context

The client must be able to follow the local player immediately and authoritatively. Prior documentation defined `playerUpdate` as “private state only” and did not include position fields, which created an easy failure mode where the client had no reliable local position early in a session.

### References

- `docs/technical/backend/server/protocol.md` → `playerUpdate`
- `docs/technical/frontend/runtime.md` → “Render Loop”
- `docs/technical/backend/game/entities.md` → “Synchronization boundary”

### Options

1. Keep local-player position only in `entityDelta` and require the client to derive camera follow from the entity stream.
2. Include `x`, `y` in `playerUpdate` for the session owner and drive camera follow from it.

### Decision

Choose option 2.

### Impact

- Docs: `playerUpdate` now includes `x`, `y` and documents tick-rate cadence while spawned.
- Code: server must populate `playerUpdate.data.x/y` and keep it authoritative; client camera follow uses those fields.
- Tests: protocol validation tests must require `x`/`y` in `playerUpdate`.

## DL-0002 — Idle keepalive uses `input` messages + configurable WS idle timeout

### Context

The server enforces an application-level idle timeout. If the client sends `input` only when movement/actions are active, an idle player can be disconnected even though the socket is healthy.

### References

- `docs/technical/backend/server/http_ws.md` → “Keepalive / idle timeout”
- `docs/technical/backend/server/protocol.md` → `input`
- `docs/technical/contracts/protocol.md` → “Keepalive / idle timeout”
- `docs/technical/frontend/input/stability.md` → “Session Lifecycle”

### Options

1. Introduce explicit protocol keepalive messages (`ping`/`pong`).
2. Use the existing `input` message as the keepalive frame (idle `dx=0, dy=0`).

### Decision

Choose option 2 (minimal protocol surface).

### Impact

- Docs: `input` cadence is specified as ~50ms during gameplay, including idle keepalive frames; `server.json` documents idle timeout parameters.
- Code: WS session idle checks must be based on time since the last received client WebSocket frame (text/ping/pong).
- Tests: integration tests should ensure idle keepalive traffic does not trigger disconnect under default `server.json`.

## DL-0003 — Touch mapping uses dynamic “gesture vs joystick” interpretation

### Context

Strict left-joystick/right-gesture zoning makes one-hand play awkward and caused implementation confusion. A single touch should become a joystick when dragged, but remain a gesture (tap/long-press) when stationary.

### References

- `docs/technical/frontend/input/touch.md`
- `docs/design/ux/controls/mobile.md`
- `docs/technical/frontend/input/unified_input.md`

### Options

1. Keep strict zones: left half for joystick; right half for gestures.
2. Make the mapping dynamic: drag becomes joystick; stationary becomes tap/long-press, regardless of where it started.

### Decision

Choose option 2 with a small movement threshold (`~12px`) to avoid accidental joystick activation.

### Impact

- Docs: touch rules clarify mutual exclusion between joystick and gestures for a given touch identifier.
- Code: input manager must cancel tap/long-press detection once a touch becomes a joystick.
- Tests: client input unit tests should cover threshold behavior (stationary tap vs drag).

## DL-0004 — Recipe identifiers are `snake_case` everywhere

### Context

The crafting surface spans protocol (`craft`), server config (`crafting.json`), and client UI. Docs drifted: some UI docs used PascalCase recipe IDs while schemas and protocol used `snake_case`.

### References

- `docs/technical/backend/server/protocol.md` → `craft`
- `docs/technical/config/schemas/crafting.md`
- `docs/technical/frontend/ui/crafting.md`
- `docs/design/mechanics/crafting/recipes.md`

### Options

1. Switch protocol/config to PascalCase.
2. Fix UI docs to match the schema/protocol `snake_case` convention.

### Decision

Choose option 2.

### Impact

- Docs: all recipe IDs are documented as `snake_case`.
- Code: client sends `craft.data.recipe` using the `crafting.json` recipe `id`.
- Tests: crafting protocol validation rejects non-`snake_case` recipe IDs.
