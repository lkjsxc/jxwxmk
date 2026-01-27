# Resources

Static entities harvestable by players.

For the full catalog (trees/stones/ores/plants by biome and tier), see `resources/README.md`.

## Types

### Tree
- **Amount**: `balance.resources.tree_amount`
- **Drops**: Wood x5 (on depletion)

### Rock
- **Amount**: `balance.resources.rock_amount`
- **Drops**: Stone x3 (on depletion)
- **Rock Multiplier**: Tool damage multiplied by `tools.rock_mult`

### Food
- **Amount**: `balance.resources.food_amount`
- **Drops**: Berry x2 (on depletion)

## Respawn

Resources respawn after being depleted.

- Depletion sets a per-node cooldown timer.
- When the cooldown expires, the node reappears (optionally with small position jitter within the chunk).
- Respawn timing and density caps are configured in `config/spawning.json`.
