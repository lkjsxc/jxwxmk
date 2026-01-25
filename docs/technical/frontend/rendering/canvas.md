# Canvas Rendering

## Optimization

- **Offscreen Canvas**: Pre-render static assets (trees, rocks) to offscreen canvases to avoid re-drawing paths every frame.
- **Culling**: Only draw entities within `Camera Viewport + Padding`.
- **Layers**:
    1.  **Background**: Terrain (Grid tiles).
    2.  **Ground Items**: Dropped resources.
    3.  **Entities**: Players, Mobs, Walls (Sorted by Y-coordinate for pseudo-3D depth).
    4.  **Overlays**: Health bars, Names.
    5.  **UI**: Hotbar, Chat, Joysticks (Mobile).

## Loop
`requestAnimationFrame` drives the render loop, decoupled from the network tick rate. Interpolation is used for smooth movement.
