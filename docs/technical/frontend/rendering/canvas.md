# Canvas Loop

The render loop is driven by `requestAnimationFrame` and interpolates between server snapshots.

## Frame Flow

1. Update input animations (button pulses).
2. Interpolate entity positions using `alpha = (now - lastUpdateAt) / 50`.
3. Move camera toward the local player.
4. Draw background grid and all entities.
5. Render HUD + UI overlays.

## Interpolation

- Each world update overwrites `world` and stores the previous snapshot.
- Per-entity positions are lerped between `prevWorld` and `world`.

## Notes

- No viewport culling is implemented yet.
- Visual hit flashes are handled by client-only `lastHitAt` markers.
