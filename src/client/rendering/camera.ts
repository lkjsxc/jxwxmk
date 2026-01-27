export class Camera {
  x = 0;
  y = 0;
  zoom = 1;
  private targetX = 0;
  private targetY = 0;
  private initialized = false;

  follow(x: number, y: number): void {
    this.targetX = x;
    this.targetY = y;
    if (!this.initialized) {
      this.x = x;
      this.y = y;
      this.initialized = true;
    }
  }

  update(): void {
    this.x += (this.targetX - this.x) * 0.1;
    this.y += (this.targetY - this.y) * 0.1;
  }

  adjustZoom(delta: number): void {
    this.zoom = Math.max(0.5, Math.min(2.0, this.zoom + delta));
  }
}
