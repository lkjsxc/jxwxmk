import type { Camera } from '../render/camera';

export type MouseHandler = (type: 'down' | 'up' | 'move', x: number, y: number) => void;
export type WheelHandler = (delta: number) => void;

const LONG_PRESS_THRESHOLD = 250; // ms

export class MouseManager {
  private x = 0;
  private y = 0;
  private down = false;
  private startTime = 0;
  private longPressTriggered = false;
  private longPressTimer: number | null = null;

  private handlers: Set<MouseHandler> = new Set();
  private wheelHandlers: Set<WheelHandler> = new Set();

  constructor(
    private canvas: HTMLCanvasElement,
    private camera: Camera
  ) {
    this.setupListeners();
  }

  private setupListeners(): void {
    this.canvas.addEventListener('mousedown', this.onMouseDown);
    this.canvas.addEventListener('mouseup', this.onMouseUp);
    this.canvas.addEventListener('mousemove', this.onMouseMove);
    this.canvas.addEventListener('wheel', this.handleWheel, { passive: false });

    // Prevent context menu on right-click
    this.canvas.addEventListener('contextmenu', (e) => e.preventDefault());
  }

  private onMouseDown = (e: MouseEvent): void => {
    this.down = true;
    this.x = e.clientX;
    this.y = e.clientY;
    this.startTime = Date.now();
    this.longPressTriggered = false;

    this.handlers.forEach((h) => h('down', this.x, this.y));

    // Start long-press timer
    this.longPressTimer = window.setTimeout(() => {
      if (this.down && !this.longPressTriggered) {
        this.longPressTriggered = true;
        // Emit as long-press
        this.handlers.forEach((h) => h('up', this.x, this.y));
      }
    }, LONG_PRESS_THRESHOLD);
  };

  private onMouseUp = (e: MouseEvent): void => {
    if (!this.down) return;

    if (this.longPressTimer) {
      clearTimeout(this.longPressTimer);
      this.longPressTimer = null;
    }

    this.down = false;
    this.handlers.forEach((h) => h('up', e.clientX, e.clientY));
  };

  private onMouseMove = (e: MouseEvent): void => {
    this.x = e.clientX;
    this.y = e.clientY;
    this.handlers.forEach((h) => h('move', this.x, this.y));
  };

  private handleWheel = (e: WheelEvent): void => {
    e.preventDefault();
    const delta = e.deltaY > 0 ? -0.1 : 0.1;
    this.wheelHandlers.forEach((h) => h(delta));
  };

  getPosition(): { x: number; y: number } {
    return { x: this.x, y: this.y };
  }

  isDown(): boolean {
    return this.down;
  }

  getWorldPosition(canvasWidth: number, canvasHeight: number): { x: number; y: number } {
    return this.camera.screenToWorld(this.x, this.y, canvasWidth, canvasHeight);
  }

  onMouse(handler: MouseHandler): () => void {
    this.handlers.add(handler);
    return () => this.handlers.delete(handler);
  }

  onWheel(handler: WheelHandler): () => void {
    this.wheelHandlers.add(handler);
    return () => this.wheelHandlers.delete(handler);
  }

  destroy(): void {
    if (this.longPressTimer) {
      clearTimeout(this.longPressTimer);
    }
    this.canvas.removeEventListener('mousedown', this.onMouseDown);
    this.canvas.removeEventListener('mouseup', this.onMouseUp);
    this.canvas.removeEventListener('mousemove', this.onMouseMove);
    this.canvas.removeEventListener('wheel', this.handleWheel);
  }
}
