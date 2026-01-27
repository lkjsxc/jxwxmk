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
{ "type": "spawn", "data": { "settlement_id": null } }
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

### slot

```json
{ "type": "slot", "data": { "slot": 0 } }
```

### swapSlots

```json
{ "type": "swapSlots", "data": { "from": 0, "to": 1 } }
```

### name

```json
{ "type": "name", "data": { "name": "NewName" } }
```

## Server -> Client

### welcome

```json
{ "type": "welcome", "id": "<player_uuid>", "token": "<session_token>", "version": 2, "spawned": false }
```

### sessionRevoked

```json
{ "type": "sessionRevoked", "reason": "login_elsewhere" }
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

### achievement

```json
{ "type": "achievement", "data": { "id": "first_steps", "name": "First Steps" } }
```

### notification

```json
{ "type": "notification", "data": { "text": "You feel hungry." } }
```

### npcInteraction

```json
{ "type": "npcInteraction", "data": { "npc_id": "<uuid>", "name": "Trader Lina", "text": "Need supplies?", "options": ["Browse", "Goodbye"] } }
```

### questUpdate

```json
{ "type": "questUpdate", "data": { "quest": { "id": "caravan_guard", "name": "Guard the Caravan", "state": "InProgress", "objectives": [] } } }
```

## Entity Shapes

### Entity Snapshot / Update

```json
{ "id": "e1", "kind": "resource", "subtype": "tree", "x": 12.5, "y": 9.0, "hp": 30.0, "max_hp": 30.0, "level": 2, "name": null, "range": null }
```

- `kind`: `player | resource | mob | structure | npc` (players arrive via `entityDelta`).
- `subtype`: resource/mob/structure/NPC type identifier.
- `hp`, `max_hp`, `level`, `name`, `range` are optional and omitted when irrelevant.

### Entity Removal

```json
{ "id": "e1", "kind": "resource" }
```
