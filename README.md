# JXWXMK

Authoritative multiplayer survival game with a single Rust server and embedded web client.

This repo is **docs-first**: the documentation is intended to be sufficient to recreate the full system from scratch.

## Start Here (docs)

- `docs/README.md` (documentation index)
- `docs/policy/INSTRUCT.md` (operating contract + hard invariants)
- `docs/tmp/src-recreate.md` (single-prompt full reconstruction template)
- `docs/implementation/reconstruction_acceptance.md` (definition of “done”)
- `docs/implementation/todo/README.md` (multi-file reconstruction backlog)

If you’re using a tool that supports repo instruction files (for example, `gemini-cli` reads `GEMINI.md`), prefer that; `GEMINI.md` is kept identical to `docs/tmp/src-recreate.md`.

## Architecture (at a glance)

- Single runtime container: **Rust server + PostgreSQL** (no separate runtime Node service).
- Fixed tick simulation (20–60Hz) with **single-writer world state**.
- WebSocket gameplay protocol (currently **v3**) with strict validation + structured errors.
- Frontend is build-time TypeScript (esbuild) embedded into the Rust binary via `rust-embed`.

## Build/run (after `src/` exists)

```bash
docker build -f src/runtime/Dockerfile -t jxwxmk .
```

```bash
docker run --rm \
  -p 8080:8080 \
  -v jxwxmk_pgdata:/var/lib/postgresql/data \
  -v ./config:/app/config \
  jxwxmk
```

Compose examples live under `src/runtime/compose/`.
