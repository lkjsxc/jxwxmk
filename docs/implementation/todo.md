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

- [ ] Implement config loader for `config/` directory.
- [ ] Define schema for:
  - [ ] `server.json` (port, tick_rate, rate limits)
  - [ ] `world.json` (chunk size, streaming radii, seed)
  - [ ] `balance.json` (player, mobs, tools, resources, structures)
  - [ ] `survival.json` (hunger/temp/thirst rates)
  - [ ] `crafting.json` (recipes, station tiers)
  - [ ] `spawning.json` (budgets, respawn timers)
  - [ ] `biomes.json` (biome modifiers)
  - [ ] `settlements.json` (barrier cores, tiers)
  - [ ] `economy.json` (vendor pricing, taxes)
  - [ ] `quests.json` (templates and rewards)
  - [ ] `achievements.json` (definitions and bonuses)

## 4. Server + Networking

- [ ] Actix Web server with security headers and logging.
- [ ] HTTP endpoints:
  - [ ] `GET /health`.
  - [ ] `GET /` + `GET /{filename}` for embedded assets.
- [ ] WebSocket endpoint `GET /ws` with optional token.
- [ ] Implement `GameSession` actor:
  - [ ] Parse client JSON messages.
  - [ ] Send `Join` on start and `Leave` on stop.
  - [ ] Serialize server messages to JSON.

## 5. Protocol

- [ ] Define JSON payloads for:
  - [ ] `welcome`
  - [ ] `chunkAdd` / `chunkRemove`
  - [ ] `entityDelta`
  - [ ] `achievement`
  - [ ] `notification`
  - [ ] `npcInteraction`
  - [ ] `questUpdate`
- [ ] Define client message types:
  - [ ] `input`, `craft`, `trade`, `npcAction`, `acceptQuest`.

## 6. World State + Entities

- [ ] Implement `World` struct with chunk map and interest sets.
- [ ] Implement entity data models:
  - [ ] Player + Inventory + Stats + Reputation
  - [ ] Item + ItemType
  - [ ] Resource Node + ResourceType (leveled)
  - [ ] Mob + MobType (leveled)
  - [ ] Structure + StructureType (tiered)
  - [ ] NPC + Role
  - [ ] BarrierCore

## 7. Game Engine (Tick)

- [ ] Implement `GameEngine` actor that owns `World`.
- [ ] Start fixed tick loop at `tick_rate`.
- [ ] Activate/deactivate chunks based on player positions.
- [ ] Spawn initial settlements and starter chunks.
- [ ] Broadcast chunk deltas each tick.

## 8. Systems

- [ ] Survival system (hunger, temperature, thirst, healing).
- [ ] Interaction system (movement, actions, NPCs).
- [ ] Crafting system (stations + recipes).
- [ ] Achievement system (requirements + bonuses).
- [ ] Quest system (templates + progression).
- [ ] Barrier system (safe zones).

## 9. Client Core

- [ ] WebSocket connection with localStorage token persistence.
- [ ] Chunk add/remove and delta handling.
- [ ] Input loop sending `input` at fixed cadence.

## 10. Client Rendering

- [ ] Canvas renderer for chunks, entities, and HUD.
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

- [ ] Add schema + migrations (accounts, player_state, chunk_state).
- [ ] Load/save player state by token.
- [ ] Periodic checkpoints for chunk deltas and settlement state.

## 13. Tests (Docker)

- [ ] Unit tests for crafting, survival, achievements, quests.
- [ ] Integration tests for WebSocket handshake and config loading.
- [ ] Smoke test for Docker runtime (server + Postgres up).
