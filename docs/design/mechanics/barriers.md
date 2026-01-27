# Barriers + Safe Zones

Barriers define protected zones and anchor settlements.

## Barrier Core

- A barrier core creates a safe-zone radius.
- Core level determines range and settlement tier cap.

## Scale (World Units)

- All barrier ranges are measured in **world units (wu)**. See: `../world/scale_and_chunks.md`
- Target configuration (in `config/settlements.json`):
  - `base_range = 24.0wu`
  - `level_multiplier = 6.0wu`
  - Effective range: `base_range + (level - 1) * level_multiplier`
    - L1: `24wu`, L3: `36wu`, L6: `54wu`, L10: `78wu`

## Safe-Zone Rules

- No PvP within the barrier radius.
- Hostile mobs are pushed out or despawned.
- Guard NPCs enforce rules and escort offenders out.

## Settlement Integration

- Each settlement is centered on a core.
- Respawn and trade services depend on core integrity.
- Damaged cores reduce safe-zone size until repaired.

## Configuration

- Core parameters live in `config/settlements.json`.
