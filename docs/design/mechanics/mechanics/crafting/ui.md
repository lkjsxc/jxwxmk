# Crafting UI Design

## Layout
- **Split View**:
    - **Left**: Scrollable list of craftable items (Icon + Name only).
    - **Right**: Details pane for the selected item.

## Interaction
- **Select**: Clicking an item in the list selects it.
- **Craft**: Clicking the "Craft" button in the details pane attempts to craft.

## Details Pane Content
- **Item Preview**: Large icon.
- **Name**: Item name.
- **Description**: Brief description (optional).
- **Requirements List**:
    - List of materials needed vs owned (e.g., "Wood: 5/10").
    - Red text if insufficient.
- **Craft Button**:
    - Enabled only if materials are sufficient.
    - Text: "Craft".
