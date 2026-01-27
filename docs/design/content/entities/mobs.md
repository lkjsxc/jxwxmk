# Mobs

AI-controlled entities spawned at world start.

## Passive

### Rabbit
- **HP**: `balance.mobs.rabbit_health`
- **Behavior**: random wandering.
- **Drop**: Meat x2 on kill.

## Aggressive

### Wolf
- **HP**: `balance.mobs.wolf_health` (scaled by level)
- **Damage**: `balance.mobs.wolf_dmg` (scaled by level)
- **Behavior**: chases nearest spawned player within `aggression_range`.

### Bear
- **HP**: `balance.mobs.bear_health` (scaled by level)
- **Damage**: `balance.mobs.bear_dmg` (scaled by level)
- **Behavior**: chases nearest spawned player within `aggression_range`.

## Leveling

Mob level scales with distance from world center using `leveling.mob_level_factor`.
