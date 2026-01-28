# Tick + Time Contract

## Fixed tick

- The simulation runs at a fixed tick rate configured by `server.tick_rate` (target 20–60Hz).
- The simulation step uses a fixed `dt = 1.0 / tick_rate` seconds. Do not use variable `dt` for simulation decisions.

## Tick order (canonical)

Per `docs/technical/backend/game/engine.md`, each tick executes in this order:

1. Dequeue validated input events into a bounded queue.
2. Activate/deactivate chunks based on player positions.
3. Run systems (survival, combat, AI, crafting, quests, etc.).
4. Update regeneration timers and spawn budgets.
5. Build per-player delta updates.
6. Broadcast deltas to interested clients.

## Determinism seams

- Simulation-affecting randomness must use a seeded RNG stream owned by the tick loop.
- Input ordering must be deterministic within a tick:
  - do not depend on hash iteration order
  - do not depend on wall-clock timing of message arrival beyond queue insertion order

## Backpressure (bounded queues)

All queues must be bounded and have defined overflow behavior.

Minimum required behaviors:

- If the inbound WS message rate exceeds server limits, the server must:
  - drop/ignore excess messages and/or
  - disconnect the client (with a reason)
- If the engine input queue is full, the server must not allocate unbounded memory; it must:
  - drop oldest/newest (documented choice), or
  - reject the sender (disconnect or throttle)

Overflow behavior must be visible via logs/metrics (see: `docs/technical/operability/README.md`).
