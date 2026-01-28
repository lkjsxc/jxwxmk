# `economy.json`

Purpose: NPC vendor pricing and currency sinks.

References:
- `docs/design/mechanics/economy_trade.md`

## Schema (v1)

```json
{
  "version": 1,
  "tax_rate": 0.05,
  "vendor_prices": {
    "wood": { "buy": 1, "sell": 0 },
    "stone": { "buy": 1, "sell": 0 }
  }
}
```

## Validation rules

- `tax_rate` is finite and in `[0.0, 1.0]`.
- `vendor_prices` keys are `snake_case` item IDs.
- `buy`/`sell` are integers >= 0.

## Notes

- Currency and multi-tier pricing can be added in later versions; this schema defines a simple baseline.
