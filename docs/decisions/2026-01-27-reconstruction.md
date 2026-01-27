# 2026-01-27 Reconstruction Decisions

## 1) Resource Respawn Rules

- **Context**: `docs/design/content/entities/resources.md` states resources do not respawn, while `docs/technical/backend/game/spawning_and_ai.md` + `docs/design/world/regeneration.md` describe cooldown-based respawn.
- **Options**:
  1. Disable respawn entirely (content doc).
  2. Implement cooldown-based respawn (technical + world docs).
- **Decision**: Implement cooldown-based respawn using `config/spawning.json`.
- **Impact**: `game/systems/spawning.rs` schedules respawns; depleted nodes return after `resource_respawn_seconds`.
- **Follow-up**: If the design intent is still “no respawn,” clarify which doc supersedes the technical backend docs.

## 2) Survival Config Key Names

- **Context**: `docs/design/mechanics/survival/hunger.md` uses `mechanics.*`, but `docs/technical/backend/game/systems_survival.md` uses `survival.*`, and only `config/survival.json` exists.
- **Options**:
  1. Add a `mechanics.json` and mirror fields.
  2. Use `survival.json` keys for implementation.
- **Decision**: Use `config/survival.json` fields (`hunger_decay`, `starve_damage`, etc.).
- **Impact**: Survival system loads `survival.json` and does not reference `mechanics.*`.
- **Follow-up**: If `mechanics.*` is intended, add a schema + file in `config/` and update technical docs.

## 3) Temperature Modifiers

- **Context**: `docs/technical/backend/game/systems_survival.md` mentions biome modifiers; `docs/design/mechanics/survival/temperature.md` says biome shifts are not implemented yet.
- **Options**:
  1. Ignore biome modifiers for now.
  2. Implement minimal biome modifiers from `config/biomes.json` only.
- **Decision**: Apply `temperature_modifier` and `hunger_modifier` from `config/biomes.json` with no day/night or weather logic.
- **Impact**: Survival tick reads the current chunk’s biome and nudges temperature/hunger accordingly.
- **Follow-up**: Confirm whether biome modifiers should be deferred to a later phase.

## 4) Inventory Stack Limits

- **Context**: `docs/design/ux/ui/inventory.md` removes stack limits; `docs/technical/backend/game/entities.md` includes `max_stack`.
- **Options**:
  1. Enforce `max_stack` values.
  2. Allow unlimited stacking and treat `max_stack` as optional metadata.
- **Decision**: Unlimited stacking; `max_stack` is optional and not enforced.
- **Impact**: Inventory operations only check slot bounds and total count.
- **Follow-up**: If stack caps are required later, introduce them in `config/balance.json`.

## 5) Door Recipe Mismatch (Client vs Server)

- **Context**: `docs/technical/frontend/ui/crafting.md` lists Door = 15 wood and calls out a mismatch; `docs/design/mechanics/crafting/recipes.md` + `config/crafting.json` require 30 wood.
- **Options**:
  1. Align client display to 30 wood.
  2. Preserve the documented mismatch.
- **Decision**: Preserve mismatch: server enforces 30 wood, client list shows 15 wood.
- **Impact**: Client recipe list remains inconsistent; server remains authoritative.
- **Follow-up**: Confirm if the mismatch should be resolved in future docs.

## 6) Hotbar Number Keys

- **Context**: `docs/design/ux/ui/inventory.md` says keys 1-7 select slots; `docs/technical/frontend/ui/hud.md` says number keys are not wired yet.
- **Options**:
  1. Implement number key selection.
  2. Keep only mouse/touch selection.
- **Decision**: Do not wire number keys yet; keep click/tap only.
- **Impact**: `InputManager` ignores number keys for slot selection; UI uses pointer events.
- **Follow-up**: If key selection is desired now, update technical frontend docs.

## 7) Persistence Scope

- **Context**: `docs/technical/backend/persistence/README.md` describes multiple tables/checkpoints; `docs/technical/backend/database/README.md` + `schema.md` define only `players` table.
- **Options**:
  1. Implement only `players` table now.
  2. Add additional tables and migrations immediately.
- **Decision**: Implement only `players` table with periodic save/load and token rotation.
- **Impact**: Persistence layer stores player state only; settlement/chunk state is in-memory.
- **Follow-up**: Clarify when additional tables should be added and how migrations will be managed.

## 8) Temperature Convergence Rate

- **Context**: `docs/technical/backend/game/systems_survival.md` says temperature moves toward `neutral_temp` but does not specify a rate; `docs/design/mechanics/survival/temperature.md` references `mechanics.cold_decay` which is not present in config.
- **Options**:
  1. Add a new config field for temperature convergence rate.
  2. Use a fixed small lerp factor per tick.
- **Decision**: Use a fixed 0.1 * `delta_seconds` lerp factor toward the target temperature.
- **Impact**: Temperature changes gradually without additional config fields.
- **Follow-up**: If a specific rate is desired, add it to `config/survival.json` and update the system.
