# Input Handling

The input manager abstracts hardware differences between Mobile and Desktop.

## Abstraction Layer
`InputState` object sent to server:
```json
{
  "dx": 0.5,   // Normalized X (-1.0 to 1.0)
  "dy": -0.5,  // Normalized Y (-1.0 to 1.0)
  "attack": true,
  "rotation": 1.57 // Radians (optional, mostly for mouse aim)
}
```

## Implementations
- [Touch](touch.md)
- [Keyboard/Mouse](keyboard.md)
