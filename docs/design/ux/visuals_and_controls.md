# Visual Feedback + Controls

## Damage Feedback

- **Animation**: Hit flash + scale pulse.
- **Duration**: ~0.25 seconds.
- **Effect**: All damageable objects (players, mobs, resources, structures, props) scale up to ~1.15x and return to 1.0x quickly.

## HP Bars

- **Color**: Translucent red (`rgba(255, 0, 0, 0.5)`).
- **Visibility**: Show on damage or when below 100%.

## Interaction Consistency

- **Selection Logic**: The object highlighted in UI must match the backend target.
- **Priority**: Distance > angle > type.

## Pointer Interaction Rules

- **Tap/Click**: Primary action only when the world is clicked (never UI).
- **Long-Press/Click-Hold**: Interact when in range.
- **Hotbar**: Selecting a slot never triggers a world action.
