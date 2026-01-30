# Protocol

Gameplay traffic is JSON over WebSocket with explicit message types.

## Client -> Server

All client messages are objects with `type` and `data`.

Identifier convention:

- All identifiers inside `data` (recipe IDs, item IDs, achievement IDs, quest IDs, entity subtypes) are `snake_case` strings unless explicitly noted.

### input

```json
{ "type": "input", "data": { "dx": -1.0, "dy": 0.0, "attack": false, "interact": false, "aim": { "x": 12.5, "y": 9.0 } } }
```

- `dx`, `dy`: movement vector components (clamp to `[-1.0, 1.0]`).
- `attack`: primary action (tap/click).
- `interact`: secondary action (long-press/hold).
- `aim`: world-space target point in **world units (wu)** (see: `../../../design/world/scale_and_chunks.md`).
  - Required when `attack` or `interact` is `true`.
  - Used for authoritative targeting (gather/attack/NPC interact) and structure placement.

Cadence:

- The client sends `input` every ~50ms while connected during gameplay.
- When idle, it still sends `input` with `dx=0, dy=0, attack=false, interact=false` (keepalive).

### spawn

```json
{ "type": "spawn", "data": { "settlement_id": null } }
```

### craft

```json
{ "type": "craft", "data": { "recipe": "wood_pickaxe" } }
```

### trade

```json
{ "type": "trade", "data": { "npc_id": "<uuid>", "item": "salt_crate", "count": 2, "buy": true } }
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
{ "type": "welcome", "id": "<player_uuid>", "token": "<session_token>", "version": 3, "spawned": false }
```

### sessionRevoked

```json
{ "type": "sessionRevoked", "reason": "login_elsewhere" }
```

### playerUpdate

Private player-only state update for the **session owner**.

This message is the authoritative source for:

- inventory + hotbar selection (`active_slot`)
- vitals (HP / hunger / temperature)
- profile/progression (level / XP / stats)
- quest list (initial state and changes)
- unlocked achievements list (initial state and changes)

Cadence:

- While `spawned` is true, the server sends `playerUpdate` at the server tick rate (so the client can drive camera follow and HUD updates authoritatively).

```json
{
  "type": "playerUpdate",
  "data": {
    "id": "<player_uuid>",
    "name": "NewName",
    "spawned": true,
    "x": 12.5,
    "y": 9.0,
    "vitals": { "hp": 30.0, "max_hp": 30.0, "hunger": 80.0, "max_hunger": 100.0, "temperature": 50.0, "max_temperature": 100.0 },
    "inventory": [null, { "item": "wood", "count": 12 }, null],
    "active_slot": 1,
    "level": 1,
    "xp": 100,
    "stats": { "steps": 123, "kills": 0, "crafts": 1, "gathers": 5, "deaths": 0 },
    "quests": [
      { "id": "caravan_guard", "name": "Guard the Caravan", "state": "InProgress", "objectives": [] }
    ],
    "achievements": ["first_steps"]
  }
}
```

- `x`, `y`: local player position in world units (wu).
  - This duplicates public world state for the session owner only and is used by the client to drive camera follow even before/without a local `entityDelta` snapshot.
- `inventory`: fixed-size array of length 30.
  - Each element is either `null` (empty slot) or `{ "item": "<snake_case_id>", "count": <int> }`.
- `active_slot`: hotbar selection index in `[0, 6]`.
  - The hotbar corresponds to inventory slots `0..=6`.

### chunkAdd

```json
{ "type": "chunkAdd", "data": { "coord": [12, -4], "biome": "forest", "entities": { "resources": [], "mobs": [], "structures": [], "npcs": [] } } }
```

`entities.*` are arrays of entity snapshots.

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

### error

Structured error message for rejected inputs or recoverable failures.

```json
{ "type": "error", "data": { "code": "invalid_message", "message": "Invalid message.", "details": null } }
```

- `code`: stable machine-readable identifier (`snake_case`).
- `message`: user-facing text suitable for a toast.
- `details`: optional extra info (may be null/omitted).

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
- Private player-only state (inventory, vitals, quests, achievements, etc.) is synchronized separately via `playerUpdate`.

### Entity Removal

```json
{ "id": "e1", "kind": "resource" }
```
