# Rebuild TODO

A structured checklist for recreating the full system from scratch.

## 1. Repository + Docs

- [ ] Enforce root allowlist (README, LICENSE, AGENTS, docker-compose, config/, docs/, src/, .github/).
- [ ] Create documentation TOCs for every directory.
- [ ] Add this TODO document and link it from `docs/README.md`.

## 2. Build + Runtime Container

- [ ] Create multi-stage Dockerfile:
  - [ ] Node stage: `esbuild` bundles `src/client/index.ts` to `src/static/game.js`.
  - [ ] Rust stage: compile server and embed `static/`.
  - [ ] Runtime stage: Debian slim + PostgreSQL 15 + server binary.
- [ ] Create entrypoint script:
  - [ ] Initialize Postgres data dir (if empty).
  - [ ] Start Postgres locally on `127.0.0.1`.
  - [ ] Ensure `kkmypk` database exists.
  - [ ] Launch server binary.
- [ ] Add `docker-compose.yml` for local runtime.

## 3. Configuration System

- [ ] Implement `AppConfig` struct and `config.json` loader.
- [ ] Define schema for:
  - [ ] `server` (port, tick_rate)
  - [ ] `game` (world dims, interact range, spawn radius)
  - [ ] `mechanics` (hunger/cold decay, heal/starve/freeze, cooldowns)
  - [ ] `spawning` (densities, unit area)
  - [ ] `leveling` (mob level factor, tool XP)
  - [ ] `barriers` (base range, level multiplier, spawn caps)
  - [ ] `balance` (player, mobs, tools, resources, structures)

## 4. Server + Networking

- [ ] Actix Web server with security headers and logging.
- [ ] HTTP endpoints:
  - [ ] `GET /health`.
  - [ ] `GET /` + `GET /{filename}` for embedded assets.
- [ ] WebSocket endpoint `GET /ws` with optional token.
- [ ] Implement `GameSession` actor:
  - [ ] Parse client JSON input fields.
  - [ ] Send `Join` on start and `Leave` on stop.
  - [ ] Serialize server messages to JSON.

## 5. Protocol

- [ ] Define JSON payloads for:
  - [ ] `welcome`
  - [ ] `world`
  - [ ] `achievement`
  - [ ] `notification`
  - [ ] `npcInteraction`
  - [ ] `questUpdate`
- [ ] Define client message fields:
  - [ ] `dx`, `dy`, `attack`, `interact`
  - [ ] `spawn`, `craft`, `slot`, `name`, `swapSlots`
  - [ ] `npcAction`, `trade`, `acceptQuest`

## 6. World State + Entities

- [ ] Implement `World` struct with maps for players, resources, mobs, structures, NPCs, barrier cores.
- [ ] Implement entity data models:
  - [ ] Player + Inventory + Stats
  - [ ] Item + ItemType
  - [ ] Resource + ResourceType
  - [ ] Mob + MobType
  - [ ] Structure + StructureType
  - [ ] NPC + NpcType
  - [ ] BarrierCore

## 7. Game Engine (Tick)

- [ ] Implement `GameEngine` actor that owns `World`.
- [ ] Start fixed tick loop at `tick_rate`.
- [ ] Spawn initial entities on start:
  - [ ] Resources by density.
  - [ ] Mobs by density + level scaling.
  - [ ] Barrier cores (center + probabilistic extras).
  - [ ] Village NPCs near cores.
- [ ] Broadcast world snapshot each tick (filter unspawned players).

## 8. Systems

- [ ] Survival system (hunger, temperature, healing, starvation, freezing).
- [ ] Interaction system:
  - [ ] Movement with speed bonuses.
  - [ ] Attack action (consume food, place structures, gather, attack mobs, attack players).
  - [ ] Interact action (NPC proximity).
- [ ] Crafting system:
  - [ ] Recipe list and inventory consumption.
  - [ ] Server-authoritative crafting output.
- [ ] Achievement system:
  - [ ] Requirements + stat bonuses.
  - [ ] Unlock tracking + notifications.
- [ ] Quest system:
  - [ ] Quest states and objectives.
  - [ ] Gather/kill progress updates.
  - [ ] Elder dialogue for acceptance + completion.
- [ ] Barrier system:
  - [ ] Remove hostile mobs inside barrier range each tick.

## 9. Client Core

- [ ] WebSocket connection with localStorage token persistence.
- [ ] World snapshot handling and interpolation.
- [ ] Input loop sending `InputState` every 50ms.

## 10. Client Rendering

- [ ] Canvas renderer for map, entities, and HUD.
- [ ] Camera follow + zoom.
- [ ] Highlight closest target and show tooltips.
- [ ] Draw barrier range and mob levels.

## 11. Client UI

- [ ] Hotbar with slot selection.
- [ ] Inventory drag-and-drop + swap.
- [ ] Crafting menu and recipe details.
- [ ] Profile tab with name update.
- [ ] Quest log + pinning + tracker.
- [ ] Achievements list + detail + pinning.
- [ ] NPC dialogue modal.
- [ ] Game over screen + respawn flow.
- [ ] Toast notifications.

## 12. Persistence (Future)

- [ ] Add schema + migrations (accounts, player_state, world_state).
- [ ] Load/save player state by token.
- [ ] Periodic checkpoints for world structures.

## 13. Tests (Docker)

- [ ] Unit tests for crafting, survival, achievements, quests.
- [ ] Integration tests for WebSocket handshake and config loading.
- [ ] Smoke test for Docker runtime (server + Postgres up).
