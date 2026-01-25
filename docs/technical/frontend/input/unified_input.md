# Input System

Unified handling of Mouse and Touch events for cross-platform support.

## Architecture

### InputManager
-   **Abstraction**: Maps hardware events (Keyboard, Mouse, Touch) to a logical `InputState`.
-   **Cooldown Enforcement**: 
    -   Prevents sending rapid-fire actions to the server.
    -   **A Button (Attack/Use)**: Default 500ms cooldown.
    -   **B Button (Interact)**: Default 300ms cooldown.
-   **Pointer Tracking**:
    -   `mouseX / mouseY`: Tracks the primary interaction point.
    -   `isPointerDown`: Unified flag for `mousedown` or `touchstart`.
-   **Touch Zones**:
    -   Left side of screen: Reserved for Virtual Joystick.
    -   Right side of screen: Reserved for A/B action buttons.
    -   UI Layer: Overrides game-world input when a menu is active.

### UIManager
-   Uses normalized coordinates from `InputManager` to perform hit-testing against UI elements (Buttons, Tabs, Grid slots).
-   **Event Consumption**: If a pointer event hits a UI element, it prevents the action from being passed to the game world (e.g., clicking a button doesn't trigger an attack).
