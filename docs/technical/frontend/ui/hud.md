# HUD + Hotbar

## HUD Bars

- Rendered in the top-left corner.
- Bars:
  - HP (red)
  - Hunger (orange)
  - Temperature (blue; inverted by `100 - cold`)

## Hotbar

- 7 slots rendered at the bottom center.
- Active slot is highlighted in yellow.
- Clicking a slot sends `{ "type": "slot", "data": { "slot": <index> } }` to the server.
- Active item name is displayed above the hotbar.

## Notes

- Number keys `1-7` switch the active hotbar slot.
- Pointer/touch selection on a slot also switches the active slot.
- Slot switching never triggers a world action; it only changes `active_slot`.
