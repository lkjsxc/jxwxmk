# NPC Interaction Mechanics

NPC interaction is handled by the server and delivered to the client as a modal dialogue.

## Dialogue System

- Player presses Interact near an NPC.
- Server responds with a dialogue payload (`npcInteraction`) containing text + options.
- Selecting an option sends `npcAction` with the NPC id and option index.

## Current NPC Types

- **Elder**: quest giver (wood gatherer -> wolf hunter).
- **Merchant**: placeholder dialogue and trade inventory (empty).
- **Guard**: placeholder dialogue.

## Trading

- Trade messages exist in the protocol, but server-side trade logic is not implemented.

## Quest Integration

- Dialogue options can start quests or complete them.
- Quest progress is tracked server-side and pushed to the client.
