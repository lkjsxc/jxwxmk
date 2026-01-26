# Main Menu (The Hub)

The central interface for **kkmypk**.

## States

### 1. In-Game Hub
- **Close Button**: 'X' icon in top-right corner to resume game immediately.
- **Tabs**:
    - **Inventory**: Bag management.
    - **Crafting**: Recipe list.
    - **Profile**: Player information and customization.
    - **Guidebook**: Manual.
    - **Settings**: Audio/Graphics.

### 4. Profile Tab
- **Stats**: Total time survived, items gathered.
- **Customization**:
    - **Name Input Box**: A simulated text field. Displays the player's current username by default. Clicking it focuses the input for editing.
    - **Keyboard Input**: While focused, typing updates the character buffer in real-time.
    - **Visual**: Displays a blinking cursor (|) when focused.
    - **Action**: "Update Name" button or pressing "Enter" sends the change to the server.

### 3. Game Over Screen
A dedicated overlay when `HP <= 0`.
- **Header**: "YOU DIED".
- **Action**: "RESPAWN" button (reloads page/resets session).
- **Visual**: Dark gray/black overlay, blocking all other input.