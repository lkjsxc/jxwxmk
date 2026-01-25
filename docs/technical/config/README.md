# Configuration

The server is configurable via `config.json`.

## Structure

```json
{
  "server": {
    "port": 8080,
    "tick_rate": 20
  },
  "game": {
    "world_width": 2000,
    "world_height": 2000,
    "spawn_protection": false
  },
  "mechanics": {
    "hunger_decay": 0.05,
    "cold_decay": 0.05
  }
}
```

## Loading
- Loaded at startup.
- Values override defaults.
- Hot-reloading is not required for this iteration.
