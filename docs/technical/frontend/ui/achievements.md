# Frontend Achievement UI

## HUD Notification
- **Component**: `AchievementToast`
- **Logic**:
    - Listens for `AchievementUnlocked` WS message.
    - Pushes notification to a queue.
    - Renders overlay on `Canvas` or HTML DOM (prefer Canvas for unity with game render).
    - Timer handles the 3s lifetime.

## Menu Screen
- **Tab**: Add "Achievements" button to Main Menu.
- **Layout**: Grid or List view.
    - **Icon**: Placeholder or generated.
    - **Title**: Bold text.
    - **Desc**: Small text.
    - **Progress**: `10/100` textual or visual bar.
    - **Status**: Gold border for unlocked, gray for locked.

## State Management
- `AppState` includes `achievements: Map<string, AchievementStatus>`.
- `AchievementStatus` tracks unlocked state and current progress.
