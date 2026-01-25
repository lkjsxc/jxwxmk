# Mobile Controls (Touch)

Since the game supports smartphones, a Virtual Joystick overlay is required.

## Layout

### Left Zone (Movement)
- **Type**: Dynamic Floating Joystick.
- **Trigger**: Touch start on left half of screen.
- **Logic**: 
    - Center = Initial touch point.
    - Drag = vector for movement (WASD equivalent).
    - Deadzone = 10px.
    - Max Range = 100px.

### Right Zone (Action)
- **A Button (Action/Attack)**:
    -   Primary interaction button.
    -   **Contextual Actions**:
        -   **Holding Food**: Eats the item.
        -   **Holding Placeable**: Places the structure.
        -   **Holding Tool/Weapon**: Gathers resources or attacks mobs.
    -   Hold to auto-repeat.
- **B Button (Interact)**:
    -   Secondary interaction button.
    -   Used for world-object interactions:
        -   Opening Doors/Chests.
        -   Accessing Workbenches/Furnaces.

### UI Overlay
- **Hotbar**: Centered bottom. Tap to select item.
- **Crafting Menu**: Toggle button top-right.
- **Chat**: Semi-transparent overlay top-left.

## Gestures
- **Pinch**: Zoom camera.
- **Two-finger Tap**: Cancel/Back.
