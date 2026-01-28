# 0005 — Operability baseline: `/metrics` + structured logs

## Context

A tick-based authoritative server needs visibility into:

- tick overruns
- queue overflow/backpressure
- persistence checkpoint failures
- abuse signals

Without metrics and structured logs, performance regressions and correctness bugs are hard to diagnose.

## Doc references

- `docs/technical/operability/README.md`
- `docs/technical/operability/logging.md`
- `docs/technical/operability/metrics.md`
- `docs/technical/backend/server/http_ws.md` (adds `/metrics`)

## Options considered

1. Rely on ad-hoc logs only.
   - Pros: minimal dependencies.
   - Cons: low signal, hard to alert on, easy to miss regressions.

2. Add a small, bounded metrics surface and enforce structured logs.
   - Pros: modern operability baseline; debuggable; supports budgets.
   - Cons: adds work and requires discipline about label cardinality.

## Decision (chosen)

Choose option 2:

- Add `GET /metrics` exporting Prometheus text.
- Require structured logs with stable event names and key context fields.
- Treat tick/queue/persistence metrics as first-class acceptance criteria.

## Impact

- Backend adds a new admin endpoint (`/metrics`).
- Tests must validate `/metrics` output is parsable.
- Logging fields become part of the implementation contract.

## Follow-ups

- If metrics dependencies become too heavy, keep the endpoint but simplify the exporter; logs are not a substitute for metrics.
