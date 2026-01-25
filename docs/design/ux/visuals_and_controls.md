# Visual Feedback & Controls

## Damage Feedback
- **Animation**: "Hit Flash" + Scale Pulse.
- **Duration**: ~0.25 seconds.
- **Effect**: Entity scales up to 1.2x and back to 1.0x (or shrinks) quickly.

## HP Bars
- **Color**: Translucent Red (rgba(255, 0, 0, 0.5)).

## Interaction Consistency
- **Selection Logic**: The object highlighted/displayed in UI MUST match the object processed by the backend/input logic.
- **Priority**: Distance > Angle > Type?

## PC Controls Fixes
- **Mouse Click**: Should ONLY trigger "Attack/Use" (A button equivalent) if clicked in game world, not UI.
- **B Button**: Should trigger "Interact", NOT "Attack".
- **Hotbar**: Clicking hotbar slots should select the slot, not attack.
