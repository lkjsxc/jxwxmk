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

