# Client Architecture

The client is a "dumb" terminal that renders state and sends input.

## Rendering Loop

- `requestAnimationFrame` loop.
- Clears canvas.
- Interpolates entity positions between server snapshots for smoothness.
- Draws terrain, entities, and UI overlay.

## Input Handling

- Captures Keyboard (WASD) and Mouse (Click/Move).
- Sends input events to server via WebSocket immediately.
- **Client-Side Prediction**: Minimal initially to avoid complexity (rubber-banding is acceptable for prototype).

## Asset Management

- Procedural drawing (shapes) initially.
- Simple sprite loading later.
