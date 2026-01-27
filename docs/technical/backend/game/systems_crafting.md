# Crafting System

Crafting is server-authoritative and data-driven.

## Recipes

- Recipes load from `config/crafting.json` and `config/balance.json`.
- Each recipe includes required station tier and materials.

## Craft Flow

1. Client sends `craft` with recipe ID.
2. Server validates station proximity and player tier.
3. Ingredients are consumed from inventory.
4. Output item is added with derived level.
5. Craft stats and XP are updated.
