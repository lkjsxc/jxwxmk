# Interaction System

Handles movement, actions, and NPC interactions.

## Movement

- Movement input is applied per tick.
- Speed uses `balance.player.base_speed` with stat bonuses.
- Position updates are clamped to chunk-local constraints (no global bounds).
- Steps increment stats for achievements.

## Action Resolution (Priority)

1. **Consume**: use consumable in active slot.
2. **Place**: place structure item if valid in chunk.
3. **Gather**: apply tool damage to nearest resource node.
4. **Attack**: resolve combat against mobs or players.

The server resolves actions authoritatively; the client never “assumes” a consume/place/gather/attack succeeded.

### Targeting (`aim`)

For actions that need a target (place/gather/attack/NPC interaction), the server uses the `aim` point provided by the `input` message (see: `docs/technical/backend/server/protocol.md`):

- `aim` is world-space (`wu`) and must be validated server-side (finite numbers, max range from player, etc.).
- The server picks the best target near `aim` within interaction range.
  - If no valid target exists, the action is a no-op (or yields a notification), but must not mutate state.

## Consume

- If the active slot contains a consumable item and `interact` is true, the server:
  - validates the item is consumable
  - consumes 1 unit (or the item’s configured consume amount)
  - applies the item effects (at minimum: hunger restore and/or health restore)
  - clamps vitals to their max values
- Consumption must be rate-limited (configurable) to prevent spam.

## Placement

Placement is driven by the active slot containing a structure item and the player triggering the place action.

- Structures snap to a `2.5wu` grid (see: `docs/design/world/scale_and_chunks.md`).
- A placement is valid only if:
  - `aim` is present and within interaction range
  - the structure does not overlap another structure collider
  - the structure does not overlap the player collider
- On success:
  - consume 1 structure item from inventory
  - add a structure entity to the chunk state
  - mark the chunk dirty for persistence checkpointing
  - broadcast via `entityDelta`

## Tool Scaling

- Tool damage and tiers are defined in `config/balance.json`.
- Tool level increases with XP and affects yield/damage.

## NPC Interaction

- Interact triggers NPC dialogue and trade payloads.
- Quest acceptance and turn-in are validated server-side.
