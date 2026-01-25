# Heads-Up Display (HUD)

The persistent on-screen overlay during gameplay.

## Elements

1.  **Vitals Bars** (Top Left):
    -   **Labels**: "HP", "HG" (Hunger), "TP" (Temp) displayed to the left of each bar.
    -   **Visual**: Translucent backgrounds and fills (alpha ~0.6).
    -   **Health**: Red.
    -   **Hunger**: Orange.
    -   **Temperature**: Blue to Red.

2.  **Entity HP Gauges**:
    -   Displayed above Resources, Mobs, and Structures only when their HP is not at maximum.
    -   Small, translucent green/red bar.

3.  **Hotbar** (Bottom Center):
    -   **7 Slots** (Keys 1-7).
    -   Translucent background.
    -   Selection via Keyboard or Mouse Click.

4.  **Action Buttons** (Mobile):
    -   **A/B Buttons**: Translucent, pulse/enlarge when used or during cooldown transition.
    -   Positioned higher to avoid accidental OS gesture triggers.
