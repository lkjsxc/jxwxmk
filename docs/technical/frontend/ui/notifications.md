# Notifications + Tracker

## Toasts

- Used for achievements, general notifications, and recoverable errors.
- Single active toast displayed near the bottom center.

### Error toasts

- The `error` protocol message (see: `docs/technical/backend/server/protocol.md`) should surface as a toast.
- If the error code maps to a specific UI surface (e.g., crafting), the UI may also highlight that surface.

## NPC Interaction Overlay

- Centered modal showing NPC name, dialogue text, and options.
- Clicking an option sends `npcAction`.
- Options like "Goodbye" close the modal.

## Pinned Tracker

- Displays a compact tracker for one pinned quest and/or achievement.
- Shows objective progress bars in the top-right area.
