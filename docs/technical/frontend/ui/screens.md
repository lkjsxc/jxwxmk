# Screens

## Game Over

- Triggered when the local player is missing from chunk updates.
- Displays a "YOU DIED" overlay and a Respawn button.
- Clicking Respawn sends a `spawn` message (no token reset).

## Menu Overlay

- Toggle via the top-right menu button.
- Pressing the menu button shows a page list directly underneath it.
- Selecting a page opens that page; pages are independent and contain no buttons/links to other pages.
- Each page has only an "X" close button in the top-right; switching pages is done via the page list.
- Menu consumes pointer input while open.
- Profile page includes Player ID copy and a Device Login form (enter Player ID to claim).

## Session Revoked

- Triggered when the server invalidates the current session (logged in elsewhere).
- Shows a blocking overlay with a reconnect/login prompt.
