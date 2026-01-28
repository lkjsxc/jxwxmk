# HUD + Hotbar

## HUD Bars

- Rendered in the top-left corner.
- Bars:
  - HP (red)
  - Hunger (orange)
  - Temperature (blue; inverted by `100 - cold`)
- Values come from `playerUpdate.vitals` (not from public `entityDelta` updates).

## Hotbar

- 7 slots rendered at the **bottom of the screen**, centered horizontally (viewport-anchored, not world-anchored).
- Active slot is highlighted in yellow.
- Clicking a slot sends `{ "type": "slot", "data": { "slot": <index> } }` to the server.
- Active item name is displayed above the hotbar.
- Hotbar items are read from `playerUpdate.inventory[0..=6]`.
- Hotbar selection is `playerUpdate.active_slot` (authoritative).

Layout notes:

- Always visible during gameplay.
- Keep a small bottom padding; on mobile, respect safe-area insets so the hotbar is not under the OS gesture area.

## Notes

- Number keys `1-7` switch the active hotbar slot.
- Pointer/touch selection on a slot also switches the active slot.
- Slot switching never triggers a world action; it only changes `active_slot`.
