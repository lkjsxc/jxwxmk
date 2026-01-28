# Quest UI

## Quest Log

- Displays all quests from the player state.
- Each quest card shows:
  - Name
  - State (`NotStarted`, `InProgress`, `ReadyToTurnIn`, `Completed`)
  - Description
  - Objectives with progress

Data sources:

- Initial quest list comes from `playerUpdate.quests`.
- Incremental updates arrive via `questUpdate` (and/or a fresh `playerUpdate` snapshot).

## Pinning

- Each quest card has a Pin/Unpin button.
- The pinned quest is shown in the HUD tracker.

## NPC Interactions

- NPC dialogue options can trigger quest acceptance or completion.
- Quest updates arrive via `questUpdate` messages and replace the matching quest entry.
- Accepting a quest sends `{ "type": "acceptQuest", "data": { "quest_id": "<id>" } }`.
