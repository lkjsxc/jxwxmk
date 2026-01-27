# Crafting UI

## Layout

- Left panel: recipe list.
- Right panel: details and requirements for the selected recipe.

## Recipes (Client-side Display)

- Wood Pick (`WoodPickaxe`): Wood x10
- Stone Pick (`StonePickaxe`): Wood x10, Stone x10
- Wood Wall (`WoodWall`): Wood x20
- Door (`Door`): Wood x30
- Torch (`Torch`): Wood x2
- Workbench (`Workbench`): Wood x50

## Interaction

- Selecting a recipe highlights it and shows requirements.
- Clicking Craft sends `{ "type": "craft", "data": { "recipe": "<ItemType>" } }`.
- Craft button is enabled only if requirements are met (client-side check).

## Server Authority Note

The server enforces recipe requirements; the client display mirrors the server-side recipe table.
