import { connection } from './connection';
import type { InputMessage } from './types';

const INPUT_INTERVAL_MS = 50; // ~20Hz input sampling
const ATTACK_COOLDOWN_MS = 500;
const INTERACT_COOLDOWN_MS = 400;
const LONG_PRESS_MS = 275; // ~250-300ms

interface PointerState {
  id: number;
  x: number;
  y: number;
  startX: number;
  startY: number;
  startTime: number;
  isDown: boolean;
  isLongPress: boolean;
}

export class InputManager {
  // Movement
  dx = 0;
  dy = 0;

  // Actions
  attack = false;
  interact = false;

  // Pointer state
  mouseX = 0;
  mouseY = 0;
  private pointers = new Map<number, PointerState>();
  private isPointerDown = false;
  private pressStartTime = 0;

  // Cooldowns
  private lastAttackTime = 0;
  private lastInteractTime = 0;

  // Joystick (touch)
  private joystickActive = false;
  private joystickCenterX = 0;
  private joystickCenterY = 0;
  private joystickCurrentX = 0;
  private joystickCurrentY = 0;
  private readonly joystickMaxRadius = 50;

  // Input loop
  private intervalId: number | null = null;

  // World-space aim point
  private aimX = 0;
  private aimY = 0;

  // Camera reference for screen-to-world conversion
  private cameraX = 0;
  private cameraY = 0;
  private zoom = 1;

  // UI State tracking
  private modalsOpen = new Set<string>();

  constructor() {
    this.setupKeyboard();
    this.setupMouse();
    this.setupTouch();
    this.setupVisibilityHandling();
  }

  setCamera(x: number, y: number, zoom: number): void {
    this.cameraX = x;
    this.cameraY = y;
    this.zoom = zoom;
  }

  registerModal(name: string): void {
    this.modalsOpen.add(name);
  }

  unregisterModal(name: string): void {
    this.modalsOpen.delete(name);
  }

  start(): void {
    if (this.intervalId !== null) return;
    this.intervalId = window.setInterval(() => this.tick(), INPUT_INTERVAL_MS);
  }

  stop(): void {
    if (this.intervalId !== null) {
      clearInterval(this.intervalId);
      this.intervalId = null;
    }
  }

  private tick(): void {
    // Update joystick movement if active
    if (this.joystickActive) {
      const dx = this.joystickCurrentX - this.joystickCenterX;
      const dy = this.joystickCurrentY - this.joystickCenterY;
      const dist = Math.sqrt(dx * dx + dy * dy);

      if (dist > 5) {
        const normalized = Math.min(dist, this.joystickMaxRadius) / this.joystickMaxRadius;
        this.dx = (dx / dist) * normalized;
        this.dy = (dy / dist) * normalized;
      } else {
        this.dx = 0;
        this.dy = 0;
      }
    }

    // Check for long-press interact (only if not clicking on UI)
    if (this.isPointerDown && !this.interact && !this.attack) {
      const pressDuration = Date.now() - this.pressStartTime;
      if (pressDuration >= LONG_PRESS_MS && this.canInteract()) {
        this.interact = true;
        this.updateAim();
      }
    }

    // ALWAYS send input message to keep connection alive (server timeout is 10s)
    // This also ensures the server knows our current movement state
    const msg: InputMessage = {
      type: 'input',
      data: {
        dx: this.dx,
        dy: this.dy,
        attack: this.attack,
        interact: this.interact,
      },
    };

    // Include aim when attacking or interacting
    if (this.attack || this.interact) {
      msg.data.aim = { x: this.aimX, y: this.aimY };
    }

    connection.send(msg);

    // Reset one-shot actions
    this.attack = false;
    this.interact = false;
  }

  private canAttack(): boolean {
    return Date.now() - this.lastAttackTime >= ATTACK_COOLDOWN_MS;
  }

  private canInteract(): boolean {
    return Date.now() - this.lastInteractTime >= INTERACT_COOLDOWN_MS;
  }

  private updateAim(): void {
    // Convert screen coordinates to world coordinates
    const canvas = document.getElementById('game-canvas') as HTMLCanvasElement;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const centerX = rect.width / 2;
    const centerY = rect.height / 2;

    const screenDx = this.mouseX - centerX;
    const screenDy = this.mouseY - centerY;

    const PPU = 16; // pixels per world unit at zoom 1.0
    this.aimX = this.cameraX + screenDx / (PPU * this.zoom);
    this.aimY = this.cameraY + screenDy / (PPU * this.zoom);
  }

  // ===== Keyboard Handling =====

  private keys = new Set<string>();

  private setupKeyboard(): void {
    window.addEventListener('keydown', (e) => {
      // Don't handle keys if typing in an input
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
        return;
      }

      this.keys.add(e.key.toLowerCase());
      this.updateMovementFromKeys();

      // Hotbar slots 1-7
      if (e.key >= '1' && e.key <= '7') {
        const slot = parseInt(e.key) - 1;
        connection.send({ type: 'slot', data: { slot } });
      }

      // Interact key (E)
      if (e.key.toLowerCase() === 'e' && this.canInteract()) {
        this.interact = true;
        this.updateAim();
        this.lastInteractTime = Date.now();
      }
    });

    window.addEventListener('keyup', (e) => {
      this.keys.delete(e.key.toLowerCase());
      this.updateMovementFromKeys();
    });
  }

  private updateMovementFromKeys(): void {
    if (this.joystickActive) return; // Joystick takes precedence

    let dx = 0;
    let dy = 0;

    if (this.keys.has('w') || this.keys.has('arrowup')) dy -= 1;
    if (this.keys.has('s') || this.keys.has('arrowdown')) dy += 1;
    if (this.keys.has('a') || this.keys.has('arrowleft')) dx -= 1;
    if (this.keys.has('d') || this.keys.has('arrowright')) dx += 1;

    // Normalize diagonal movement
    if (dx !== 0 && dy !== 0) {
      const len = Math.sqrt(dx * dx + dy * dy);
      dx /= len;
      dy /= len;
    }

    this.dx = dx;
    this.dy = dy;
  }

  // ===== Mouse Handling =====

  private setupMouse(): void {
    window.addEventListener('mousemove', (e) => {
      this.mouseX = e.clientX;
      this.mouseY = e.clientY;
    });

    window.addEventListener('mousedown', (e) => {
      if (e.button !== 0) return; // Only left click

      // Check if clicking on UI or modal
      if (this.isClickOnUI(e.clientX, e.clientY)) return;

      this.isPointerDown = true;
      this.pressStartTime = Date.now();
      this.mouseX = e.clientX;
      this.mouseY = e.clientY;

      // Tap = attack
      if (this.canAttack()) {
        this.attack = true;
        this.updateAim();
        this.lastAttackTime = Date.now();
      }
    });

    window.addEventListener('mouseup', () => {
      this.isPointerDown = false;
    });

    // Zoom with mouse wheel
    window.addEventListener('wheel', (e) => {
      const delta = e.deltaY > 0 ? -0.1 : 0.1;
      this.emitZoom?.(delta);
    }, { passive: true });

    // Prevent context menu
    window.addEventListener('contextmenu', (e) => {
      e.preventDefault();
    });
  }

  private isClickOnUI(x: number, y: number): boolean {
    // Check if any modal is open
    if (this.modalsOpen.size > 0) return true;

    // Check if click is inside UI overlay elements
    const target = document.elementFromPoint(x, y);
    if (!target) return false;

    // Check for modal or UI elements
    if (target.closest('.modal')) return true;
    if (target.closest('.overlay')) return true;
    if (target.closest('#login-screen')) return true;

    return false;
  }

  // ===== Touch Handling =====

  private setupTouch(): void {
    const canvas = document.getElementById('game-canvas');
    if (!canvas) return;

    canvas.addEventListener('touchstart', (e) => {
      e.preventDefault();

      for (let i = 0; i < e.changedTouches.length; i++) {
        const touch = e.changedTouches[i];
        const x = touch.clientX;
        const y = touch.clientY;

        const screenWidth = window.innerWidth;

        // Left side = joystick
        if (x < screenWidth / 2 && !this.joystickActive) {
          this.joystickActive = true;
          this.joystickCenterX = x;
          this.joystickCenterY = y;
          this.joystickCurrentX = x;
          this.joystickCurrentY = y;
        }

        // Right side = actions
        if (x >= screenWidth / 2) {
          this.isPointerDown = true;
          this.pressStartTime = Date.now();
          this.mouseX = x;
          this.mouseY = y;

          // Tap = attack
          if (this.canAttack()) {
            this.attack = true;
            this.updateAim();
            this.lastAttackTime = Date.now();
          }
        }

        this.pointers.set(touch.identifier, {
          id: touch.identifier,
          x,
          y,
          startX: x,
          startY: y,
          startTime: Date.now(),
          isDown: true,
          isLongPress: false,
        });
      }
    }, { passive: false });

    canvas.addEventListener('touchmove', (e) => {
      e.preventDefault();

      for (let i = 0; i < e.changedTouches.length; i++) {
        const touch = e.changedTouches[i];
        const pointer = this.pointers.get(touch.identifier);
        if (!pointer) continue;

        pointer.x = touch.clientX;
        pointer.y = touch.clientY;

        // Update joystick with clamping
        if (this.joystickActive && pointer.startX < window.innerWidth / 2) {
          // Clamp joystick position to max radius
          const dx = touch.clientX - this.joystickCenterX;
          const dy = touch.clientY - this.joystickCenterY;
          const dist = Math.sqrt(dx * dx + dy * dy);

          if (dist > this.joystickMaxRadius) {
            const angle = Math.atan2(dy, dx);
            this.joystickCurrentX = this.joystickCenterX + Math.cos(angle) * this.joystickMaxRadius;
            this.joystickCurrentY = this.joystickCenterY + Math.sin(angle) * this.joystickMaxRadius;
          } else {
            this.joystickCurrentX = touch.clientX;
            this.joystickCurrentY = touch.clientY;
          }
        }

        // Update aim for right side
        if (pointer.x >= window.innerWidth / 2) {
          this.mouseX = touch.clientX;
          this.mouseY = touch.clientY;
        }
      }
    }, { passive: false });

    canvas.addEventListener('touchend', (e) => {
      e.preventDefault();

      for (let i = 0; i < e.changedTouches.length; i++) {
        const touch = e.changedTouches[i];
        const pointer = this.pointers.get(touch.identifier);
        if (!pointer) continue;

        // Release joystick if this was the joystick finger
        if (this.joystickActive && pointer.startX < window.innerWidth / 2) {
          this.joystickActive = false;
          this.dx = 0;
          this.dy = 0;
        }

        // Release actions if this was the action finger
        if (pointer.startX >= window.innerWidth / 2) {
          this.isPointerDown = false;
        }

        this.pointers.delete(touch.identifier);
      }
    });

    canvas.addEventListener('touchcancel', (e) => {
      this.pointers.clear();
      this.joystickActive = false;
      this.isPointerDown = false;
      this.dx = 0;
      this.dy = 0;
    });
  }

  // ===== Visibility Handling =====

  private setupVisibilityHandling(): void {
    // Handle tab visibility change - ensure input continues when tab is not focused
    document.addEventListener('visibilitychange', () => {
      if (document.visibilityState === 'visible') {
        // Resume input loop if it was stopped
        if (connection.isConnected && this.intervalId === null) {
          this.start();
        }
      }
    });

    // Handle window focus/blur
    window.addEventListener('blur', () => {
      // Clear keys when window loses focus to prevent stuck keys
      this.keys.clear();
      this.dx = 0;
      this.dy = 0;
    });
  }

  // Zoom callback
  emitZoom: ((delta: number) => void) | null = null;
}

export const input = new InputManager();
