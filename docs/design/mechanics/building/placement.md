# Building Mechanics

Players can place structures in the world.

## Placement Logic
1.  **Select**: Equip a structure item (e.g., Wall) in hand.
2.  **Preview**: A semi-transparent "ghost" version follows the cursor/player.
    -   **Green**: Valid location.
    -   **Red**: Invalid (collides with player, water, or other structure).
3.  **Place**: Input "Attack" or "Interact" to solidify the structure.
    -   Consumes item from inventory.
    -   Updates server grid.

## Restrictions
-   Cannot place within enemy spawn zones (optional).
-   Grid snapping (structures align to a `2.5wu` grid, ~40px at zoom 1.0).
