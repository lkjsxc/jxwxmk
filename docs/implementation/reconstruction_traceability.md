# Reconstruction Traceability Matrix

This document maps acceptance criteria to documentation sources, implementation locations, and tests.

## Traceability Format

| Acceptance Item | Source Docs | Implementation | Tests |
|-----------------|-------------|----------------|-------|

---

## A) Repo + docs invariants

| Acceptance Item | Source Docs | Implementation | Tests |
|-----------------|-------------|----------------|-------|
| Root allowlist | `docs/policy/INSTRUCT.md` §1.1 | Root directory structure | Visual inspection |
| One README per directory | `docs/policy/INSTRUCT.md` §1.2 | All `src/`, `config/`, `docs/` dirs | `find` command validation |
| No placeholder markers | `docs/policy/INSTRUCT.md` §1.2.1 | `src/`, `docs/` content grep | `grep -r "TODO\|TBD\|stub"` |
| TOC reachability | `docs/policy/INSTRUCT.md` §1.2 | All leaf docs linked via README chain | Manual traversal |

## B) Runtime container

| Acceptance Item | Source Docs | Implementation | Tests |
|-----------------|-------------|----------------|-------|
| Multi-stage Docker build | `docs/technical/deployment/README.md` | `src/runtime/Dockerfile` | `docker build` command |
| Single container runtime | `docs/policy/INSTRUCT.md` §1.4 | Runtime stage in Dockerfile | Container inspection |
| PostgreSQL internal only | `docs/technical/deployment/README.md` | `entrypoint.sh` bind config | Port scan validation |
| Health endpoint | `docs/technical/backend/server/http_ws.md` | `net` crate HTTP handler | Integration test |

## C) Configuration

| Acceptance Item | Source Docs | Implementation | Tests |
|-----------------|-------------|----------------|-------|
| Config file set | `docs/technical/config/files.md` | `config/*.json` | File existence |
| Config loading | `docs/technical/config/README.md` | `config` crate loader | Unit tests |
| Schema validation | `docs/technical/config/schemas/` | `config` crate validation | Unit tests |
| Unknown field rejection | `docs/technical/contracts/config.md` | Serde `deny_unknown_fields` | Unit tests |

## D) Backend HTTP + WebSocket

| Acceptance Item | Source Docs | Implementation | Tests |
|-----------------|-------------|----------------|-------|
| HTTP routes | `docs/technical/backend/server/http_ws.md` | `net` crate HTTP handlers | Integration tests |
| Single-session enforcement | `docs/technical/security/session_model.md` | Session registry in `net` | Integration test |
| Security headers | `docs/technical/backend/server/http_ws.md` | Actix middleware | Integration test |
| Protocol messages | `docs/technical/backend/server/protocol.md` | `protocol` crate types | Unit tests |
| Strict validation | `docs/technical/contracts/protocol.md` | `protocol` crate validation | Unit tests |
| Error responses | `docs/technical/contracts/protocol.md` §Error model | `protocol` crate error types | Integration tests |
| Private state isolation | `docs/technical/contracts/protocol.md` §Public vs private | `game` crate broadcast logic | Unit tests |
| Input aim validation | `docs/technical/backend/server/protocol.md` | `systems` crate interaction | Unit tests |
| Static asset embedding | `docs/technical/backend/server/static_assets.md` | `assets` crate with `rust-embed` | Integration test |

## E) Game simulation

| Acceptance Item | Source Docs | Implementation | Tests |
|-----------------|-------------|----------------|-------|
| Fixed tick loop | `docs/technical/contracts/tick.md` | `game` crate tick loop | Metrics validation |
| Single-writer rule | `docs/technical/contracts/authority.md` | `game` crate ownership | Code review |
| Tick backpressure | `docs/technical/contracts/tick.md` §Backpressure | Bounded queues in `game` | Load test |
| Chunk streaming | `docs/technical/backend/game/world_state.md` | `world` + `game` crates | Integration test |
| Interest management | `docs/technical/backend/game/world_state.md` | `world` crate interest sets | Unit tests |
| Settlements | `docs/design/world/settlements.md` | `world` crate settlement gen | Unit tests |

## F) Gameplay systems

| Acceptance Item | Source Docs | Implementation | Tests |
|-----------------|-------------|----------------|-------|
| Survival system | `docs/technical/backend/game/systems_survival.md` | `systems` crate survival | Unit tests |
| Movement | `docs/technical/backend/game/systems_interaction.md` | `systems` crate movement | Unit tests |
| Interaction | `docs/technical/backend/game/systems_interaction.md` | `systems` crate interaction | Unit tests |
| Crafting | `docs/technical/backend/game/systems_crafting.md` | `systems` crate crafting | Unit tests |
| Spawning/AI | `docs/technical/backend/game/spawning_and_ai.md` | `systems` crate spawning | Unit tests |
| Barriers | `docs/technical/backend/game/barriers.md` | `systems` crate barriers | Unit tests |
| Death/respawn | `docs/technical/backend/game/death.md` | `systems` crate death | Unit tests |
| Achievements | `docs/technical/backend/game/achievements.md` | `systems` crate achievements | Unit tests |
| Quests | `docs/technical/backend/game/quests.md` | `systems` crate quests | Unit tests |

## G) Persistence

| Acceptance Item | Source Docs | Implementation | Tests |
|-----------------|-------------|----------------|-------|
| SQL migrations | `docs/technical/backend/database/schema.md` | `persistence` crate migrations | Integration test |
| Player state | `docs/technical/backend/persistence/README.md` | `persistence` crate player ops | Integration test |
| Settlement state | `docs/technical/backend/persistence/README.md` | `persistence` crate settlement ops | Integration test |
| Chunk deltas | `docs/technical/backend/persistence/README.md` | `persistence` crate chunk ops | Integration test |
| Checkpoint strategy | `docs/technical/contracts/persistence.md` | `game` crate checkpoint logic | Integration test |

## H) Frontend

| Acceptance Item | Source Docs | Implementation | Tests |
|-----------------|-------------|----------------|-------|
| TypeScript build | `docs/technical/frontend/build.md` | `src/client/` + esbuild | Build test |
| Connection flow | `docs/technical/frontend/runtime.md` | `src/client/` connection logic | Manual test |
| PlayerUpdate handling | `docs/technical/frontend/ui/README.md` | `src/client/` state store | Manual test |
| Chunk cache | `docs/technical/frontend/runtime.md` | `src/client/` chunk cache | Manual test |
| Canvas rendering | `docs/technical/frontend/rendering/README.md` | `src/client/` renderer | Manual test |
| UI surfaces | `docs/technical/frontend/ui/README.md` | `src/client/` UI components | Manual test |

## I) Tests

| Acceptance Item | Source Docs | Implementation | Tests |
|-----------------|-------------|----------------|-------|
| Unit tests | `docs/technical/testing/README.md` | `src/server/*/tests/` | `cargo test` |
| Integration tests | `docs/technical/testing/README.md` | `src/server/tests/` | Docker test runner |
| Docker-first | `docs/policy/INSTRUCT.md` §1.5 | All test commands use Docker | CI validation |

## J) Operability

| Acceptance Item | Source Docs | Implementation | Tests |
|-----------------|-------------|----------------|-------|
| Structured logs | `docs/technical/operability/logging.md` | `env_logger` + structured format | Log inspection |
| Metrics endpoint | `docs/technical/operability/metrics.md` | `net` crate metrics handler | Integration test |
| Lifecycle | `docs/technical/operability/lifecycle.md` | `game` + `net` crate lifecycle | Integration test |

## K) Modularity

| Acceptance Item | Source Docs | Implementation | Tests |
|-----------------|-------------|----------------|-------|
| Crate boundaries | `docs/technical/module_map.md` | `src/server/Cargo.toml` workspace | Build validation |
| Dependency rules | `docs/technical/module_map.md` §Dependency Rules | Crate `Cargo.toml` files | Build validation |
| Framework isolation | `docs/technical/module_map.md` §Boundary hygiene | `world`/`systems` crate imports | Code review |

---

## Doc-to-Code Mapping

### Protocol Types
- **Docs**: `docs/technical/backend/server/protocol.md`, `docs/technical/contracts/protocol.md`
- **Code**: `src/server/protocol/src/lib.rs` (message types, validation)

### Config
- **Docs**: `docs/technical/config/`, `docs/technical/contracts/config.md`
- **Code**: `src/server/config/src/lib.rs` (loader, validation)

### World State
- **Docs**: `docs/technical/backend/game/world_state.md`, `docs/design/world/`
- **Code**: `src/server/world/src/lib.rs` (World, Chunk, entities)

### Systems
- **Docs**: `docs/technical/backend/game/systems_*.md`
- **Code**: `src/server/systems/src/` (survival, combat, crafting, etc.)

### Game Engine
- **Docs**: `docs/technical/backend/game/engine.md`, `docs/technical/contracts/tick.md`
- **Code**: `src/server/game/src/lib.rs` (tick loop, event queues)

### Persistence
- **Docs**: `docs/technical/backend/persistence/`, `docs/technical/backend/database/`
- **Code**: `src/server/persistence/src/lib.rs` (sqlx, migrations)

### Network
- **Docs**: `docs/technical/backend/server/http_ws.md`, `docs/technical/security/`
- **Code**: `src/server/net/src/lib.rs` (HTTP, WebSocket, sessions)

### Assets
- **Docs**: `docs/technical/backend/server/static_assets.md`
- **Code**: `src/server/assets/src/lib.rs` (rust-embed, serving)

### Client
- **Docs**: `docs/technical/frontend/`
- **Code**: `src/client/src/` (TypeScript client)

### Runtime
- **Docs**: `docs/technical/deployment/`, `docs/technical/operability/lifecycle.md`
- **Code**: `src/runtime/` (Dockerfile, entrypoint)
