# Camera

The camera smooth-follows the local player and supports zoom.

## Follow Behavior

- `Camera.follow(targetX, targetY)` sets the target.
- `Camera.update()` lerps current position toward target (`0.1` factor).
- On first spawn, the camera snaps to the player position.

## Zoom

- Mouse wheel adjusts zoom by +/-0.1.
- Zoom is clamped between `0.5` and `2.0`.

## Notes

- The camera is not clamped to world bounds yet.
