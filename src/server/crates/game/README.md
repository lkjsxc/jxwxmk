# Game Crate

Game engine with fixed tick loop and event processing.

## Modules

- `engine.rs`: GameEngine with tick loop, event queue, system orchestration
- `events.rs`: GameEvent and GameResponse types

## Architecture

- Single-writer: Only GameEngine mutates World
- Event queue: I/O handlers enqueue, tick loop processes
- Fixed tick rate: 20-60Hz configurable
