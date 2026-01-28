# Reconstruction Traceability Matrix

Mapping requirements to code and tests.

| Section | Doc References | Planned Module | Planned Tests |
|---|---|---|---|
| **A) Invariants** | `docs/policy/INSTRUCT.md` | `src/` (Structure verified) | N/A |
| **B) Runtime** | `docs/technical/deployment/README.md`<br>`docs/setup/docker.md` | `src/runtime/Dockerfile`<br>`src/runtime/entrypoint.sh` | `docker build` (Passed) |
| **C) Config** | `docs/technical/config/README.md`<br>`docs/technical/config/schemas/` | `src/crates/config/` | Unit tests for loaders |
| **D) Backend** | `docs/technical/backend/server/`<br>`docs/technical/contracts/protocol.md` | `src/bin/server/`<br>`src/crates/net/` | Integration (WS handshake, Auth) |
| **E) Engine** | `docs/technical/backend/game/engine.md`<br>`docs/technical/backend/game/world_state.md` | `src/crates/game/`<br>`src/crates/world/` | Unit (Tick loop, Chunk logic) |
| **F) Systems** | `docs/technical/backend/game/systems_*.md` | `src/crates/systems/` | Unit (Survival, Crafting logic) |
| **G) Persistence** | `docs/technical/backend/persistence/` | `src/crates/persistence/` | Integration (Migrations, Save/Load) |
| **H) Frontend** | `docs/technical/frontend/` | `src/client/`<br>`src/static/` | Build check (Passed), Manual verify |
| **I) Tests** | `docs/technical/testing/` | `src/crates/*/tests/` | Dockerized suite |
| **J) Operability** | `docs/technical/operability/` | `src/bin/server/` | `/metrics` check |
| **K) Modularity** | `docs/technical/module_map.md` | `src/Cargo.toml`<br>`src/crates/*/Cargo.toml` | Compile check (Passed) |
