# Protocol Contract (Versioning, Errors, Limits)

This document defines the **global rules** for the WebSocket/HTTP protocol.
Message shapes live in `docs/technical/backend/server/protocol.md`; this file defines cross-cutting semantics.

## Versioning

- The server is authoritative for the protocol version.
- `welcome.version` is the protocol version implemented by the server.
- If the client cannot support the server version, it must fail closed (show a blocking screen and stop sending gameplay messages).

## Envelope

Client → server:

- Every message is a JSON object: `{ "type": "<string>", "data": <object> }`.
- Unknown `type` values are rejected.

Server → client:

- Server messages use the same envelope pattern, except `welcome` which includes top-level fields as documented.
- The client must treat server messages as authoritative and should ignore unknown server→client message types (useful when docs evolve), as long as `welcome.version` is supported.

## Public vs private state (privacy boundary)

The protocol separates:

- **Public world state**: chunk membership and entity visibility (`chunkAdd` / `chunkRemove` / `entityDelta`).
- **Private player state**: owner-only state that must never be broadcast to other players (`playerUpdate`).

Examples of private state:

- inventory contents and counts
- hunger/temperature meters
- quest log and achievements set

This boundary is required to prevent data leakage and unnecessary bandwidth.

## Strictness and unknown fields

To prevent silent drift:

- The server rejects unknown message types.
- The server rejects invalid message payloads (wrong types, missing required fields).
- The server may reject unknown fields in `data` (recommended: `deny_unknown_fields` on inbound types).

## Limits (must be enforced server-side)

The server must enforce hard caps to prevent abuse:

- max WS message bytes
- max messages per second (and burst)
- max string lengths (names, IDs)
- max counts in arrays/maps (inventory size, entity delta counts)

The caps are configured via `server.json` (see: `docs/technical/config/schemas/server.md`).

## Error model (modern baseline)

Validation failures must be visible to the client in a machine-readable way.

- Use a structured error message:
  - `{ "type": "error", "data": { "code": "<string>", "message": "<string>", "details": <object|null> } }`
- `code` is stable and used by the UI to decide how to react.
- `message` is user-facing text suitable for a toast.
- `details` is optional and may be omitted in production if it risks leaking info.

Fatal auth/session failures continue to use `sessionRevoked` (and disconnect).

### Error codes (baseline contract)

The following `error.code` values are the baseline set. Add new codes only when documented.

- `invalid_message`: JSON parse failed or message envelope invalid.
- `unknown_type`: unknown `type` string.
- `invalid_payload`: payload failed schema validation (wrong types, missing required fields, unknown fields).
- `rate_limited`: exceeded WS/HTTP rate limits.
- `cooldown`: gameplay action rejected due to server-side cooldown.
- `not_spawned`: action rejected because the player is not spawned.
- `invalid_aim`: missing/invalid `aim` or out-of-range target.
- `invalid_slot`: invalid inventory/hotbar index.
- `insufficient_items`: crafting/trade requires items the player does not have.
- `server_busy`: server refuses a request due to overload/backpressure.

## Correlation (debuggability)

For debug builds, the server may include a `request_id` field inside `error.details` and in logs so issues can be traced end-to-end.
