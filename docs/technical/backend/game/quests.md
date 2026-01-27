# Quest System

Quests are stored per player and updated server-side based on gather/kill events and NPC dialogue.

## Data Model

- `QuestState`: `NotStarted`, `InProgress`, `ReadyToTurnIn`, `Completed`
- `ObjectiveType`:
  - `Gather { item, count, current }`
  - `Kill { mob_type, count, current }`
  - `TalkTo { npc_name }` (defined but not used in current quests)

## Initial Quests

`QuestSystem::get_initial_quests()` returns:

1. `wood_gatherer`
   - Gather 10 Wood
2. `wolf_hunter`
   - Kill 3 Wolves

## Progress Updates

- Gather and kill events are emitted from the Interaction System.
- Quest objectives are updated if the quest is `InProgress`.
- When all objectives are complete, state moves to `ReadyToTurnIn`.

## NPC Dialogue Integration

NPC dialogue (Elder) drives quest acceptance and completion.

- Selecting "I need a quest" starts `wood_gatherer` if not started.
- When `wood_gatherer` is ready, selecting the first option completes it and consumes required items.
- `wolf_hunter` becomes available after `wood_gatherer` is completed.

## Direct Acceptance Path

- The server also accepts an `acceptQuest` message with a quest id.
- This path transitions a quest from `NotStarted` to `InProgress`.
- The current client UI does not use this path.

## Network

Quest updates are sent to the client as `questUpdate` messages whenever state or progress changes.
