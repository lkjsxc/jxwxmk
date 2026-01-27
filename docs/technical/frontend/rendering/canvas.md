# Canvas Loop

The render loop is driven by `requestAnimationFrame` and interpolates between chunk deltas.

## Frame Flow

1. Update input animations (button pulses).
2. Interpolate entity positions using last delta timestamps.
3. Move camera toward the local player.
4. Draw background grid and visible chunks.
5. Render HUD + UI overlays.

## Interpolation

- Chunk deltas update entity positions and store previous state.
- Interpolation is per-entity, not whole-world snapshots.

## Notes

- Viewport culling is based on view radius and chunk bounds.
- Visual hit flashes are handled by client-only `lastHitAt` markers.

Viewport culling is currently minimal; the reconstruction draws all cached entities.
