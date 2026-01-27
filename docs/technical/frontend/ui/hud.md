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
- Clicking a slot sends a `slot` message to the server.
- Active item name is displayed above the hotbar.

## Notes

- Number keys 1-7 are not wired to hotbar selection in the current client.
