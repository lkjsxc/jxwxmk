# Client Source (TypeScript)

Build-time only client code compiled to `../static/game.js` and served by the Rust server.

## Contents

- `index.ts`: Entry point (WebSocket + input + render loop).
- `net/`: WebSocket client and protocol handling.
- `input/`: Unified input manager (keyboard/mouse/touch).
- `rendering/`: Canvas renderer and camera.
- `ui/`: HUD, menus, and overlays.
- `state/`: Client-side cached world state.
- `data/`: Static client lists (achievements, recipes).
