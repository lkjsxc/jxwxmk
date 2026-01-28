# Unified Input

`InputManager` collects keyboard, mouse, and touch signals into a single `InputState` payload.

## Logical Actions

- **Move**: `dx`, `dy` from WASD or virtual joystick.
- **Attack**: primary action (tap/click on world).
- **Interact**: long-press/hold on world or `E` key.

## Long-Press Detection

- Long-press is detected at ~250-300ms for both mouse and touch.
- The same threshold is used across devices to preserve parity.

## Cooldowns

- Attack: 500ms (`attackCooldown`).
- Interact: 400ms (`interactCooldown`).

These are client-side rate limits; the server still enforces its own cooldowns.

## Pointer Tracking

- `mouseX`, `mouseY` track the current pointer location.
- `isPointerDown` is true for mouse down or active touches.
- `pressStartMs` tracks when the current pointer press started for long-press detection.
- UI hit-testing uses these values to consume input.

## Protocol target (`aim`)

The client sends a world-space target point for authoritative action resolution:

- Convert the current pointer position to world coordinates using the camera transform.
- Include it as `aim: { x, y }` in the `input` message whenever `attack` or `interact` is true.
