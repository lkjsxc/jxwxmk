# Implementation TODOs (Reconstruction Backlog)

This directory tree is the **task list** for implementing the full system from scratch.

It is intentionally split across **multiple files and directories** so agents can work in focused slices without losing requirements.

## How to use this backlog

1. Read and obey `docs/policy/INSTRUCT.md`.
2. Read the scope boundary: `docs/implementation/reconstruction_scope.md`.
3. Use `docs/implementation/reconstruction_acceptance.md` as the definition of “done”.
4. Execute tasks in the order linked below (foundation → runtime → backend → game → frontend → tests).
5. If something is unclear or conflicts, record a decision in `docs/implementation/decision_log.md` and continue.

## Rules for items in this backlog

Every checkbox item should be:

- **Concrete** (what to implement, not “make X better”)
- **Doc-traceable** (point to the doc(s) that require it)
- **Locatable** (name the intended `src/` path(s))
- **Test-backed** where required (name the test(s) and the Docker command to run them)

## Sections (execute in order)

- [01 — Foundation (repo + `src/` skeleton)](01-foundation/README.md)
- [02 — Runtime (Docker + entrypoint + compose)](02-runtime/README.md)
- [03 — Configuration (`config/*.json` + loader)](03-config/README.md)
- [04 — Backend (HTTP/WS + protocol + assets)](04-backend/README.md)
- [05 — Game + World (tick loop + chunks + streaming)](05-game/README.md)
- [06 — Gameplay Systems (survival/crafting/etc.)](06-systems/README.md)
- [07 — Persistence (Postgres + sqlx + checkpointing)](07-persistence/README.md)
- [08 — Frontend (TS client + Canvas + UI)](08-frontend/README.md)
- [09 — Tests (Dockerized unit + integration)](09-tests/README.md)
- [10 — CI (GitHub Actions)](10-ci/README.md)

## When work remains unfinished

This backlog is the **only** place where “unfinished work” checklists are allowed.
Do not add TODO/TBD placeholders elsewhere in `docs/` or `src/`.
