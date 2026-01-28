# Game Crate

Tick loop and engine orchestration.

## Purpose

Owns the authoritative world state and runs the fixed tick loop.

## Single-Writer Model

- Only `GameEngine` mutates world state
- Network handlers enqueue `GameEvent`s
- Events processed deterministically each tick

## Tick Order

1. Dequeue input events
2. Activate/deactivate chunks
3. Run systems (survival, combat, etc.)
4. Update spawn budgets
5. Build delta updates
6. Broadcast to clients

## GameEvent Types

- `Join` - Player connected
- `Leave` - Player disconnected
- `Input` - Player input message

## WorldHandle

Narrow handle for read-only world access from other crates.
