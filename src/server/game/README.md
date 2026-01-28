# Game Engine

The core simulation logic, isolated from the network layer.

## Responsibilities

- **Tick Loop**: Updates the world at a fixed rate.
- **World State**: Manages chunks, entities, and players.
- **Event Queue**: Processes incoming inputs deterministically.

## Modules

- `engine.rs`: The `GameEngine` struct and tick loop.
- `world.rs`: `World` struct and spatial management.
- `entities.rs`: Entity struct definitions.
- `systems/`: Gameplay system implementations.
