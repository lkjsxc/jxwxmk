# Logging

Logging is a first-class contract: it is the primary tool for debugging a deterministic tick server in production.

## Baseline requirements

- Logs must be structured and consistent.
- Logs must include enough context to correlate:
  - player/session events
  - protocol errors
  - tick overruns
  - persistence failures

## Recommended fields

When applicable, include:

- `event` (stable event name)
- `player_id`
- `token_id` (or a hash/prefix; avoid leaking full token)
- `remote_ip`
- `route` (HTTP route or `ws`)
- `code` (for errors)
- `tick` (tick counter)
- `ms` (durations)

## Required event coverage

- Startup summary:
  - config files loaded + resolved values
  - migrations applied
  - tick rate
- Session lifecycle:
  - session claimed (token rotated)
  - WS connected/disconnected
  - session revoked
- Abuse signals:
  - rate limit exceeded
  - invalid message rejected
  - queue overflow / backpressure triggered
- Persistence:
  - checkpoint success/failure (counts + durations)
- Performance:
  - tick overrun (tick duration > `dt`)

## Event names (baseline contract)

Use stable `event` values so logs remain queryable across refactors:

- `startup_config_loaded`
- `startup_migrations_applied`
- `session_claimed`
- `ws_connected`
- `ws_disconnected`
- `session_revoked`
- `protocol_error`
- `rate_limited`
- `engine_queue_overflow`
- `tick_overrun`
- `checkpoint_ok`
- `checkpoint_failed`
