# `server.json`

Purpose: server runtime parameters (HTTP/WS ports, tick rate, protocol version, and hard limits).

## Schema (v1)

```json
{
  "version": 1,
  "bind_http": "0.0.0.0:8080",
  "protocol_version": 3,
  "tick_rate": 30,
  "limits": {
    "ws_max_message_bytes": 16384,
    "ws_messages_per_sec": 30,
    "ws_burst": 60,
    "max_name_len": 24
  },
  "rate_limits": {
    "session_claim_per_ip_per_minute": 10
  }
}
```

## Validation rules

- `bind_http`: must be a valid socket bind string.
- `protocol_version`: integer; must match the server implementation.
- `tick_rate`: integer in `[20, 60]` (recommended); must be > 0.
- `ws_max_message_bytes`: integer > 0.
- `ws_messages_per_sec`: integer > 0.
- `ws_burst`: integer >= `ws_messages_per_sec`.
- `max_name_len`: integer in `[1, 64]` (recommended).
- `session_claim_per_ip_per_minute`: integer > 0.

## Notes

- Hard limits must be enforced server-side (see: `docs/technical/contracts/protocol.md`).
- Rate limits apply even if the client throttles itself.
