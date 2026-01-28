# 0002 — Input targeting via `aim`

## Context

The game’s interaction model is tap/click (attack/gather) and long-press/hold (interact), and building placement is described as following the cursor/target point.

Originally, the documented `input` protocol message contained only movement + booleans:

- `dx`, `dy`, `attack`, `interact`

That shape does not provide enough information for the server to authoritatively choose *which* entity/location the player intended to act on (especially for placement).

## Doc references

- `docs/technical/backend/server/protocol.md` (WS protocol message shapes)
- `docs/technical/backend/game/systems_interaction.md` (action resolution + targeting)
- `docs/technical/frontend/input/unified_input.md` (pointer tracking)
- `docs/design/world/scale_and_chunks.md` (world units and placement grid)

## Options considered

1. **No target in protocol** (server always picks nearest entity to player).
   - Pros: simplest payload.
   - Cons: mismatches “tap/click on world” UX and makes placement awkward/underspecified.

2. **Add a dedicated `target`/`place` message** with coordinates.
   - Pros: explicit per-action targeting.
   - Cons: expands protocol surface; requires additional message types and client plumbing.

3. **Extend the existing `input` message** with a single world-space target point.
   - Pros: minimal protocol expansion; supports gather/attack/interact/place consistently.
   - Cons: requires validation and clear semantics.

## Decision (chosen)

Choose **Option 3**.

Extend the `input` message with:

- `aim: { x, y }` (world-space coordinates in `wu`)

Rules:

- `aim` is **required** when `attack` or `interact` is true.
- The server validates `aim` and uses it for authoritative targeting and placement.

## Impact

- Updates the protocol definition in `docs/technical/backend/server/protocol.md`.
- Updates frontend input/runtime docs to send `aim`.
- Updates interaction system docs to use `aim` for target selection.

## Follow-ups

- If later actions require more detail (e.g., drag-to-select, area targeting), introduce additional explicit message types rather than overloading `input`.
