# `crafting.json`

Purpose: data-driven recipes for server-authoritative crafting.

References:
- `docs/technical/backend/game/systems_crafting.md`
- `docs/technical/backend/server/protocol.md` (`craft`)
- `docs/design/mechanics/crafting/recipes.md`

## Schema (v1)

```json
{
  "version": 1,
  "recipes": [
    {
      "id": "wood_pickaxe",
      "station": "hand",
      "inputs": [{ "item": "wood", "count": 10 }],
      "output": { "item": "wood_pickaxe", "count": 1 }
    }
  ]
}
```

## Validation rules

- `recipes` is an array with unique `id` values.
- `id` is `snake_case`.
- `station` is one of: `hand`, `workbench`, `forge`, `tannery`, `alchemy`, `carpentry`.
- `inputs[*].item` and `output.item` are `snake_case` item IDs.
- All `count` values are integers > 0.

## Notes

- Station gating may be a no-op for the initial implementation if explicitly documented, but the field exists for forward compatibility.
