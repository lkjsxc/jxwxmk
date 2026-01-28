# Achievements UI

## Toast Notification

- Triggered by `achievement` messages.
- Displays a 3-second toast with title + name.

## Achievements Tab

- Left side: list of all achievements (locked vs unlocked).
- Right side: details for the selected achievement.
- Progress bars are shown only if the client has requirement metadata.

Data sources:

- The set of unlocked achievements comes from `playerUpdate.achievements`.
- `achievement` messages are used for the toast UX (the server should also update `playerUpdate` so the tab stays in sync).

## Client Data Source

- The client uses a hardcoded `ALL_ACHIEVEMENTS` list.
- The list does **not** include requirement metadata, so progress bars are usually hidden.

## Pinning

- Selected achievement can be pinned.
- Pinned achievements appear in the HUD tracker with progress (if requirement metadata is present).
