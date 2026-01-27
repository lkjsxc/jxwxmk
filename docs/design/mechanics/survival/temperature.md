# Temperature Mechanics

## Target Temperature

- Each tick, the player's temperature moves toward `balance.player.neutral_temp` at a rate of `mechanics.cold_decay` per second (scaled per tick).

## Effects

- Temperature <= 0: apply `mechanics.freeze_dmg` per second (scaled per tick).

## Not Implemented Yet

- Biome-based temperature shifts
- Day/night cycles
- Heat sources (campfires)
