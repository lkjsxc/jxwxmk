# 0004 — Schema-first config + strict validation

## Context

Reconstruction and long-term maintenance are fragile when config files are named but not specified:

- agents invent fields and semantics
- typos silently change gameplay
- code and docs drift

To keep the system deterministic and agent-friendly, config must be schema-first and validated strictly.

## Doc references

- `docs/technical/config/README.md`
- `docs/technical/config/files.md`
- `docs/technical/config/schemas/README.md`
- `docs/technical/contracts/config.md`

## Options considered

1. Loose config parsing (ignore unknown fields).
   - Pros: forward-compatible.
   - Cons: silent typos and drift; dangerous for an authoritative server.

2. Strict config parsing (reject unknown fields; versioned schemas).
   - Pros: prevents drift; deterministic; easier reconstruction.
   - Cons: requires explicit schema updates for changes.

## Decision (chosen)

Choose option 2:

- Document schemas under `docs/technical/config/schemas/`.
- Require `version` in each config file.
- Reject unknown fields and invalid types.
- Validate numeric ranges and identifier formats (`snake_case`).

## Impact

- Adds a concrete spec surface for config values.
- Makes reconstruction more complete and reduces “agent guesswork”.

## Follow-ups

- If config evolution needs smoother transitions, add explicit migration tooling and schema version bumps per file.
