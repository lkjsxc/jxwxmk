# Visuals

## Interaction Targeting

- The closest entity within 60px is outlined and shows a tooltip.
- Tooltips show context: Gather / Attack / Talk / Interact.

## Health + Damage Feedback

- All damageable objects (resources, mobs, structures, players, NPCs) render simple health bars when damaged.
- Client tracks `lastHitAt` to animate a 0.25s scale pulse (1.0 -> 1.15 -> 1.0).
- Use a quick ease-out to keep the hit readable without slowing combat.

## Barrier Cores

- Rendered as a glowing core with a dashed circular range indicator.
- Pulsing effect uses a sine wave to scale the core.

## Mob Levels

- Mob level is displayed above the mob (`Lv.X`).
- Max HP bar scales with level.
