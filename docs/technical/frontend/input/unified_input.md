# Unified Input

`InputManager` collects keyboard, mouse, and touch signals into a single `InputState` payload.

## Logical Actions

- **Move**: `dx`, `dy` from WASD or virtual joystick.
- **Attack**: primary action (tap/click on world).
- **Interact**: long-press/hold on world or `E` key.

## Cooldowns

- Attack: 500ms (`attackCooldown`)
- Interact: 400ms (`interactCooldown`)

These are client-side rate limits; the server still enforces its own cooldowns.

## Pointer Tracking

- `mouseX`, `mouseY` track the current pointer location.
- `isPointerDown` is true for mouse down or active touches.
- `pressStartMs` tracks when the current pointer press started for long-press detection.
- UI hit-testing uses these values to consume input.
