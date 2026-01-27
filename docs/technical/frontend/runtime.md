# Client Runtime

The client is initialized in `src/client/index.ts`.

## Connection

- Establishes WebSocket to `/ws` with stored token (localStorage).
- On `welcome`:
  - Stores token.
  - Sends `{ "type": "spawn", "data": { "settlement_id": null } }` if `spawned` is false.

## World Updates

- On `chunkAdd`:
  - Insert chunk into local cache.
- On `chunkRemove`:
  - Evict chunk from local cache.
- On `entityDelta`:
  - Apply updates/removals inside the target chunk.
- If the local player entity is missing for too long, switch to Game Over.

## Other Messages

- `achievement`: show toast.
- `notification`: show toast.
- `npcInteraction`: open NPC modal.
- `questUpdate`: merge into player quest list.

## Input Loop

- Every 50ms, sends `{ "type": "input" }` if movement or actions are active.
- One-off messages are sent for crafting, trades, NPC actions, and quests.

## Render Loop

- Uses `requestAnimationFrame`.
- Interpolates entity positions within active chunks.
- Delegates UI and HUD rendering to `UIManager`.
