# Camera

The camera smooth-follows the local player and supports zoom.

Rendering scale follows the canonical mapping in the world design docs:
- `ppu = 16` pixels per world unit at `zoom = 1.0`.
- `screen_px = world_wu * ppu * zoom`.
- See: `../../../design/world/scale_and_chunks.md`

## Follow Behavior

- `Camera.follow(targetX, targetY)` sets the target.
- `Camera.update()` lerps current position toward target (`0.1` factor).
- On first spawn, the camera snaps to the player position.

## Zoom

- Mouse wheel adjusts zoom by +/-0.1.
- Default zoom is `1.0` (recommended: start at `1.1` on spawn for readability).
- Zoom is clamped between `0.75` and `2.0` (avoid “too far away” framing).

## Notes

- The world is unbounded; the camera does not clamp to world edges.
