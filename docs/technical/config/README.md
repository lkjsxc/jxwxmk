# Configuration

`config.json` is the authoritative balance and simulation configuration loaded by the server at startup.

## Location

- The server loads `config.json` from the working directory (`/app/config.json` in the runtime container).
- The file currently lives at the repository root to match the server loader.

## Schema (Current)

```json
{
  "server": {
    "port": 8080,
    "tick_rate": 20
  },
  "game": {
    "world_width": 4000.0,
    "world_height": 4000.0,
    "interact_range": 60.0,
    "spawn_radius": 500.0
  },
  "mechanics": {
    "hunger_decay": 0.05,
    "cold_decay": 0.02,
    "heal_rate": 0.1,
    "starve_dmg": 0.5,
    "freeze_dmg": 0.3,
    "food_value": 20.0,
    "attack_cooldown": 0.5,
    "interact_cooldown": 0.4
  },
  "spawning": {
    "resource_density": 0.2,
    "mob_density": 0.05,
    "unit_area": 10000.0
  },
  "leveling": {
    "mob_level_factor": 0.001,
    "tool_xp_per_use": 10.0
  },
  "barriers": {
    "base_range": 150.0,
    "level_multiplier": 50.0,
    "placement_chance_center": 0.1,
    "max_additional_barriers": 5
  },
  "balance": {
    "player": {
      "base_speed": 5.0,
      "max_health": 100.0,
      "max_hunger": 100.0,
      "neutral_temp": 50.0,
      "heal_threshold": 90.0
    },
    "mobs": {
      "rabbit_health": 10.0,
      "wolf_health": 50.0,
      "bear_health": 200.0,
      "wolf_dmg": 0.5,
      "bear_dmg": 1.5,
      "level_hp_mult": 0.2,
      "level_dmg_mult": 0.1,
      "aggression_range": 300.0,
      "attack_range": 30.0
    },
    "tools": {
      "base_dmg": 2.0,
      "wood_pickaxe_dmg": 4.0,
      "stone_pickaxe_dmg": 8.0,
      "rock_mult": 2.0,
      "tool_level_dmg_bonus": 0.1
    },
    "resources": {
      "tree_amount": 5,
      "rock_amount": 10,
      "food_amount": 1
    },
    "structures": {
      "wall_health": 200.0,
      "door_health": 100.0,
      "workbench_health": 50.0,
      "torch_health": 10.0
    }
  }
}
```

## Semantics

- **Tick-aware rates**: `hunger_decay`, `cold_decay`, `heal_rate`, `starve_dmg`, and `freeze_dmg` are specified per second and divided by `tick_rate` inside the survival system.
- **Cooldowns**: `attack_cooldown` and `interact_cooldown` are in seconds (server-side enforcement for attack only).
- **Spawn density**: `resource_density` and `mob_density` are scaled by `(world_area / unit_area)`.
- **Barrier ranges**: `base_range` is the level-1 radius; `level_multiplier` increases range per level.

## Usage

- The server loads `config.json` on startup via `AppConfig::load()`.
- The client does not read config directly; any visual ranges must be kept consistent in client code.
