# Barrier System

Barrier cores define safe zones and anchor settlements.

## BarrierCore

- `level` controls effective range.
- `integrity` tracks damage and repair state.
- Range formula: `base_range + (level - 1) * level_multiplier`.

### Canonical Scale

- Range is in **world units (wu)** (see: `../../../design/world/scale_and_chunks.md`).
- Target parameters (in `config/settlements.json`):
  - `base_range = 24.0`
  - `level_multiplier = 6.0`

## Generation Rules

- Barrier cores seed settlement generation.
- Core level determines settlement tier and NPC mix.

## Tick Behavior

- Hostile mobs are removed or pushed outside the safe zone.
- PvP is disabled inside the safe-zone radius.

## Protocol representation (client visibility)

Barrier cores must be visible to clients as world entities so the village anchor and safe zone are discoverable:

- Stream the barrier core as an `EntitySnapshot` inside chunk streaming:
  - `kind: "structure"`
  - `subtype: "barrier_core"`
  - `x`, `y`: core position (wu)
  - `range`: safe-zone radius (wu)
  - `name` (optional): settlement name for UI labels

## Configuration

- Parameters live in `config/settlements.json`.
