export class Camera {
  x = 0;
  y = 0;
  zoom = 1;
  pivotX = 0;
  pivotY = 0;
  private targetX = 0;
  private targetY = 0;
  private firstSnap = true;

  follow(targetX: number, targetY: number) {
    this.targetX = targetX;
    this.targetY = targetY;
  }

  adjustZoom(delta: number) {
    this.zoom = Math.min(2.0, Math.max(0.5, this.zoom + delta));
  }

  setViewport(width: number, height: number) {
    this.pivotX = width / 2;
    this.pivotY = height / 2;
  }

  update() {
    if (this.firstSnap) {
      this.x = this.targetX;
      this.y = this.targetY;
      this.firstSnap = false;
      return;
    }
    this.x += (this.targetX - this.x) * 0.1;
    this.y += (this.targetY - this.y) * 0.1;
  }

  worldToScreen(x: number, y: number) {
    return {
      x: (x - this.x) * this.zoom + this.pivotX,
      y: (y - this.y) * this.zoom + this.pivotY,
    };
  }
}
