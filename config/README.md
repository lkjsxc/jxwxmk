# Configuration

Configuration files for the game server.

## Files

- `server.json` - Server settings (port, tick rate, rate limits)
- `world.json` - World generation (chunk size, radii, seed)
- `balance.json` - Game balance (player stats)
- `survival.json` - Survival mechanics (hunger, temperature)
- `crafting.json` - Crafting recipes

## Loading

Server loads all `*.json` files from `/app/config/` at startup.
