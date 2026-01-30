# Crafting UI

References:
- `docs/technical/backend/server/protocol.md` (`craft`)
- `docs/technical/config/schemas/crafting.md` (`crafting.json`)
- `docs/design/mechanics/crafting/recipes.md`

## Layout

- Left panel: recipe list.
- Right panel: details and requirements for the selected recipe.

## Recipes (Client-side Display)

The recipe list shown in the UI is derived from `config/crafting.json` (source of truth).

Initial recipe set (see: `docs/design/mechanics/crafting/recipes.md`):

- Wood Pickaxe (`wood_pickaxe`): Wood x10
- Stone Pickaxe (`stone_pickaxe`): Wood x10, Stone x10
- Wood Wall (`wood_wall`): Wood x20
- Door (`wood_door`): Wood x30
- Torch (`torch`): Wood x2
- Workbench (`workbench`): Wood x50

## Interaction

- Selecting a recipe highlights it and shows requirements.
- Clicking Craft sends `{ "type": "craft", "data": { "recipe": "<recipe_id>" } }` where `<recipe_id>` is the selected recipe’s `id` (snake_case).
- Craft button is enabled only if requirements are met (client-side check).

## Server Authority Note

The server enforces recipe requirements. To prevent client/server drift, the client’s recipe display must be generated from the same `crafting.json` used by the server (build-time is sufficient; no runtime config endpoint is required).
