# Protocol

All gameplay traffic is JSON over WebSocket. There is no explicit protocol version or sequence number in the current implementation.

## Client -> Server Messages

Client messages are a single JSON object with optional fields. Absent fields default to safe values on the server.

### Input + Actions

```json
{
  "dx": -1.0,
  "dy": 0.0,
  "attack": false,
  "interact": false,
  "craft": "WoodPickaxe",
  "slot": 0,
  "name": "PlayerName",
  "swapSlots": [0, 5],
  "spawn": true,
  "npcAction": ["<npc_uuid>", 0],
  "trade": ["<npc_uuid>", 2, true],
  "acceptQuest": "wood_gatherer"
}
```

Field details:

- `dx`, `dy` (f64): movement vector (-1..1).
- `attack` (bool): primary action (attack, gather, eat, place structure).
- `interact` (bool): secondary action (NPC interaction).
- `craft` (string): `ItemType` enum name to craft.
- `slot` (number): active hotbar slot (0-6 enforced server-side).
- `name` (string): requested display name (trimmed, max 12).
- `swapSlots` ([from, to]): swap inventory slots.
- `spawn` (bool): request spawn.
- `npcAction` ([npc_id, option_index]): click NPC dialogue option.
- `trade` ([npc_id, item_index, buy]): placeholder; not implemented server-side.
- `acceptQuest` (string): quest ID (optional path; alternate quest acceptance flow).

## Server -> Client Messages

All server messages wrap a `type` field and a `data` payload (except `welcome`).

### welcome

```json
{
  "type": "welcome",
  "id": "<player_uuid>",
  "token": "<session_token>",
  "spawned": false
}
```

### world

```json
{
  "type": "world",
  "data": { "width": 4000.0, "height": 4000.0, "players": { }, "resources": { }, "mobs": { }, "structures": { }, "npcs": { }, "barrier_cores": { } }
}
```

See [World State](../game/world_state.md) for entity payloads.

### achievement

```json
{
  "type": "achievement",
  "data": { "id": "NoviceWalker", "name": "Novice Walker", "description": "Walk 1,000 steps", "stat_bonus": ["speed", 0.01], "requirement": { "type": "Steps", "value": 1000 } }
}
```

### notification

```json
{
  "type": "notification",
  "data": { "title": "Quest Completed!", "message": "Wood Gatherer", "color": "#0f0" }
}
```

### npcInteraction

```json
{
  "type": "npcInteraction",
  "data": {
    "npc_id": "<npc_uuid>",
    "npc_type": "Elder",
    "name": "Elder",
    "text": "Greetings, traveler.",
    "options": ["Who are you?", "I need a quest.", "Goodbye"],
    "trade_items": []
  }
}
```

### questUpdate

```json
{
  "type": "questUpdate",
  "data": {
    "id": "wood_gatherer",
    "name": "Wood Gatherer",
    "description": "Collect 10 pieces of wood for the Elder.",
    "state": "InProgress",
    "objectives": [ { "Gather": { "item": "Wood", "count": 10, "current": 3 } } ]
  }
}
```
