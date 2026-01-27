# Main Menu (The Hub)

The central interface for the game. Layout and behavior are identical across devices.

## In-Game Menu

- **Close Button**: "X" icon in top-right to resume immediately.
- **Tabs**:
  - **Inventory**: Bag management.
  - **Crafting**: Recipe list.
  - **Profile**: Player identity, level, and device login.
  - **Guidebook**: Manual.
  - **Settings**: Audio/graphics.

## Profile Tab

- **Stats**: Time survived, items gathered, kills, deaths.
- **Identity**:
  - **Player ID**: Displayed in full with a copy button.
  - **Copy Action**: Copies the Player ID to clipboard and shows a toast.
- **Device Login**:
  - **Input**: Enter a Player ID to claim that character on this device.
  - **Action**: "Login on this device" requests a new session token.
  - **Behavior**: Existing sessions for the same Player ID are revoked.
- **Customization**:
  - **Name Input**: Click to focus, type to edit.
  - **Action**: "Update Name" or Enter sends the change to the server.

## Session Revoked

- Triggered when the same Player ID logs in elsewhere.
- Shows a blocking overlay with a reconnect/login prompt.

## Game Over Screen

- **Header**: "YOU DIED".
- **Action**: "RESPAWN" button (reloads page/resets session).
- **Visual**: Dark overlay, blocks all other input.
