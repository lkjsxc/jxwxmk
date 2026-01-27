# Crafting System

Crafting is entirely server-authoritative and operates on the player's inventory.

## Recipes

Current recipes (`CraftingSystem::get_recipes()`):

- `WoodPickaxe`: Wood x10
- `StonePickaxe`: Wood x10, Stone x10
- `WoodWall`: Wood x20
- `Door`: Wood x30
- `Torch`: Wood x2
- `Workbench`: Wood x50

## Craft Flow

1. Client sends `{"craft": "<ItemType>"}`.
2. Server checks recipe and inventory availability.
3. Ingredients are consumed across slots (stacked or split).
4. Output item is added to inventory (stacked where possible).
5. `stats.items_crafted` increments.
