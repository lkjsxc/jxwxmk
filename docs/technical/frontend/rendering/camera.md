# Camera System

The camera determines the visible viewport of the game world.

## Mechanics

### Tracking (Follow)
-   **Target**: The local player's (X, Y) coordinates.
-   **Smoothing**: Linear interpolation (Lerp) with a factor of ~0.1 to avoid jitter.
-   **Clamping**: Camera cannot view outside the world bounds (0,0 to WorldW, WorldH).

### Zoom
-   **Controls**: Mouse Wheel (Desktop) or Pinch Gesture (Mobile).
-   **Range**: 0.5x (Wide view) to 2.0x (Close up).
-   **Implementation**: Canvas `scale()` transform applied before drawing world layer.

## Viewport Culling
Only entities within `CameraX - Padding` to `CameraX + Width + Padding` (and Y equiv) are rendered to save performance.
