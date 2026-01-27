# Quest System

Quests are per-player tasks with explicit objectives and states.

## Quest Structure

- `id`: Unique identifier (string)
- `name`: Display name
- `description`: Player-facing text
- `state`: `NotStarted`, `InProgress`, `ReadyToTurnIn`, `Completed`
- `objectives`: List of objective types

## Objective Types

- **Gather**: `item`, `count`, `current`
- **Kill**: `mob_type`, `count`, `current`
- **TalkTo**: `npc_name`

## Current Quests

- **Wood Gatherer**: Gather 10 Wood for the Elder.
- **Wolf Hunter**: Kill 3 Wolves (unlocked after Wood Gatherer).

## Flow

1. Accept quest via Elder dialogue.
2. Progress updates via gather/kill events.
3. When complete, state becomes `ReadyToTurnIn`.
4. Turn in via Elder dialogue; gathering quests consume required items.
