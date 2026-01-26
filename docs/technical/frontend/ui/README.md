# User Interface

This directory details the frontend UI components and systems.

## Components
- **HUD**: Heads-Up Display (Health, Hunger, Temperature bars).
- **Inventory**: Grid-based inventory management.
- **Crafting**: Recipe list and crafting details.
- **Achievements**: Achievement list and notifications.
- **Profile**: Player stats and name management.
- **Screens**: Game Over screen.

## Logic
The UI is managed by the `UIManager` class, which handles:
- Input routing (blocking clicks on UI elements).
- State management (`activeTab`, `isMenuOpen`).
- Rendering (drawing to the Canvas overlay).
