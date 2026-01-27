export interface InputState {
  dx: number;
  dy: number;
  attack: boolean;
  interact: boolean;
}

export class InputManager {
  private keys = new Set<string>();
  private pointerDown = false;
  private pressStartMs = 0;
  private longPressTriggered = false;
  private attackQueued = false;
  private interactQueued = false;
  private joystickId: number | null = null;
  private joystickOrigin: { x: number; y: number } | null = null;
  private joystickVector = { x: 0, y: 0 };
  private readonly longPressMs = 275;

  keyQueue: string[] = [];
  mouseX = 0;
  mouseY = 0;

  attach(canvas: HTMLCanvasElement): void {
    window.addEventListener("keydown", (event) => this.handleKeyDown(event));
    window.addEventListener("keyup", (event) => this.handleKeyUp(event));
    canvas.addEventListener("mousedown", (event) => this.handlePointerDown(event.clientX, event.clientY));
    canvas.addEventListener("mousemove", (event) => this.handlePointerMove(event.clientX, event.clientY));
    window.addEventListener("mouseup", () => this.handlePointerUp());
    canvas.addEventListener("touchstart", (event) => this.handleTouchStart(event), { passive: false });
    canvas.addEventListener("touchmove", (event) => this.handleTouchMove(event), { passive: false });
    canvas.addEventListener("touchend", (event) => this.handleTouchEnd(event));
  }

  update(): void {
    if (this.pointerDown && !this.longPressTriggered) {
      const elapsed = Date.now() - this.pressStartMs;
      if (elapsed >= this.longPressMs) {
        this.longPressTriggered = true;
        this.interactQueued = true;
      }
    }
  }

  consumeState(): InputState {
    const [dx, dy] = this.computeMovement();
    const attack = this.attackQueued;
    const interact = this.interactQueued;
    this.attackQueued = false;
    this.interactQueued = false;
    return { dx, dy, attack, interact };
  }

  cancelActions(): void {
    this.attackQueued = false;
    this.interactQueued = false;
  }

  private handleKeyDown(event: KeyboardEvent): void {
    this.keys.add(event.key.toLowerCase());
    if (event.key.toLowerCase() === "e") {
      this.interactQueued = true;
    }
    if (event.key.length === 1) {
      this.keyQueue.push(event.key);
    }
    if (event.key === "Backspace") {
      this.keyQueue.push("\b");
    }
  }

  private handleKeyUp(event: KeyboardEvent): void {
    this.keys.delete(event.key.toLowerCase());
  }

  private computeMovement(): [number, number] {
    let dx = 0;
    let dy = 0;
    if (this.keys.has("w")) dy -= 1;
    if (this.keys.has("s")) dy += 1;
    if (this.keys.has("a")) dx -= 1;
    if (this.keys.has("d")) dx += 1;

    dx += this.joystickVector.x;
    dy += this.joystickVector.y;

    const len = Math.hypot(dx, dy);
    if (len > 1) {
      dx /= len;
      dy /= len;
    }
    return [dx, dy];
  }

  private handlePointerDown(x: number, y: number): void {
    this.pointerDown = true;
    this.pressStartMs = Date.now();
    this.longPressTriggered = false;
    this.mouseX = x;
    this.mouseY = y;
  }

  private handlePointerMove(x: number, y: number): void {
    this.mouseX = x;
    this.mouseY = y;
  }

  private handlePointerUp(): void {
    if (!this.pointerDown) {
      return;
    }
    const elapsed = Date.now() - this.pressStartMs;
    if (elapsed < this.longPressMs && !this.longPressTriggered) {
      this.attackQueued = true;
    }
    this.pointerDown = false;
    this.longPressTriggered = false;
  }

  private handleTouchStart(event: TouchEvent): void {
    event.preventDefault();
    for (const touch of Array.from(event.changedTouches)) {
      if (touch.clientX < window.innerWidth * 0.5 && this.joystickId === null) {
        this.joystickId = touch.identifier;
        this.joystickOrigin = { x: touch.clientX, y: touch.clientY };
        this.joystickVector = { x: 0, y: 0 };
      } else if (!this.pointerDown) {
        this.handlePointerDown(touch.clientX, touch.clientY);
      }
    }
  }

  private handleTouchMove(event: TouchEvent): void {
    event.preventDefault();
    for (const touch of Array.from(event.touches)) {
      if (this.joystickId === touch.identifier && this.joystickOrigin) {
        const dx = touch.clientX - this.joystickOrigin.x;
        const dy = touch.clientY - this.joystickOrigin.y;
        const maxRadius = 50;
        const len = Math.hypot(dx, dy);
        const scale = len > maxRadius ? maxRadius / len : 1;
        this.joystickVector = { x: (dx * scale) / maxRadius, y: (dy * scale) / maxRadius };
      }
    }
  }

  private handleTouchEnd(event: TouchEvent): void {
    for (const touch of Array.from(event.changedTouches)) {
      if (this.joystickId === touch.identifier) {
        this.joystickId = null;
        this.joystickOrigin = null;
        this.joystickVector = { x: 0, y: 0 };
      } else if (this.pointerDown) {
        this.handlePointerUp();
      }
    }
  }
}
