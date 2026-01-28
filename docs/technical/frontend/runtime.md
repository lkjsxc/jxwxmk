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
- `error`: show toast (and optionally highlight the relevant UI surface based on `code`).
- `npcInteraction`: open NPC modal.
- `questUpdate`: merge into player quest list.
- `sessionRevoked`: show blocking overlay and clear stored token.

## Input Loop

- Every ~50ms, sends an `input` message if movement or actions are active.
- Payload matches `docs/technical/backend/server/protocol.md`:
  - always includes `dx`, `dy`, `attack`, `interact`
  - includes `aim` (world coords) whenever `attack` or `interact` is true
- One-off messages are sent for crafting, trades, NPC actions, and quests.

## Render Loop

- Uses `requestAnimationFrame`.
- Interpolates entity positions within active chunks.
- Delegates UI and HUD rendering to `UIManager`.
