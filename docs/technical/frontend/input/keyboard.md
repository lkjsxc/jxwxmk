# Keyboard + Mouse Input

## Keyboard Mapping

- **WASD**: movement vector.
- **E / B**: interact action.
- **1-7**: key state recorded but not currently used for hotbar selection.
- **Text input**: raw keys are pushed to `keyQueue` for the profile name field.

## Mouse Mapping

- **Left click**: interacts with on-screen UI and A/B buttons.
- **Right click**: context menu suppressed; interact requires B button or keyboard.
- **Wheel**: zoom in/out.

## Notes

- Primary attack is triggered by the on-screen A button (mouse click on button or touch). It is not mapped to a standalone keyboard key.
- UI clicks consume the pointer to avoid sending gameplay input.
