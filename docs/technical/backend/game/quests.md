# Quest System

Quests are generated from templates and stored per player.

## Data Model

- `QuestState`: `NotStarted`, `InProgress`, `ReadyToTurnIn`, `Completed`
- `ObjectiveType`:
  - `Gather { item, count, current }`
  - `Kill { mob_type, count, current }`
  - `Craft { recipe, count, current }`
  - `Deliver { item, target_npc }`
  - `Explore { biome, distance }`
  - `Defend { event_id, waves }`

## Generation

- Templates are selected by settlement tier and biome.
- Difficulty scales with player level and nearby population.

## Progress Updates

- Gather/kill/craft events are emitted by systems.
- Objectives update only for active quests.

## Configuration

- Templates and reward tables load from `config/quests.json`.
