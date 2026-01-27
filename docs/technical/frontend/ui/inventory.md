# Inventory UI

## Layout

- 30 slots in a grid.
- Grid width adapts to screen width (3, 5, or 7 columns).

## Interaction

- Clicking an item starts a drag operation.
- Releasing over another slot sends `{ "type": "swapSlots", "data": { "from": <index>, "to": <index> } }`.
- Dragged item is rendered under the pointer.

Current reconstruction renders the grid but does not yet implement drag state; swap wiring remains a follow-up.
