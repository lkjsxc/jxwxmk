# Client

TypeScript client for the game. Built with esbuild, outputs to `src/static/game.js`.

## Structure

- `types/` - TypeScript type definitions for protocol messages
- `core/` - Core systems (network, state management)
- `render/` - Canvas2D rendering (camera, renderer)
- `input/` - Input handling (keyboard, touch, mouse, unified manager)
- `ui/` - UI components (HUD, inventory, menu, notifications)
- `index.ts` - Main entry point

## Build

```bash
cd src/client && npm run build
```

## Input

- **Keyboard**: WASD/Arrow keys for movement, 1-7 for hotbar, Space for attack, I/E for inventory
- **Touch**: Left side joystick for movement, right side tap for attack, long-press for interact
- **Mouse**: Wheel for zoom

## UI

- **HUD**: HP/hunger/temperature bars, hotbar (7 slots), active item name
- **Menu**: Full-screen overlay with Inventory, Crafting, Settings, Disconnect options
- **Inventory**: 30-slot grid, stats display
