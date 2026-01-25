# Inventory & Hotbar

## Hotbar
-   **Always Visible**: Bottom center of HUD.
-   **Slots**: 7 (Keys 1-7).
-   **Selection**:
    -   **Keyboard**: Press 1-7 to select.
    -   **Clicking**: Hotbar slots are clickable for selection on all platforms.
    -   **Logic**: Clicking a hotbar slot *only* selects the slot; it does not trigger the item's use (A button action).
    -   **Visual**: Highlight active slot with a yellow border.
    -   **Translucency**: UI elements use semi-transparent backgrounds.
-   **Function**: The "Active Slot" determines what item is held/used (e.g., placing a wall, eating food).

## Design (Minecraft/Terraria Feel)

- **Visuals**: Dark, high-contrast grid with inset borders. Items are rendered as distinct shapes within these squares.

- **Tooltip**: Hovering over a slot shows the item name and quantity in a small floating box.



## Drag & Drop (D&D)

- **Mechanics**:

    - **Click & Drag**: Left-click an item and hold to move it. The item follows the cursor.

    - **Drop**: Releasing over another slot swaps the items.

    - **Split**: (Future) Right-click or modifier keys.

- **Hotbar Sync**: The hotbar is the first row of the inventory. Moving items within the menu automatically updates the hotbar.

- **Server Sync**: Drag-and-drop actions send a `SwapSlots(from, to)` message to the server to maintain authority.