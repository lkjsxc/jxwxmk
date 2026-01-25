# Heads-Up Display (HUD)

The persistent on-screen overlay during gameplay.

## Elements

1.  **Vitals Bars** (Top Left):
    -   **Health**: Semi-transparent Red (`rgba(255, 0, 0, 0.5)`).
    -   **Hunger**: Semi-transparent Orange (`rgba(255, 165, 0, 0.5)`).
    -   **Temperature**: Semi-transparent Blue/Red based on state.
    -   **Labels**: Displayed to the left ("HP", "HG", "TP").

2.  **Target Status**:
    -   **Object HP**: When hitting a resource or structure, its name and a semi-transparent HP gauge are displayed at the top-center of the screen, "above the user's" view.

3.  **Hotbar** (Bottom Center):
    -   **7 Slots** (Keys 1-7).
    -   Translucent background.
    -   Selection via Keyboard or Mouse Click.

4.  **Action Buttons** (Mobile):
    -   **A/B Buttons**: Translucent, pulse/enlarge when used or during cooldown transition.
    -   Positioned higher to avoid accidental OS gesture triggers.
