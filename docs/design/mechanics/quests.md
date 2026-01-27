# Quest System

Quests are generated per settlement and scale by player tier.

## Quest Structure

- `id`, `name`, `description`
- `state`: `NotStarted`, `InProgress`, `ReadyToTurnIn`, `Completed`
- `objectives`: ordered list with progress counters
- `rewards`: items, currency, reputation, XP

## Objective Types

- **Gather**: `item`, `count`, `current`
- **Kill**: `mob_type`, `count`, `current`
- **Craft**: `recipe`, `count`, `current`
- **Deliver**: `item`, `target_npc`
- **Explore**: `biome`, `poi`, `distance`
- **Escort**: `npc_id`, `route`
- **Defend**: `event_id`, `waves`

## Quest Sources

- Settlement boards (repeatable and daily quests).
- NPCs tied to roles (hunter, smith, trader, guard).
- World events and bosses.

## Variety Targets

- 10x increase in quest variety versus baseline.
- Each settlement tier has a pool of 30-60 quest templates.
- Biomes inject local variants and rewards.

## Sample Quest Lines

- **Starter**: Gather lumber -> craft tools -> defend a caravan.
- **Biome**: Hunt unique predators -> collect rare ore -> craft resistance gear.
- **Event**: Defend settlement -> track raid leader -> claim bounty.
