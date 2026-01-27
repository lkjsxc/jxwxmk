# 2026-01-27 Reconstruction Decisions

This log records reconciliation decisions made during reconstruction. These decisions are treated as canonical across documentation.

## 1) Resources Respawn

- Resources respawn after depletion using cooldown timers.
- Tuning lives in `config/spawning.json`; the rule shape is described in `docs/design/world/regeneration.md`.

## 2) Config Naming (Survival)

- Survival tuning uses `config/survival.json` keys under `survival.*`.
- Documentation must not reference `mechanics.*` keys.

## 3) Temperature + Biome Modifiers

- Biome modifiers are required.
- Apply:
  - `biome.temperature_modifier` (additive to the neutral temperature)
  - optional `biome.hunger_modifier` (multiplier on hunger decay; default `1.0`)
- Temperature converges toward:
  - `target = survival.neutral_temp + biome.temperature_modifier`
  - using `survival.temperature_converge_rate` (per second).

## 4) Inventory Stacking

- There are no stack limits; items of the same type stack indefinitely within a slot.
- Inventory operations validate slot bounds and non-negative counts only.

## 5) Recipes Match Server

- Client and design documentation mirror server-side recipes from `config/crafting.json`.
- Example: **Door** requires **30 Wood**.

## 6) Hotbar Slot Switching

- Hotbar selection supports number keys `1-7` and pointer/touch selection.
- Both paths send the same `slot` message to the server.

## 7) Persistence Scope (World + Players)

- Persist world state, not just players:
  - settlements + barrier cores
  - chunk deltas (structures, depleted nodes/cooldowns, event state)
  - NPC inventories / services that change over time
- Use periodic checkpoints; avoid per-tick writes.

## 8) Temperature (General)

- Keep temperature deterministic and minimal: biome modifier + convergence + threshold damage.
- Extend later with day/night, weather, and heat sources as additive modifiers (documented separately when introduced).
