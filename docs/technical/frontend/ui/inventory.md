# Inventory UI

## Layout

- 30 slots in a grid.
- Grid width adapts to screen width (3, 5, or 7 columns).

## Interaction

- Clicking an item starts a drag operation.
- Releasing over another slot sends `swapSlots` to the server.
- Dragged item is rendered under the pointer.
