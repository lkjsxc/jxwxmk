# Protocol Crate

WebSocket protocol message types and validation.

## Purpose

Defines all client→server and server→client message types with strict serde validation.

## Message Types

### Client → Server

- `input` - Movement and actions
- `spawn` - Enter the world
- `craft` - Craft an item
- `trade` - Trade with NPC
- `npcAction` - Interact with NPC
- `acceptQuest` - Accept a quest
- `slot` - Change active slot
- `swapSlots` - Swap inventory slots
- `name` - Change player name

### Server → Client

- `welcome` - Session established
- `sessionRevoked` - Session invalidated
- `playerUpdate` - Private player state
- `chunkAdd` - Chunk entered view
- `chunkRemove` - Chunk left view
- `entityDelta` - Entity updates
- `achievement` - Achievement unlocked
- `notification` - Toast message
- `error` - Structured error
- `npcInteraction` - NPC dialogue
- `questUpdate` - Quest state change

## Validation

All messages use `#[serde(deny_unknown_fields)]` to reject unexpected fields.
