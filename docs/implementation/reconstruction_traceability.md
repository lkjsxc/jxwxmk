# Reconstruction Traceability Matrix

This document maps acceptance criteria to documentation sources and implementation locations.

## Section H) Frontend (Canvas renderer + input + UI)

| Acceptance Item | Doc Source | Implementation | Test |
|-----------------|------------|----------------|------|
| `src/client/` TypeScript sources | `docs/technical/frontend/build.md` | `src/client/` | Build via Docker |
| Build via esbuild to `src/static/game.js` | `docs/technical/frontend/build.md` | `src/client/package.json` | `npm run build` in Docker |
| Client connects to `/ws`, handles `welcome` | `docs/technical/frontend/runtime.md` | `src/client/connection.ts` | Integration test |
| Client performs spawn flow | `docs/technical/frontend/runtime.md` | `src/client/index.ts` | Manual test |
| Client handles `playerUpdate` for HUD/hotbar/inventory | `docs/technical/frontend/ui/hud.md` | `src/client/ui/` | Visual verification |
| Chunk cache maintenance | `docs/technical/frontend/runtime.md` | `src/client/world.ts` | Unit test |
| `chunkAdd`/`chunkRemove`/`entityDelta` handling | `docs/technical/backend/server/protocol.md` | `src/client/world.ts` | Integration test |
| Canvas render loop (camera + entities) | `docs/technical/frontend/rendering/` | `src/client/renderer.ts` | Visual verification |
| UI: HUD (HP/hunger/temp) | `docs/technical/frontend/ui/hud.md` | `src/client/ui/hud.ts` | Visual verification |
| UI: Hotbar slot selection | `docs/technical/frontend/ui/hud.md` | `src/client/ui/hotbar.ts` | Unit test |
| UI: Inventory view (30 slots) | `docs/technical/frontend/ui/inventory.md` | `src/client/ui/inventory.ts` | Visual verification |
| UI: Crafting menu | `docs/technical/frontend/ui/crafting.md` | `src/client/ui/crafting.ts` | Visual verification |
| UI: Quests + Achievements | `docs/technical/frontend/ui/quests.md`, `achievements.md` | `src/client/ui/` | Visual verification |
| UI: Notifications/toasts | `docs/technical/frontend/ui/notifications.md` | `src/client/ui/notifications.ts` | Visual verification |
| Session revoked overlay | `docs/technical/frontend/ui/screens.md` | `src/client/ui/screens.ts` | Manual test |
| Input: Unified InputManager | `docs/technical/frontend/input/` | `src/client/input.ts` | Unit test |
| Input: Keyboard (WASD, 1-7, E) | `docs/technical/frontend/input/keyboard.md` | `src/client/input.ts` | Manual test |
| Input: Touch (joystick + gestures) | `docs/technical/frontend/input/touch.md` | `src/client/input.ts` | Manual test |
| Camera smooth follow + zoom | `docs/technical/frontend/rendering/camera.md` | `src/client/camera.ts` | Visual verification |

## Frontend Source Tree Mapping

```
src/client/
├── README.md                 # Directory documentation (required)
├── package.json              # npm manifest with esbuild
├── tsconfig.json             # TypeScript configuration
├── index.ts                  # Entry point
├── types.ts                  # Shared type definitions
├── connection.ts             # WebSocket connection manager
├── input.ts                  # InputManager (keyboard + touch)
├── camera.ts                 # Camera controller
├── renderer.ts               # Canvas2D renderer
├── world.ts                  # Chunk cache + entity management
└── ui/                       # UI components
    ├── README.md
    ├── manager.ts            # UIManager
    ├── hud.ts                # HUD bars (HP/hunger/temp)
    ├── hotbar.ts             # Hotbar (7 slots)
    ├── inventory.ts          # Inventory grid (30 slots)
    ├── crafting.ts           # Crafting menu
    ├── quests.ts             # Quest log
    ├── achievements.ts       # Achievements tab
    ├── notifications.ts      # Toasts
    ├── screens.ts            # Login/game over overlays
    └── profile.ts            # Profile page
```

## Protocol Message Coverage

| Message | Direction | Implementation | Test |
|---------|-----------|----------------|------|
| `input` | C→S | `src/client/input.ts` | Unit test |
| `spawn` | C→S | `src/client/index.ts` | Integration |
| `craft` | C→S | `src/client/ui/crafting.ts` | Integration |
| `slot` | C→S | `src/client/ui/hotbar.ts` | Unit test |
| `swapSlots` | C→S | `src/client/ui/inventory.ts` | Integration |
| `welcome` | S→C | `src/client/connection.ts` | Integration |
| `playerUpdate` | S→C | `src/client/index.ts` | Integration |
| `chunkAdd` | S→C | `src/client/world.ts` | Unit test |
| `chunkRemove` | S→C | `src/client/world.ts` | Unit test |
| `entityDelta` | S→C | `src/client/world.ts` | Unit test |
| `sessionRevoked` | S→C | `src/client/connection.ts` | Manual |
| `error` | S→C | `src/client/ui/notifications.ts` | Manual |
| `achievement` | S→C | `src/client/ui/notifications.ts` | Manual |
| `notification` | S→C | `src/client/ui/notifications.ts` | Manual |
| `questUpdate` | S→C | `src/client/ui/quests.ts` | Integration |
