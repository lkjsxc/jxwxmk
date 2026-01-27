# Spawning + AI

## Initial World Spawn

### Resources

- Total count: `(world_area / unit_area) * resource_density`.
- Types: Tree (50%), Rock (40%), Food (10%) via random selection.
- Amount is configured per type (`balance.resources`).

### Mobs

- Total count: `(world_area / unit_area) * mob_density`.
- Types: Rabbit (60%), Wolf (30%), Bear (10%).
- Level scales by distance from world center:
  - `level = 1 + (distance * leveling.mob_level_factor)`.
  - Health scales by `level_hp_mult`.

### Barrier Cores + NPCs

- A center barrier core always spawns at `(world_width/2, world_height/2)`.
- Additional cores spawn by sampling random points up to `max_additional_barriers * 10` times.
  - Each sample uses a center-weighted probability:
    - `prob = (1 - dist/max_dist)^2 * placement_chance_center`.
  - When the sample passes, a core is placed and the count increases.
- NPCs spawn near barrier cores:
  - Center: Elder + Merchant.
  - Secondary cores: 50% chance of Merchant ("Villager").

## Mob AI

- Rabbits: random wandering each tick.
- Wolves/Bears:
  - Seek the nearest spawned player within `aggression_range`.
  - Move toward the target at a fixed step size (2.0 units) when outside melee range.

## Mob Damage

- Hostile mobs apply damage to players within `attack_range` each tick.
- Damage scales with mob type and level:
  - `wolf_dmg` / `bear_dmg` base values.
  - `level_dmg_mult` per level.
