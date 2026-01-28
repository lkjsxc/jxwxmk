import type { NetworkManager } from '../core/network';
import type { Camera } from '../render/camera';
import type { KeyboardManager } from './keyboard';
import type { TouchManager } from './touch';
import type { MouseManager } from './mouse';

const INPUT_INTERVAL = 50; // ms (20Hz)
const ATTACK_COOLDOWN = 500; // ms
const INTERACT_COOLDOWN = 400; // ms

export class InputManager {
  private interval: number | null = null;
  private lastAttackTime = 0;
  private lastInteractTime = 0;

  constructor(
    private network: NetworkManager,
    private camera: Camera,
    private keyboard: KeyboardManager,
    private touch: TouchManager,
    private mouse: MouseManager,
    private canvas: HTMLCanvasElement,
    private isMenuOpen: () => boolean
  ) {
    this.setupInputs();
    this.startInputLoop();
  }

  private setupInputs(): void {
    // Touch gestures
    this.touch.onGesture((type, x, y) => {
      if (this.isMenuOpen()) return;

      const worldPos = this.camera.screenToWorld(
        x,
        y,
        this.canvas.width,
        this.canvas.height
      );

      if (type === 'tap') {
        this.sendAttack(worldPos.x, worldPos.y);
      } else if (type === 'longpress') {
        this.sendInteract(worldPos.x, worldPos.y);
      }
    });

    // Mouse handling
    this.mouse.onMouse((type, x, y) => {
      if (this.isMenuOpen()) return;

      if (type === 'up') {
        const worldPos = this.camera.screenToWorld(
          x,
          y,
          this.canvas.width,
          this.canvas.height
        );

        // For mouse, we use simple click = attack
        // Could add right-click for interact in the future
        this.sendAttack(worldPos.x, worldPos.y);
      }
    });

    // Mouse wheel for zoom
    this.mouse.onWheel((delta) => {
      this.camera.adjustZoom(delta);
    });
  }

  private sendAttack(targetX: number, targetY: number): void {
    const now = Date.now();
    if (now - this.lastAttackTime < ATTACK_COOLDOWN) return;
    this.lastAttackTime = now;

    this.network.send({
      type: 'input',
      data: {
        dx: 0,
        dy: 0,
        attack: true,
        interact: false,
        aim: { x: targetX, y: targetY },
      },
    });
  }

  private sendInteract(targetX: number, targetY: number): void {
    const now = Date.now();
    if (now - this.lastInteractTime < INTERACT_COOLDOWN) return;
    this.lastInteractTime = now;

    this.network.send({
      type: 'input',
      data: {
        dx: 0,
        dy: 0,
        attack: false,
        interact: true,
        aim: { x: targetX, y: targetY },
      },
    });
  }

  private startInputLoop(): void {
    this.interval = window.setInterval(() => {
      if (this.isMenuOpen()) return;

      // Get movement from keyboard or joystick
      const keyboardVec = this.keyboard.getMovementVector();
      const joystickVec = this.touch.getJoystickVector();

      // Prefer joystick if active, otherwise keyboard
      let dx = 0;
      let dy = 0;
      if (this.touch.isJoystickActive()) {
        dx = joystickVec.dx;
        dy = joystickVec.dy;
      } else if (keyboardVec.dx !== 0 || keyboardVec.dy !== 0) {
        dx = keyboardVec.dx;
        dy = keyboardVec.dy;
      }

      // Only send if moving
      if (dx !== 0 || dy !== 0) {
        this.network.send({
          type: 'input',
          data: {
            dx,
            dy,
            attack: false,
            interact: false,
            aim: null,
          },
        });
      }
    }, INPUT_INTERVAL);
  }

  destroy(): void {
    if (this.interval) {
      clearInterval(this.interval);
    }
  }
}
