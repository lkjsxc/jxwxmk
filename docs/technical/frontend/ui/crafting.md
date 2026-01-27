# Crafting UI

## Layout

- Left panel: recipe list.
- Right panel: details and requirements for the selected recipe.

## Recipes (Client-side Display)

- Wood Pick (`WoodPickaxe`): Wood x10
- Stone Pick (`StonePickaxe`): Wood x10, Stone x10
- Wood Wall (`WoodWall`): Wood x20
- Door (`Door`): Wood x15 (server requires x30)
- Torch (`Torch`): Wood x2
- Workbench (`Workbench`): Wood x50

## Interaction

- Selecting a recipe highlights it and shows requirements.
- Clicking Craft sends `{ "craft": "<ItemType>" }`.
- Craft button is enabled only if requirements are met (client-side check).

## Server Authority Note

The server enforces recipe requirements. The Door recipe mismatch (15 vs 30 wood) is a current client/server inconsistency.
