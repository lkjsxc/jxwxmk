# 0006 — Player State Synchronization

## Context

The existing protocol handles world state via `entityDelta` and `chunkAdd`, but lacks a mechanism to send private player-only state (like the full inventory or active quests) to the specific session owner. 

While vitals (HP, Hunger, Temp) were added to `EntitySnapshot`, the inventory is too large/complex to bundle into every entity delta update for all nearby players.

## Doc references

- `docs/technical/backend/server/protocol.md` (Server -> Client messages)
- `docs/implementation/reconstruction_acceptance.md` (Checklist H: Inventory view)

## Options considered

1. **Add `inventory` to `EntitySnapshot`**:
   - Pros: Single unified entity update path.
   - Cons: Bandwidth waste (others don't need to know my inventory); sensitive data leakage.

2. **Add a dedicated `playerUpdate` message**:
   - Pros: Private to the session owner; can contain full inventory and quest lists.
   - Cons: New message type to manage.

## Decision (chosen)

Choose **Option 2**.

Implement a `playerUpdate` message sent by the server to the session owner whenever their private state (inventory, stats, quests) changes.

```json
{
  "type": "playerUpdate",
  "data": {
    "inventory": [],
    "active_slot": 0,
    "xp": 100,
    "level": 1,
    "stats": {},
    "quests": []
  }
}
```

## Impact

- Updates `protocol` crates (Rust and TS).
- Updates `GameEngine::broadcast_deltas` to include this private message.
- Enables the UI to render the Inventory and Quest logs.
