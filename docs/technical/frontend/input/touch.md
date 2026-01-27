# Touch Input

Touch input is split into three zones:

- **Left side**: virtual joystick for movement.
- **Right side**: A (attack) and B (interact) buttons.
- **UI overlay**: consumes touches when menus are open.

## Joystick

- First touch on the left half activates the joystick.
- Movement vector is normalized to a max radius (50px).

## Buttons

- A button triggers attack.
- B button triggers interact.
- Buttons are tracked by touch identifier to support multi-touch.
