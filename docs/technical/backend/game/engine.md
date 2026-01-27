# Engine

The engine is implemented as an Actix actor (`GameEngine`) that owns the world state and processes messages.

## Startup

- Loads `AppConfig` from `config.json`.
- Builds a new `World` with configured dimensions.
- Spawns initial entities (resources, mobs, barriers, NPCs).
- Starts a fixed tick loop at `server.tick_rate` Hz.

## Tick Loop

Each tick runs in this order:

1. Barrier checks (remove hostile mobs inside barrier ranges).
2. Survival tick (hunger, temperature, healing, damage).
3. Mob AI movement and target selection.
4. Mob damage to nearby players.
5. Death cleanup (set `spawned = false`, clear inventory).
6. Broadcast world snapshot.

## Input Handling

- Client input messages are handled immediately when received by the `GameEngine` actor.
- There is no input queue; ordering is the Actix message order.
- All mutations still occur on the single engine actor thread (no concurrent writes).

## Engine Messages

`GameEngine` handles Actix messages for:

- `Join` / `Leave` (session lifecycle)
- `Spawn`
- `Input` (movement + attack + interact)
- `Craft`
- `SelectSlot`
- `UpdateName`
- `SwapSlots`
- `NpcAction`
- `Trade` (placeholder)
- `AcceptQuest`

Outbound messages to clients are sent as `ServerMessage` values (world update, achievements, notifications, quest updates, NPC interactions).
