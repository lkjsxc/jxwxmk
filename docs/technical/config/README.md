# Configuration System

The `config.json` file serves as the single source of truth for game balance.

## Schema

```json
{
  "server": {
    "port": 8080,
    "tick_rate": 20
  },
  "game": {
    "world_width": 10000,
    "world_height": 10000,
    "interact_range": 60.0,
    "spawn_radius": 200.0
  },
  "mechanics": {
    "hunger_decay": 0.1,
    "cold_decay": 0.05,
    "heal_rate": 1.0,
    "starve_dmg": 5.0,
    "freeze_dmg": 2.0,
    "attack_cooldown": 500,
    "interact_cooldown": 300
  },
  "items": {
    "food_value": 20.0,
    "wood_per_tree": 5,
    "stone_per_rock": 3
  }
}
```

## Usage
- **Consistency**: All systems (Engine, Survival, Crafting) must pull values from this config.
- **Sync**: Client-side visual ranges should match server-side logic values.