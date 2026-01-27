# Entities

## Player

Fields:

- `id`, `token`, `username`
- `x`, `y`
- `health`, `hunger`, `cold`
- `inventory` (30 slots, hotbar is slots 0-6)
- `active_slot`
- `last_attack_at`, `last_interact_at`
- `stats` (steps, kills, crafts, gathers, structures, damage_taken, deaths)
- `achievements` (set of unlocked IDs as strings)
- `quests` (list of active quests)
- `stat_bonuses` (map of stat name -> additive multiplier)
- `spawned` (bool)

Notes:

- `stat_bonuses` currently affect movement speed, gather bonus, and damage.
- Bonuses for `max_hp` or `craft` are stored but not applied in logic.
- `last_interact_at` is stored but not used for cooldown enforcement.

## Inventory

- 30 slots, each slot is `Option<Item>`.
- Stacking uses `Item.max_stack` (currently `u32::MAX`), effectively unlimited stacking.
- `Inventory::add` attempts to stack first, then fills empty slots.

## Item

- `kind`: `ItemType` enum
- `amount`: stack count
- `max_stack`: max stack size (currently unlimited)
- `level`, `xp`: used for tools to provide scaling and leveling

ItemType values:

- `Wood`, `Stone`, `Gold`, `Diamond`, `Berry`, `Meat`, `CookedMeat`
- `WoodPickaxe`, `StonePickaxe`
- `WoodWall`, `Door`, `Torch`, `Workbench`

## Resource

- `r_type`: `Tree`, `Rock`, `Food`
- `amount`: remaining resource health

## Mob

- `m_type`: `Rabbit`, `Wolf`, `Bear`
- `health`, `level`
- `target_id`: reserved for future aggression tracking (not used yet)

## Structure

- `s_type`: `Wall`, `Door`, `Torch`, `Workbench`
- `health`, `owner_id`

## NPC

- `n_type`: `Elder`, `Merchant`, `Guard`
- `name`, `x`, `y`, `health`
- `dialogue_index`: reserved for future state
- `trade_inventory`: currently `Some(vec![])` for `Merchant`, `None` otherwise

## BarrierCore

- `x`, `y`, `level`, `base_range`
- Effective range: `base_range + (level - 1) * level_multiplier`
