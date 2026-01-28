# 0003 — Protocol v3 + structured errors

## Context

The original protocol specification required strict validation but did not define a machine-readable way for the client to learn *why* a message was rejected.
This tends to produce “silent failure” UX and makes debugging hard.

Additionally, protocol examples used inconsistent identifier styles (e.g., CamelCase recipe/item IDs).

## Doc references

- `docs/technical/backend/server/protocol.md`
- `docs/technical/contracts/protocol.md`
- `docs/technical/backend/server/http_ws.md` (endpoints)
- `docs/technical/frontend/runtime.md` (client handling)

## Options considered

1. Reuse `notification` for all errors (text-only).
   - Pros: no new message type.
   - Cons: not machine-readable; hard to build good UI reactions; ambiguous semantics.

2. Add a structured `error` message with a stable `code`.
   - Pros: modern API ergonomics; enables clear client behavior; debuggable.
   - Cons: introduces a new message type and requires client updates.

## Decision (chosen)

Choose option 2:

- Add `{ "type": "error", "data": { "code", "message", "details" } }` as a server→client message.
- Bump `welcome.version` to **3** to reflect the protocol surface change.
- Standardize identifiers inside message payloads as `snake_case` strings (recipe IDs, item IDs, quest IDs, etc.).

## Impact

- Updates protocol docs and frontend runtime docs.
- Requires backend to emit structured errors for rejected inputs (or disconnect if abusive).
- Enables better UI feedback and improved testability for validation behavior.

## Follow-ups

- If/when backward compatibility becomes a goal, define explicit version negotiation; for now the client and server ship together.
