# Heads-Up Display (HUD)

The persistent on-screen overlay during gameplay.

## Elements

1.  **Vitals Bars** (Top Left):
    -   **Health**: Semi-transparent Red (`rgba(255, 0, 0, 0.5)`).
    -   **Hunger**: Semi-transparent Orange (`rgba(255, 165, 0, 0.5)`).
    -   **Temperature**: Semi-transparent Blue/Red based on state.
    -   **Labels**: Displayed to the left ("HP", "HG", "TP").

2.  **Target Status**:
    -   **Object HP**: When an entity (Resource, Mob, Structure) is damaged, a semi-transparent HP gauge is displayed directly above the object in the game world.
    -   **Fade**: The gauge only appears when HP is below 100% or recently hit.

3.  **Hotbar** (Bottom Center):
    -   **7 Slots** (Keys 1-7).
    -   Translucent background.
    -   Selection via Keyboard or Mouse Click.

4.  **Interaction Gestures**:
    -   **No On-Screen A/B Buttons**: Actions are performed via tap and long-press.
    -   **Gesture Hints**: Optional small labels near the reticle to remind Tap/Long-Press mapping.
