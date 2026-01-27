# Death + Respawn

## Death Flow

1. Each tick, the engine checks spawned players for `health <= 0`.
2. Dead players are marked as:
   - `spawned = false`
   - `health = 0`
   - `inventory = Inventory::default()`
   - `stats.deaths += 1`
3. Unspawned players are excluded from world broadcasts.

## Respawn Flow

- Client sends `{ "spawn": true }`.
- Server resets:
  - Position to a random point within `spawn_radius` around world center.
  - `health = 100`, `hunger = 100`, `cold = 50`.
  - `spawned = true`.

## Reconnect Flow

- Client connects with `?token=<uuid>`.
- If token matches an existing player, that player entity is reused.
- The welcome message includes a `spawned` flag so the client can decide to respawn.
