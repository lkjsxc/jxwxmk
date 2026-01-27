# Mobile Controls (Touch)

Mobile uses the same interaction rules as desktop, expressed through touch gestures.

## Layout

### Left Zone (Movement)
- **Type**: Dynamic floating joystick.
- **Constraints**: Inner circle distance is capped to remain within the base circle.

### World Interaction (Tap + Long-Press)
- **Tap**: Primary action (attack/gather) on the world.
- **Long-Press (~250-300ms)**: Interact with nearby objects (open/use/talk).
- **No A/B Buttons**: The right side is reserved for gestures, not action buttons.

### UI Overlay
- **Hotbar**: Centered bottom. Tap to select item.
- **Crafting Menu**: Toggle button top-right.
- **Chat**: Semi-transparent overlay top-left.

## Gestures

- **Pinch**: Zoom camera.
- **Tap**: Attack/gather in the world (ignored when UI is focused).
- **Long-Press**: Interact with nearby objects.
