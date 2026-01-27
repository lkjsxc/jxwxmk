# Screens

## Game Over

- Triggered when the local player is missing from chunk updates.
- Displays a "YOU DIED" overlay and a Respawn button.
- Clicking Respawn sends a `spawn` message (no token reset).

## Menu Overlay

- Toggle via the top-right menu button.
- Tabs: Inventory, Crafting, Profile, Quests, Achievements.
- Menu consumes pointer input while open.
- Profile tab includes Player ID copy and a Device Login form (enter Player ID to claim).

## Session Revoked

- Triggered when the server invalidates the current session (logged in elsewhere).
- Shows a blocking overlay with a reconnect/login prompt.
