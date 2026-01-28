# Threat model

This is a pragmatic threat model for a public-facing WebSocket game server.

## Threats (what we defend against)

### 1) DoS by bandwidth or CPU

- Message floods (WS spam).
- Oversized payloads.
- Pathological JSON (deep nesting, huge arrays).
- Forcing heavy per-tick work (e.g., targeting far-away coords repeatedly).

### 2) Authority bypass

- Client claims impossible movement.
- Client triggers actions faster than cooldowns.
- Client performs PvP in safe zones.
- Client crafts without resources.

### 3) Persistence abuse

- Token brute force.
- Session hijacking (token theft).
- Corrupting JSONB blobs via malformed payloads.

### 4) Data leakage (low priority for MVP, but avoid obvious mistakes)

- Detailed errors exposing internal state.
- Exposing Postgres externally.

## Mitigations (required baseline)

- Strict schema validation on every inbound message (reject invalid/unknown types/fields).
- Server-side cooldowns and rate limits (client limits are advisory only).
- Hard caps (bytes, messages/sec, counts, world bounds).
- Single-session enforcement with token rotation.
- Postgres bound to `127.0.0.1:5432` inside the container.
- Defensive logging/metrics for abuse signals.
