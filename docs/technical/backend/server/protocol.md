# Protocol

Gameplay traffic is JSON over WebSocket with explicit message types.

## Client -> Server

All client messages are objects with `type` and `data`.

### input

```json
{ "type": "input", "data": { "dx": -1.0, "dy": 0.0, "attack": false, "interact": false } }
```

### spawn

```json
{ "type": "spawn", "data": { "settlement_id": "<uuid>" } }
```

### craft

```json
{ "type": "craft", "data": { "recipe": "IronPickaxe" } }
```

### trade

```json
{ "type": "trade", "data": { "npc_id": "<uuid>", "item": "SaltCrate", "count": 2, "buy": true } }
```

### npcAction

```json
{ "type": "npcAction", "data": { "npc_id": "<uuid>", "option": 1 } }
```

### acceptQuest

```json
{ "type": "acceptQuest", "data": { "quest_id": "caravan_guard" } }
```

## Server -> Client

### welcome

```json
{ "type": "welcome", "id": "<player_uuid>", "token": "<session_token>", "version": 2, "spawned": false }
```

### chunkAdd

```json
{ "type": "chunkAdd", "data": { "coord": [12, -4], "biome": "forest", "entities": { "resources": {}, "mobs": {}, "structures": {}, "npcs": {} } } }
```

### chunkRemove

```json
{ "type": "chunkRemove", "data": { "coord": [12, -4] } }
```

### entityDelta

```json
{ "type": "entityDelta", "data": { "chunk": [12, -4], "updates": [], "removes": [] } }
```

### questUpdate / achievement / notification / npcInteraction

These remain similar in shape but are scoped to the new protocol version.
