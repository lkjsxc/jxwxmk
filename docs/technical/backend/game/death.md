# Death and Reconnection Logic

This document describes how the server handles player death and reconnection.

## Death Flow

1. **Detection**: During each world tick (`tick_world`), the server checks if any spawned player's health has reached 0 or less.
2. **Transition**: When a player dies:
    - `p.spawned` is set to `false`.
    - `p.health` is reset to `0.0`.
    - `p.stats.deaths` is incremented.
    - `p.inventory` is cleared (using `Inventory::default()`).
3. **Notification**: The player's ID remains in the server's authoritative state (for reconnection), but they are marked as not spawned. 
4. **Disappearance**: The server filters out unspawned players from the world broadcast. This ensures dead players "disappear" from the view of other players.
5. **Client Death**: On the next world update, the client detects that its local player is missing from the broadcast (or marked dead) and triggers the Game Over screen.

## Reconnection Flow

1. **Join**: The client connects via WebSocket, optionally providing a `secret_token`.
2. **Identification**:
    - If the token matches an existing player, that player entity is "possessed".
    - If no token is provided or no match is found, a new player entity is created with a new token.
3. **Welcome**: The server sends a `welcome` message containing the player ID, token, and a `spawned` boolean.
4. **Possession vs Respawn**:
    - If `spawned` is `true`, the client resumes gameplay immediately (reconnection to an alive character).
    - If `spawned` is `false`, the client may automatically request a spawn (for new players) or stay at the menu/Game Over screen.

## Respawn Flow

1. **Request**: The client sends a `Spawn` message (triggered by "Play" or "Respawn" buttons).
2. **Initialization**:
    - `health`, `hunger`, and `cold` are reset to their default maximums.
    - The player is moved to a random position within the spawn radius.
    - `spawned` is set to `true`.
