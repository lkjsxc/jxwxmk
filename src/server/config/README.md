# Config Crate

Configuration loading and validation.

## Purpose

Loads JSON config files from `/app/config/` and provides typed access.

## Config Files

- `server.json` - HTTP/WebSocket ports, tick rate, rate limits
- `world.json` - Chunk size, streaming radii, world seed
- `balance.json` - Player/mob stats, item scaling
- `survival.json` - Hunger, temperature, thirst rates
- `crafting.json` - Recipe tables

## Validation

- Rejects unknown fields (`deny_unknown_fields`)
- Provides sensible defaults for optional fields
- Fails fast on invalid config at startup
