# Reconstruction Traceability Matrix

This matrix maps `docs/implementation/reconstruction_acceptance.md` sections to:

- the documentation that specifies the requirement
- the intended `src/` ownership boundary
- the test surface that proves it

| Acceptance section | Spec docs | Implementation (planned `src/`) | Proof (tests / commands) |
|---|---|---|---|
| A) Repo + docs invariants | `docs/policy/INSTRUCT.md` | Repo structure + README topology | Placeholder sweep + TOC reachability sweep |
| B) Runtime container | `docs/technical/deployment/README.md`, `docs/setup/docker.md` | `src/runtime/` | `docker build -f src/runtime/Dockerfile -t jxwxmk .` + `GET /health` |
| C) Configuration | `docs/technical/config/*` | `config/` + `src/server/crates/config/` | Startup validation integration test |
| D) Backend HTTP + WS | `docs/technical/backend/server/*`, `docs/technical/contracts/protocol.md` | `src/server/crates/net/` + `src/server/crates/protocol/` | WS handshake + protocol validation integration tests |
| E) Game simulation | `docs/technical/backend/game/*`, `docs/technical/contracts/tick.md` | `src/server/crates/game/` + `src/server/crates/world/` | Deterministic unit tests + streaming smoke test |
| F) Gameplay systems | `docs/technical/backend/game/systems_*.md` | `src/server/crates/systems/` | Unit tests per system (crafting/survival/barriers/respawn) |
| G) Persistence | `docs/technical/backend/persistence/*`, `docs/technical/backend/database/*` | `src/server/crates/persistence/` + `src/runtime/migrations/` | Migrations apply + save/load integration tests |
| H) Frontend | `docs/technical/frontend/*` | `src/client/` → `src/static/game.js` | Docker build stage bundles client; runtime serves embedded assets |
| I) Tests (Dockerized) | `docs/technical/testing/*` | Tests under `src/server/` and runtime wiring under `src/runtime/compose/` (if used) | Single Docker/Compose command path exercises unit + integration layers |
| J) Operability | `docs/technical/operability/*` | `src/server/crates/net/` + `src/server/crates/game/` | `/metrics` parse test + structured logging checks |
| K) Modularity | `docs/technical/module_map.md`, `docs/technical/contracts/authority.md` | Crate boundaries under `src/server/crates/` | Compile-time forbidden-edge enforcement |
