# Client Runtime

The client is initialized in `src/client/index.ts`.

## Connection

- Establishes WebSocket to `/ws` with stored token (localStorage).
- On `welcome`:
  - Stores token.
  - Auto-sends `{ "spawn": true }` if `spawned` is false.

## World Updates

- On `world`:
  - Stores current world as `prevWorld`.
  - Replaces `world` with new snapshot.
  - Records `lastUpdateAt` for interpolation.
  - If player is missing, switches to Game Over state.

## Other Messages

- `achievement`: show toast.
- `notification`: show toast.
- `npcInteraction`: open NPC modal.
- `questUpdate`: merge into player quest list.

## Input Loop

- Every 50ms, sends `InputState` if:
  - Movement, attack, interact, or pointer down is active.
- Separate one-off messages are sent for crafting, slot selection, name updates, NPC actions, and trades.

## Render Loop

- Uses `requestAnimationFrame`.
- Interpolates entity positions between `prevWorld` and `world`.
- Delegates UI and HUD rendering to `UIManager`.
