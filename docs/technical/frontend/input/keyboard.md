# Keyboard + Mouse Input

Keyboard/mouse input mirrors touch gestures; it does not add unique gameplay rules.

## Keyboard Mapping

- **W/A/S/D**: Movement vector.
- **E**: Interact action (same as long-press).
- **1-7**: Switch active hotbar slot (sends `{ "type": "slot", "data": { "slot": <index> } }`).
- **Text input**: Raw keys are pushed to `keyQueue` for name fields.

## Mouse Mapping

- **Left click**: Interacts with on-screen UI and attacks in-world on tap.
- **Click-and-hold (~250-300ms)**: Triggers interact on long-press when in range.
- **Right click**: Context menu suppressed; no gameplay binding.
- **Wheel**: Zoom in/out.

## Notes

- Primary attack is triggered by tap/click in the world; no dedicated key.
- UI clicks consume the pointer to avoid sending gameplay input.
