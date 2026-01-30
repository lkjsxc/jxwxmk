# Touch Input

Touch input uses a dynamic “gesture vs joystick” interpretation so one-hand play is possible:

- A touch that stays mostly in place becomes a world gesture (tap / long-press).
- A touch that moves beyond a small threshold becomes the movement joystick.
- UI overlays always consume touches while open.

## Joystick

- If a touch moves more than `~12px` from its start point, treat it as joystick control.
- The joystick is “floating”: its base is the touch start point.
- The movement vector is normalized and capped to a max radius (`50px`).

## Gestures

- **Tap**: Primary attack/gather action.
- **Long-press (~250-300ms)**: Interact with nearby objects.
- **No A/B buttons**: Gestures replace on-screen action buttons.
- Gestures are tracked by touch identifier to support multi-touch.

Interaction and joystick are mutually exclusive for a given touch:

- If the touch becomes a joystick (moves past the activation threshold), it must not also trigger tap/long-press.
- If the touch stays within the threshold, it must not drive movement.
