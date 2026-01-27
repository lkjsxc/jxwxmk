# Keyboard + Mouse Input

## Keyboard Mapping

- **WASD**: movement vector.
- **E**: interact action.
- **1-7**: key state recorded but not currently used for hotbar selection.
- **Text input**: raw keys are pushed to `keyQueue` for the profile name field.

## Mouse Mapping

- **Left click**: interacts with on-screen UI and attacks in-world on tap.
- **Click-and-hold**: triggers interact on long-press when in range.
- **Right click**: context menu suppressed; no gameplay binding.
- **Wheel**: zoom in/out.

## Notes

- Primary attack is triggered by tap/click in the world. It is not mapped to a standalone keyboard key.
- UI clicks consume the pointer to avoid sending gameplay input.
