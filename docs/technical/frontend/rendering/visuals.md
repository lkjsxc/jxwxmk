# Visuals

## Interaction Targeting

- The closest entity within 60px is outlined and shows a tooltip.
- Tooltips show context: Gather / Attack / Talk / Interact.

## Health + Damage Feedback

- Resources, mobs, structures, and players render simple health bars when damaged.
- Client tracks `lastHitAt` to animate a brief scale pulse.

## Barrier Cores

- Rendered as a glowing core with a dashed circular range indicator.
- Pulsing effect uses a sine wave to scale the core.

## Mob Levels

- Mob level is displayed above the mob (`Lv.X`).
- Max HP bar scales with level.
