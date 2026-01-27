# Combat Mechanics

Combat is implemented as part of the attack action. The server is authoritative for all damage.

## Damage Calculation

`Damage = ToolDamage * (1 + damage_bonus + level_bonus)`

- Base damage and tool damage values are from `config.json`.
- Tool level adds `tool_level_dmg_bonus` per level above 1.

## Tools

- **Hand**: `tools.base_dmg`
- **Wood Pickaxe**: `tools.wood_pickaxe_dmg`
- **Stone Pickaxe**: `tools.stone_pickaxe_dmg`
- **Rock Multiplier**: `tools.rock_mult` applies only to rocks.

## Mob Behavior

- **Rabbit**: random wander, non-hostile.
- **Wolf/Bear**: chase nearest spawned player within `aggression_range` and deal damage within `attack_range`.
- Mob damage is applied every tick when in range (no separate cooldown).

## Player Interaction

- Players can damage other players within `game.interact_range` when attacking.
