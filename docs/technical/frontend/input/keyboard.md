# Keyboard/Mouse Input

## Event Mapping

- `keydown` / `keyup`:
    - Track state in `keysPressed` object.
    - WASD -> Vector2 Direction.
- `mousemove`:
    - Calculate rotation angle relative to screen center.
- `mousedown` / `mouseup`:
    - Set `attack` flag.

## Optimization
Only send packet if state changes (e.g., key press/release or significant mouse delta), but throttle to ~20-50ms to prevent network flooding.
