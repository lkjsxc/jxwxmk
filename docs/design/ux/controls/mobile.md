# Mobile Controls (Touch)

Mobile uses the same interaction rules as desktop, expressed through touch gestures.

## Layout

### World Surface (Joystick + Gestures)
Mobile interprets touches dynamically so one-hand play is possible:

- **Tap** becomes the primary action (attack/gather) when the finger stays mostly in place.
- **Long-Press (~250-300ms)** becomes interact (open/use/talk) when the finger stays mostly in place.
- **Drag** becomes the movement joystick once the finger moves beyond a small threshold.

Joystick rules:

- **Type**: Dynamic floating joystick (base at touch start).
- **Constraints**: Inner circle distance is capped to remain within the base circle (50px radius).

World interaction rules:
- **Tap**: Primary action (attack/gather) on the world.
- **Long-Press (~250-300ms)**: Interact with nearby objects (open/use/talk).
- **No A/B Buttons**: World interaction is expressed through gestures, not dedicated action buttons.

### UI Overlay
- **Hotbar**: Centered bottom. Tap to select item.
- **Crafting Menu**: Toggle button top-right.
- **Chat**: Semi-transparent overlay top-left.

Safe-area note:

- Keep the hotbar above the OS gesture area (safe-area insets) so taps do not conflict with system navigation.

## Gestures

- **Pinch**: Zoom camera.
- **Tap**: Attack/gather in the world (ignored when UI is focused).
- **Long-Press**: Interact with nearby objects.
