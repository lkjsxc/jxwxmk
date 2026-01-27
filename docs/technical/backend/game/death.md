# Death + Respawn

## Death Flow

1. Each tick, the engine checks spawned players for `health <= 0`.
2. Dead players are marked as unspawned and a death record is stored.
3. Inventory drop rules are applied (configurable by biome or PvP state).

## Respawn Flow

- Player respawns at their bound settlement core.
- Vitals reset to configured defaults.
- Respawn cooldowns are enforced to prevent abuse.

## Reconnect Flow

- Clients reconnect with their session token.
- The server reattaches the player and restores persisted state.
