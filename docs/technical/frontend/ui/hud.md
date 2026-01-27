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

- Number keys 1-7 are not wired to hotbar selection in the current client.
- Hotbar click handling is stubbed in the current reconstruction.
- Active item label rendering is stubbed in the current reconstruction.
