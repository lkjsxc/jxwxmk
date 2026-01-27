# Touch Input

Touch input is split into three zones:

- **Left side**: virtual joystick for movement.
- **Right side**: world interaction gestures (tap + long-press).
- **UI overlay**: consumes touches when menus are open.

## Joystick

- First touch on the left half activates the joystick.
- Movement vector is normalized to a max radius (50px).

## Gestures

- **Tap**: primary attack/gather action.
- **Long-press (~300ms)**: interact with nearby objects.
- Gestures are tracked by touch identifier to support multi-touch.
