export class Camera {
  x = 0;
  y = 0;
  zoom = 1.1; // Start at 1.1 as per docs
  targetX = 0;
  targetY = 0;

  // Configuration
  private readonly lerpFactor = 0.1;
  private readonly minZoom = 0.75;
  private readonly maxZoom = 2.0;
  private readonly defaultZoom = 1.1;
  private readonly pixelsPerUnit = 16;

  // Snapping
  private firstFollow = true;

  follow(targetX: number, targetY: number): void {
    this.targetX = targetX;
    this.targetY = targetY;

    if (this.firstFollow) {
      this.x = targetX;
      this.y = targetY;
      this.firstFollow = false;
    }
  }

  update(): void {
    // Smooth lerp toward target
    this.x += (this.targetX - this.x) * this.lerpFactor;
    this.y += (this.targetY - this.y) * this.lerpFactor;
  }

  adjustZoom(delta: number): void {
    this.zoom = Math.max(
      this.minZoom,
      Math.min(this.maxZoom, this.zoom + delta)
    );
  }

  resetZoom(): void {
    this.zoom = this.defaultZoom;
  }

  // Convert world units to screen pixels
  worldToScreen(worldX: number, worldY: number, canvasWidth: number, canvasHeight: number): { x: number; y: number } {
    const scale = this.pixelsPerUnit * this.zoom;
    return {
      x: (worldX - this.x) * scale + canvasWidth / 2,
      y: (worldY - this.y) * scale + canvasHeight / 2,
    };
  }

  // Convert screen pixels to world units
  screenToWorld(screenX: number, screenY: number, canvasWidth: number, canvasHeight: number): { x: number; y: number } {
    const scale = this.pixelsPerUnit * this.zoom;
    return {
      x: (screenX - canvasWidth / 2) / scale + this.x,
      y: (screenY - canvasHeight / 2) / scale + this.y,
    };
  }

  getScale(): number {
    return this.pixelsPerUnit * this.zoom;
  }

  reset(): void {
    this.firstFollow = true;
    this.zoom = this.defaultZoom;
  }
}
