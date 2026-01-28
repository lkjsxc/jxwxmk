# 08 — Frontend (TS client + Canvas + UI)

Goal: implement the “dumb client”: renderer + input only, matching the server protocol.

References:
- `docs/technical/frontend/build.md`
- `docs/technical/frontend/runtime.md`
- `docs/technical/frontend/input/README.md`
- `docs/technical/frontend/rendering/README.md`
- `docs/technical/frontend/ui/README.md`
- `docs/technical/backend/server/protocol.md` (message shapes)

## A) Build pipeline

- [ ] `npm run build` bundles `src/client/index.ts` to `src/static/game.js` using `esbuild`.
- [ ] Docker build invokes the build step in the Node stage.

## B) Connection + session flow

- [ ] Client stores the session token in `localStorage`.
- [ ] Client connects to `/ws?token=<token>` when a token exists.
- [ ] On `welcome`:
  - store token
  - send `spawn` if `spawned` is false
- [ ] On `sessionRevoked`:
  - clear token
  - show blocking overlay and return to login
- [ ] On `error`:
  - show a toast with the error message
  - optionally highlight the relevant UI surface based on `code`

## C) World replication

- [ ] Maintain local chunk cache keyed by chunk coord.
- [ ] Handle:
  - `chunkAdd` (insert chunk)
  - `chunkRemove` (evict chunk)
  - `entityDelta` (apply updates/removals)

## D) Input

- [ ] Implement `InputManager` (keyboard + touch unified).
- [ ] Every ~50ms, send `input` when movement/actions are active.
- [ ] Include `aim` world coordinates when `attack` or `interact` is true.
- [ ] Implement slot switching:
  - number keys / clicking hotbar sends `slot`
  - drag/drop (optional) uses `swapSlots`

## E) Rendering (Canvas2D)

- [ ] `requestAnimationFrame` loop draws world and UI.
- [ ] Camera follows player smoothly (per rendering docs).
- [ ] Draw entities with minimal visuals (shapes/sprites), using interpolation.

## F) UI (Canvas-rendered)

Minimum surfaces required (see acceptance criteria):

- [ ] HUD bars (HP/hunger/temp).
- [ ] Hotbar (7 slots) + active slot highlight.
- [ ] Inventory view.
- [ ] Crafting menu wired to `craft` messages.
- [ ] Quests and achievements surfaces.
- [ ] Notifications/toasts.
- [ ] Login/profile screen that can claim a player id and show/copy it.

## Done when

- [ ] A player can claim a session, connect, spawn, move, and see chunk/entity updates.
