# Visual Feedback & Controls

## Damage Feedback
- **Animation**: "Hit Flash" + Scale Pulse.
- **Duration**: ~0.25 seconds.
- **Effect**: All damageable objects (players, mobs, resources, structures, props) scale up to ~1.15x and return to 1.0x quickly.

## HP Bars
- **Color**: Translucent Red (rgba(255, 0, 0, 0.5)).

## Interaction Consistency
- **Selection Logic**: The object highlighted/displayed in UI MUST match the object processed by the backend/input logic.
- **Priority**: Distance > Angle > Type?

## Pointer Interaction Rules
- **Tap/Click**: Triggers primary action only when the world is clicked (never UI).
- **Long-Press/Click-Hold**: Triggers interact when in range.
- **Hotbar**: Clicking slots selects the slot only; it never triggers an action.
