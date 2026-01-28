# Server

Rust workspace containing the authoritative game server.

## Workspace Structure

```
server/
├── Cargo.toml          # Workspace manifest
├── src/main.rs         # Binary entrypoint
├── protocol/           # Message types and validation
├── config/             # Config loading and validation
├── world/              # World state, chunks, entities
├── systems/            # Gameplay systems (survival, combat, etc.)
├── game/               # Tick loop and engine orchestration
├── persistence/        # PostgreSQL persistence
├── net/                # HTTP + WebSocket handlers
└── assets/             # Static asset embedding
```

## Crate Dependencies

Per `docs/technical/module_map.md`:

- `protocol` - No internal deps (base types)
- `config` → `protocol`
- `world` → `config`
- `systems` → `world` + `config`
- `game` → `systems` + `world` + `config` + `protocol` + `persistence`
- `persistence` → `world` + `protocol` + `config`
- `net` → `protocol` + `game`
- `assets` - No internal deps

## Single-Writer Model

- Only `game` crate mutates world state
- `net` enqueues events; never mutates directly
- `persistence` provides load/save adapters

## Entrypoint

`src/main.rs` constructs the engine and starts Actix Web server.
