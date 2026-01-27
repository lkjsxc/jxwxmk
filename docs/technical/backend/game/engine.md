# Engine

`GameEngine` owns world state, the chunk cache, and the fixed tick loop.

## Startup

- Loads configuration from `config/`.
- Initializes world seed and chunk manager.
- Spawns initial settlements and starting chunks.
- Starts fixed tick loop at `server.tick_rate` Hz.

## Tick Loop

Each tick runs in this order:

1. Dequeue input events into a bounded queue.
2. Activate/deactivate chunks based on player positions.
3. Run systems (survival, combat, AI, crafting, quests).
4. Update regeneration timers and spawn budgets.
5. Build per-player delta updates.
6. Broadcast deltas to interested clients.

## Input Handling

- WebSocket handlers enqueue validated input events.
- The engine is the only writer to world state.
- Input ordering is deterministic within a tick.

## Engine Messages

`GameEngine` handles:

- `Join` / `Leave`
- `Spawn` / `BindSettlement`
- `Input` (movement + actions)
- `Craft`
- `Trade`
- `NpcAction`
- `AcceptQuest`
