# Game Module (Engine)

The central orchestrator and authoritative game loop.

## Responsibilities
- The fixed tick loop.
- Event queue management.
- Triggering systems.
- Broadcasting deltas.
- Managing the `World` instance.

## Dependencies
- `systems`
- `world`
- `protocol`
- `persistence`
- `config`
