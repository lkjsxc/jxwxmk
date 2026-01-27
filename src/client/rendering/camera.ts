export class Camera {
  x = 0;
  y = 0;
  zoom = 1;
  private firstSnap = true;

  follow(targetX: number, targetY: number) {
    if (this.firstSnap) {
      this.x = targetX;
      this.y = targetY;
      this.firstSnap = false;
      return;
    }
    this.x += (targetX - this.x) * 0.1;
    this.y += (targetY - this.y) * 0.1;
  }

  adjustZoom(delta: number) {
    this.zoom = Math.min(2.0, Math.max(0.5, this.zoom + delta));
  }
}
