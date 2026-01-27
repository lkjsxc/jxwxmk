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

Current reconstruction only implements Gather/Attack + NPC interactions; Consume/Place are queued for follow-up.

## Tool Scaling

- Tool damage and tiers are defined in `config/balance.json`.
- Tool level increases with XP and affects yield/damage.

## NPC Interaction

- Interact triggers NPC dialogue and trade payloads.
- Quest acceptance and turn-in are validated server-side.
