export type KeyHandler = (key: string, pressed: boolean) => void;

export class KeyboardManager {
  private keys = new Set<string>();
  private handlers: Set<KeyHandler> = new Set();

  constructor() {
    this.setupListeners();
  }

  private setupListeners(): void {
    window.addEventListener('keydown', (e) => {
      if (e.repeat) return;
      this.keys.add(e.key);
      this.handlers.forEach((h) => h(e.key, true));
    });

    window.addEventListener('keyup', (e) => {
      this.keys.delete(e.key);
      this.handlers.forEach((h) => h(e.key, false));
    });
  }

  isPressed(key: string): boolean {
    return this.keys.has(key);
  }

  getMovementVector(): { dx: number; dy: number } {
    let dx = 0;
    let dy = 0;

    if (this.isPressed('w') || this.isPressed('W') || this.isPressed('ArrowUp')) dy -= 1;
    if (this.isPressed('s') || this.isPressed('S') || this.isPressed('ArrowDown')) dy += 1;
    if (this.isPressed('a') || this.isPressed('A') || this.isPressed('ArrowLeft')) dx -= 1;
    if (this.isPressed('d') || this.isPressed('D') || this.isPressed('ArrowRight')) dx += 1;

    // Normalize
    if (dx !== 0 || dy !== 0) {
      const mag = Math.sqrt(dx * dx + dy * dy);
      dx /= mag;
      dy /= mag;
    }

    return { dx, dy };
  }

  onKey(handler: KeyHandler): () => void {
    this.handlers.add(handler);
    return () => this.handlers.delete(handler);
  }

  destroy(): void {
    // Cleanup handled by window event listeners
  }
}
