import type { Entity } from './types';

const PPU = 16; // pixels per world unit at zoom 1.0
const MIN_ZOOM = 0.75;
const MAX_ZOOM = 2.0;
const DEFAULT_ZOOM = 1.1;
const FOLLOW_FACTOR = 0.1;

export class Camera {
  x = 0;
  y = 0;
  zoom = DEFAULT_ZOOM;

  private targetX = 0;
  private targetY = 0;
  private hasInitialTarget = false;

  follow(targetX: number, targetY: number): void {
    this.targetX = targetX;
    this.targetY = targetY;

    // Snap on first target
    if (!this.hasInitialTarget) {
      this.x = targetX;
      this.y = targetY;
      this.hasInitialTarget = true;
    }
  }

  update(): void {
    // Smooth lerp toward target
    this.x += (this.targetX - this.x) * FOLLOW_FACTOR;
    this.y += (this.targetY - this.y) * FOLLOW_FACTOR;
  }

  adjustZoom(delta: number): void {
    this.zoom = Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, this.zoom + delta));
  }

  // Convert world coordinates to screen coordinates
  worldToScreen(worldX: number, worldY: number, canvasWidth: number, canvasHeight: number): { x: number; y: number } {
    const screenX = (worldX - this.x) * PPU * this.zoom + canvasWidth / 2;
    const screenY = (worldY - this.y) * PPU * this.zoom + canvasHeight / 2;
    return { x: screenX, y: screenY };
  }

  // Convert screen coordinates to world coordinates
  screenToWorld(screenX: number, screenY: number, canvasWidth: number, canvasHeight: number): { x: number; y: number } {
    const worldX = (screenX - canvasWidth / 2) / (PPU * this.zoom) + this.x;
    const worldY = (screenY - canvasHeight / 2) / (PPU * this.zoom) + this.y;
    return { x: worldX, y: worldY };
  }

  // Get viewport bounds in world coordinates
  getViewport(canvasWidth: number, canvasHeight: number): { minX: number; minY: number; maxX: number; maxY: number } {
    const halfViewWidth = (canvasWidth / 2) / (PPU * this.zoom);
    const halfViewHeight = (canvasHeight / 2) / (PPU * this.zoom);

    return {
      minX: this.x - halfViewWidth,
      minY: this.y - halfViewHeight,
      maxX: this.x + halfViewWidth,
      maxY: this.y + halfViewHeight,
    };
  }

  reset(): void {
    this.x = 0;
    this.y = 0;
    this.zoom = DEFAULT_ZOOM;
    this.hasInitialTarget = false;
  }
}

export const camera = new Camera();
