import type { JoystickState, PointerState } from '../types';

export type JoystickHandler = (dx: number, dy: number) => void;
export type GestureHandler = (type: 'tap' | 'longpress', x: number, y: number) => void;

const LONG_PRESS_THRESHOLD = 250; // ms
const JOYSTICK_MAX_RADIUS = 50; // pixels

export class TouchManager {
  private joystickState: JoystickState = {
    active: false,
    touchId: null,
    centerX: 0,
    centerY: 0,
    dx: 0,
    dy: 0,
  };

  private pointerState: PointerState = {
    x: 0,
    y: 0,
    down: false,
    startTime: 0,
    longPressTriggered: false,
    touchId: null,
  };

  private joystickEl: HTMLElement;
  private joystickZone: HTMLElement;
  private actionZone: HTMLElement;

  private joystickHandlers: Set<JoystickHandler> = new Set();
  private gestureHandlers: Set<GestureHandler> = new Set();
  private longPressTimer: number | null = null;

  constructor(
    joystickEl: HTMLElement,
    joystickZone: HTMLElement,
    actionZone: HTMLElement
  ) {
    this.joystickEl = joystickEl;
    this.joystickZone = joystickZone;
    this.actionZone = actionZone;

    this.setupListeners();
  }

  private setupListeners(): void {
    // Joystick zone (left side)
    this.joystickZone.addEventListener('touchstart', this.onJoystickStart, { passive: false });
    this.joystickZone.addEventListener('touchmove', this.onJoystickMove, { passive: false });
    this.joystickZone.addEventListener('touchend', this.onJoystickEnd, { passive: false });
    this.joystickZone.addEventListener('touchcancel', this.onJoystickEnd, { passive: false });

    // Action zone (right side)
    this.actionZone.addEventListener('touchstart', this.onActionStart, { passive: false });
    this.actionZone.addEventListener('touchmove', this.onActionMove, { passive: false });
    this.actionZone.addEventListener('touchend', this.onActionEnd, { passive: false });
    this.actionZone.addEventListener('touchcancel', this.onActionEnd, { passive: false });
  }

  private onJoystickStart = (e: TouchEvent): void => {
    e.preventDefault();

    if (this.joystickState.active) return;

    const touch = e.changedTouches[0];
    this.joystickState.active = true;
    this.joystickState.touchId = touch.identifier;
    this.joystickState.centerX = touch.clientX;
    this.joystickState.centerY = touch.clientY;
    this.joystickState.dx = 0;
    this.joystickState.dy = 0;

    // Position the joystick element
    this.updateJoystickVisual();
    this.joystickEl.classList.remove('hidden');
  };

  private onJoystickMove = (e: TouchEvent): void => {
    e.preventDefault();

    if (!this.joystickState.active) return;

    const touch = this.findTouch(e.changedTouches, this.joystickState.touchId);
    if (!touch) return;

    // Calculate offset from center
    let dx = touch.clientX - this.joystickState.centerX;
    let dy = touch.clientY - this.joystickState.centerY;

    // Clamp to max radius
    const distance = Math.sqrt(dx * dx + dy * dy);
    if (distance > JOYSTICK_MAX_RADIUS) {
      const ratio = JOYSTICK_MAX_RADIUS / distance;
      dx *= ratio;
      dy *= ratio;
    }

    // Normalize to [-1, 1]
    this.joystickState.dx = dx / JOYSTICK_MAX_RADIUS;
    this.joystickState.dy = dy / JOYSTICK_MAX_RADIUS;

    this.updateJoystickVisual();
    this.notifyJoystick();
  };

  private onJoystickEnd = (e: TouchEvent): void => {
    e.preventDefault();

    const touch = this.findTouch(e.changedTouches, this.joystickState.touchId);
    if (!touch) return;

    this.joystickState.active = false;
    this.joystickState.touchId = null;
    this.joystickState.dx = 0;
    this.joystickState.dy = 0;

    this.joystickEl.classList.add('hidden');
    this.notifyJoystick();
  };

  private onActionStart = (e: TouchEvent): void => {
    e.preventDefault();

    if (this.pointerState.down) return;

    const touch = e.changedTouches[0];
    this.pointerState.down = true;
    this.pointerState.touchId = touch.identifier;
    this.pointerState.x = touch.clientX;
    this.pointerState.y = touch.clientY;
    this.pointerState.startTime = Date.now();
    this.pointerState.longPressTriggered = false;

    // Start long-press timer
    this.longPressTimer = window.setTimeout(() => {
      if (this.pointerState.down && !this.pointerState.longPressTriggered) {
        this.pointerState.longPressTriggered = true;
        this.notifyGesture('longpress', this.pointerState.x, this.pointerState.y);
      }
    }, LONG_PRESS_THRESHOLD);
  };

  private onActionMove = (e: TouchEvent): void => {
    e.preventDefault();

    if (!this.pointerState.down) return;

    const touch = this.findTouch(e.changedTouches, this.pointerState.touchId);
    if (!touch) return;

    // Update position (for aim)
    this.pointerState.x = touch.clientX;
    this.pointerState.y = touch.clientY;
  };

  private onActionEnd = (e: TouchEvent): void => {
    e.preventDefault();

    const touch = this.findTouch(e.changedTouches, this.pointerState.touchId);
    if (!touch) return;

    // Clear long-press timer
    if (this.longPressTimer) {
      clearTimeout(this.longPressTimer);
      this.longPressTimer = null;
    }

    // If not already triggered as long-press, it's a tap
    if (this.pointerState.down && !this.pointerState.longPressTriggered) {
      const duration = Date.now() - this.pointerState.startTime;
      if (duration < LONG_PRESS_THRESHOLD) {
        this.notifyGesture('tap', this.pointerState.x, this.pointerState.y);
      }
    }

    this.pointerState.down = false;
    this.pointerState.touchId = null;
  };

  private findTouch(touches: TouchList, id: number | null): Touch | null {
    if (id === null) return null;
    for (let i = 0; i < touches.length; i++) {
      if (touches[i].identifier === id) {
        return touches[i];
      }
    }
    return null;
  }

  private updateJoystickVisual(): void {
    const scale = JOYSTICK_MAX_RADIUS;
    const knobX = this.joystickState.dx * scale;
    const knobY = this.joystickState.dy * scale;

    this.joystickEl.style.left = `${this.joystickState.centerX}px`;
    this.joystickEl.style.top = `${this.joystickState.centerY}px`;

    const knob = this.joystickEl.querySelector('.joystick-knob') as HTMLElement;
    if (knob) {
      knob.style.transform = `translate(calc(-50% + ${knobX}px), calc(-50% + ${knobY}px))`;
    }
  }

  private notifyJoystick(): void {
    this.joystickHandlers.forEach((h) => h(this.joystickState.dx, this.joystickState.dy));
  }

  private notifyGesture(type: 'tap' | 'longpress', x: number, y: number): void {
    this.gestureHandlers.forEach((h) => h(type, x, y));
  }

  getJoystickVector(): { dx: number; dy: number } {
    if (!this.joystickState.active) return { dx: 0, dy: 0 };
    return { dx: this.joystickState.dx, dy: this.joystickState.dy };
  }

  isJoystickActive(): boolean {
    return this.joystickState.active;
  }

  onJoystick(handler: JoystickHandler): () => void {
    this.joystickHandlers.add(handler);
    return () => this.joystickHandlers.delete(handler);
  }

  onGesture(handler: GestureHandler): () => void {
    this.gestureHandlers.add(handler);
    return () => this.gestureHandlers.delete(handler);
  }

  getPointerPosition(): { x: number; y: number } | null {
    if (!this.pointerState.down) return null;
    return { x: this.pointerState.x, y: this.pointerState.y };
  }

  destroy(): void {
    if (this.longPressTimer) {
      clearTimeout(this.longPressTimer);
    }
  }
}
