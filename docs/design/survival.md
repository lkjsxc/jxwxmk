# Survival Mechanics

Survival is the primary pressure on the player. Current implementation focuses on **Hunger** and **Temperature**.

## Health (HP)

- Reaches 0: player dies and is unspawned.
- Regenerates slowly when hunger is above the heal threshold.
- Lost via starvation and freezing.

## Hunger

- Decays over time (per-second rate defined in `config.json`).
- At 0, the player takes starvation damage each tick.
- Eating food restores hunger by a fixed amount.

## Temperature (Cold)

- Moves toward a neutral temperature (configurable).
- If temperature reaches 0, the player takes freezing damage each tick.

## Not Implemented Yet

- Thirst
- Biome-driven temperature changes
- Day/night temperature cycles
