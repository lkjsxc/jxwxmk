# NPC Interaction Mechanics

Interaction with NPCs is a core part of the MMORPG experience, enabling progression and economy.

## Dialogue System
- Players trigger dialogue by interacting with an NPC (e.g., clicking or pressing 'E').
- The server sends a `DialogueState` packet containing text and possible options.
- Options can lead to:
  - More dialogue.
  - Quest acceptance/completion.
  - Opening the Trade menu.
  - Ending the conversation.

## Trading System
- Merchants have a specific inventory and price list.
- Players can sell items for gold or other resources.
- Players can buy items using gold or other resources.
- The server validates all trades to prevent cheating.

## Quest System
- Quests have states: `NotStarted`, `InProgress`, `ReadyToTurnIn`, `Completed`.
- Objectives can include:
  - Gathering items.
  - Killing mobs.
  - Reaching a location.
  - Talking to another NPC.
- Rewards include items, XP, or access to new areas.
