# NPCs (Non-Player Characters)

NPCs are stationary entities that players can interact with.

## NPC Types

- **Elder**: quest giver.
- **Merchant**: trade placeholder (inventory exists but no trading logic).
- **Guard**: minimal dialogue.

## Properties (Current)

- `id`, `npc_type`, `name`, `x`, `y`, `health`
- `dialogue_index` (reserved for future use)
- `trade_inventory` (empty list for Merchant, `None` otherwise)

## Interactions

- Dialogue options are served by the server.
- Options can trigger quest acceptance or completion.
