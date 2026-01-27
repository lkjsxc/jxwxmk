# Profile UI

## Displayed Fields

- Player ID (full) + copy button.
- Player level + XP progress.
- Stats: kills, deaths, crafted, steps.
- Session state: connected / revoked / invalid.

## Copy Player ID

- Copy button writes full Player ID to clipboard.
- Show a brief toast confirming the copy.

## Name Editing

- Clicking the name field focuses input.
- Keyboard input is appended to the name buffer.
- Clicking "Update Name" sends `{ "name": "<new_name>" }`.

## Device Login

- Input field accepts a Player ID to claim on this device.
- Clicking "Login on this device" requests a new session token.
- Server revokes any existing session for that Player ID (single-session rule).
- On revoke, the previous device receives `sessionRevoked` and is disconnected.
- Invalid IDs show a blocking error toast and do not change local session.
