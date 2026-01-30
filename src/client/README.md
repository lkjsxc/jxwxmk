# Client

TypeScript client for the JXWXMK survival game. Built with esbuild and embedded into the Rust server binary.

## Build

```bash
npm install
npm run build    # Outputs ../static/game.js
npm run watch    # Development watch mode
```

## Architecture

The client is a "dumb renderer" - it maintains no authoritative state:

- **Connection**: WebSocket to `/ws` with session token
- **Input**: Unified keyboard + touch input manager
- **Rendering**: Canvas2D with interpolated entity movement
- **UI**: Canvas-rendered HUD, hotbar, inventory, crafting, quests

## Source Layout

- `index.ts` - Entry point, initializes all subsystems
- `types.ts` - Shared type definitions
- `connection.ts` - WebSocket connection manager
- `input.ts` - InputManager (keyboard + touch unified)
- `camera.ts` - Camera controller with smooth follow
- `renderer.ts` - Canvas2D world renderer
- `world.ts` - Chunk cache and entity management
- `ui/` - UI component modules
  - `manager.ts` - UIManager coordinating all UI
  - `hud.ts` - HP/hunger/temp bars
  - `hotbar.ts` - 7-slot hotbar
  - `inventory.ts` - 30-slot inventory grid
  - `crafting.ts` - Crafting menu
  - `quests.ts` - Quest log
  - `achievements.ts` - Achievements tab
  - `notifications.ts` - Toast notifications
  - `screens.ts` - Login/game over overlays
  - `profile.ts` - Profile page

## Protocol

See `docs/technical/backend/server/protocol.md` for message formats.
