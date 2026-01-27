# Interaction System

Handles player movement, attack actions, and NPC interactions.

## Movement

- Movement vector (`dx`, `dy`) is applied each input message.
- Speed = `balance.player.base_speed * (1 + stat_bonuses["speed"])`.
- Position is clamped within world bounds.
- Each movement input increments `stats.steps_taken`.

## Attack Action

Attack is a multi-purpose action with the following priority:

1. **Consume food**
   - If active slot holds `Berry`, `Meat`, or `CookedMeat`, consume 1 and add `mechanics.food_value` hunger.

2. **Place structure**
   - If active slot holds a structure item (`WoodWall`, `Door`, `Torch`, `Workbench`), place a structure at the player position and decrement the item.
   - `stats.structures_placed` increments.

3. **Gather resources**
   - Find the closest resource within `game.interact_range` and apply tool damage.
   - Rock resources use `tools.rock_mult`.
   - When resource is depleted, drop items:
     - Tree -> `Wood` x5
     - Rock -> `Stone` x3
     - Food -> `Berry` x2
   - `stats.resources_gathered` increments.

4. **Attack mobs**
   - Find the closest mob within `game.interact_range` and apply tool damage.
   - On kill, drop `Meat` x2 and increment `stats.mobs_killed`.

5. **Attack other players**
   - If no resource or mob is targeted, damage the closest other player in range.

### Tool Damage + XP

- Base damage starts at `tools.base_dmg`.
- Pickaxes override base damage:
  - `WoodPickaxe` -> `tools.wood_pickaxe_dmg`
  - `StonePickaxe` -> `tools.stone_pickaxe_dmg`
- Tool level adds `(level - 1) * tools.tool_level_dmg_bonus` damage.
- Tool XP gains:
  - `tool_xp_per_use` for gathering
  - `tool_xp_per_use * 2` for combat
- When XP >= `100 * level`, tool level increases and triggers a `LevelUp` event.

### Cooldown

- Attack is gated by `mechanics.attack_cooldown` (seconds), tracked by `Player.last_attack_at`.

## Interact Action

- Searches for the nearest NPC within `game.interact_range`.
- If found, returns an `Npc` interaction event.
- NPC interaction drives dialogue, quest acceptance, and quest turn-in (see [Quest System](quests.md)).

## Notes

- `mechanics.interact_cooldown` is defined but not enforced server-side.
