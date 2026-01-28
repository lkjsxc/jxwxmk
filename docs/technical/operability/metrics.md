# Metrics

Metrics provide the feedback loop for performance budgets and abuse controls.

## Principles

- Metrics must be cheap (tick-safe).
- Metrics must be bounded (no high-cardinality labels like raw UUIDs).

## Recommended metrics

Metric naming convention:

- Prefix all metrics with `jxwxmk_`.
- Use base units in names (seconds, bytes).
- Avoid high-cardinality labels (never label by UUID).

### Tick + simulation (required)

- `jxwxmk_tick_duration_seconds` (histogram)
- `jxwxmk_tick_overrun_total` (counter)
- `jxwxmk_active_players` (gauge)
- `jxwxmk_active_chunks` (gauge)

### Queues + backpressure (required)

- `jxwxmk_engine_input_queue_len` (gauge)
- `jxwxmk_engine_input_dropped_total` (counter)
- `jxwxmk_ws_messages_dropped_total` (counter)

### Persistence (required)

- `jxwxmk_checkpoint_duration_seconds` (histogram)
- `jxwxmk_checkpoint_failures_total` (counter)

### Network (recommended)

- `jxwxmk_ws_bytes_sent_per_tick` (histogram)
- `jxwxmk_ws_messages_sent_per_tick` (histogram)

## Export strategy

Baseline requirement: expose `GET /metrics` as a plaintext endpoint (Prometheus format) categorized as an “admin” HTTP endpoint.

If metrics export is disabled for any reason, the server must emit periodic log summaries so performance regressions are still visible.
