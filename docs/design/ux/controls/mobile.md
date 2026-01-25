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
- **A Button (Attack)**:
    -   Large circular button (radius 40px) at bottom-right.
    -   Hold to auto-repeat (Gather/Attack).
- **B Button (Interact/Build)**:
    -   Smaller button near A Button.
    -   Used for:
        -   Opening Doors/Chests.
        -   Placing the currently selected building item.
        -   Eating food (if held).

### UI Overlay
- **Hotbar**: Centered bottom. Tap to select item.
- **Crafting Menu**: Toggle button top-right.
- **Chat**: Semi-transparent overlay top-left.

## Gestures
- **Pinch**: Zoom camera.
- **Two-finger Tap**: Cancel/Back.
