# 06 — Gameplay Systems (survival/crafting/etc.)

Goal: implement server-authoritative gameplay systems invoked by the tick loop.

References (technical):
- `docs/technical/backend/game/systems_survival.md`
- `docs/technical/backend/game/systems_interaction.md`
- `docs/technical/backend/game/systems_crafting.md`
- `docs/technical/backend/game/spawning_and_ai.md`
- `docs/technical/backend/game/barriers.md`
- `docs/technical/backend/game/death.md`
- `docs/technical/backend/game/achievements.md`
- `docs/technical/backend/game/quests.md`

References (design anchors):
- `docs/design/world/scale_and_chunks.md` (wu, placement grid)
- `docs/design/mechanics/crafting/recipes.md` (seed recipes)
- `docs/design/mechanics/building/placement.md` (placement model)

## A) Survival system

- [x] Hunger decay and thresholds per `systems_survival.md` (tick-rate scaled).
- [x] Temperature convergence per `systems_survival.md` (lerp toward biome target).
- [x] Optional thirst when `survival.thirst_enabled` is true.
- [x] Clamp vitals each tick.
- [x] Emit notifications for meaningful state changes (e.g., "hungry", "freezing") using `notification`.

## B) Interaction system

Movement:
- [x] Apply movement per tick using `balance.player.base_speed` and stat bonuses.
- [x] Update step stats for achievements.

Targeting:
- [x] Validate and use `input.data.aim` for target selection (see `docs/technical/backend/server/protocol.md` and `docs/technical/backend/game/systems_interaction.md`).
- [x] Enforce max interaction range (in `wu`) and reject invalid `aim`.

Actions (priority order):
- [x] Consume:
  - validate consumable in active slot
  - consume inventory count
  - apply effects (hunger restore and/or health restore)
  - clamp vitals
  - server-side cooldown/rate limit
- [x] Place:
  - validate structure item in active slot
  - snap to `2.5wu` grid
  - validate collision (no overlap with player or other structures)
  - consume inventory
  - create structure entity; mark chunk dirty
- [x] Gather:
  - choose nearest resource node to `aim` within range
  - apply tool damage with scaling
  - on depletion: drop items and start respawn cooldown
- [x] Attack:
  - choose target (mob/player) near `aim`
  - apply combat resolution (see combat tasks below)

Inventory/slots:
- [x] `slot` changes active slot server-side.
- [x] `swapSlots` swaps inventory slots server-side.

## C) Crafting system

- [x] Load recipes from `config/crafting.json` and any needed balance tables.
- [x] Validate recipe id and requirements.
- [x] Consume ingredients from inventory and add output item.
- [x] Update crafting stats and XP.
- [x] Station gating is currently a no-op unless promoted by config/docs.

## D) Spawning + AI

- [x] Chunk-local spawn budgets for resources and mobs.
- [x] Respawn timers stored per node/mob type.
- [x] Deterministic seeding for baseline spawns; persist deltas for depletion/placements.
- [x] Implement minimal AI loops:
  - passive wander
  - predator aggro + leash

## E) Barrier safe zones

- [x] Safe-zone radius formula per `docs/technical/backend/game/barriers.md`.
- [x] Enforce rules:
  - no PvP inside radius
  - hostile mobs removed/pushed outside

## F) Death + respawn

- [x] Detect player death (`health <= 0`) in tick loop.
- [x] Mark unspawned and apply drop rules.
- [x] Respawn at bound settlement core with vitals reset and cooldowns enforced.

## G) Achievements system

- [x] Load achievements from `config/achievements.json`.
- [x] Evaluate at end of tick for players with changed stats.
- [x] Grant XP and bonuses; persist; send `achievement` message.

## H) Quest system

- [x] Load quest templates from `config/quests.json`.
- [x] Handle `acceptQuest` and create per-player active quest state.
- [x] Update objectives based on emitted events (gather/kill/craft/etc.).
- [x] Send `questUpdate` messages with the documented shape.

## Done when

- [x] All systems run inside the tick loop without network code inside systems.
- [x] Each system has deterministic unit tests (see `docs/implementation/todo/09-tests/README.md`).
