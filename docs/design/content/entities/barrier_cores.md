# Barrier Cores

Barrier cores are settlement anchors and safe-zone generators.

## Properties

- `id`, `x`, `y`, `level`, `faction`
- `base_range`, `level_multiplier`

## Range

`range = base_range + (level - 1) * level_multiplier`

## Settlement Coupling

- Each core spawns a settlement around it.
- Core level controls settlement tier and NPC variety.
- Cores can be repaired or upgraded with rare materials.
