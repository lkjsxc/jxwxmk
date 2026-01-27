import { InputState } from "./state";
import { bindKeyboard, computeMovement } from "./keyboard";
import { bindTouch } from "./touch";

export class InputManager {
  state: InputState;
  private keyMap: Record<string, boolean>;
  private lastAttackAt = 0;
  private lastInteractAt = 0;
  private attackCooldown = 500;
  private interactCooldown = 400;
  private longPressMs = 280;

  constructor(private canvas: HTMLCanvasElement) {
    this.state = {
      dx: 0,
      dy: 0,
      attack: false,
      interact: false,
      mouseX: 0,
      mouseY: 0,
      isPointerDown: false,
      pressStartMs: null,
      keyQueue: [],
      touchActive: false,
    };
    this.keyMap = bindKeyboard(this.state, canvas);
    bindTouch(this.state, canvas);
  }

  update() {
    const move = computeMovement(this.keyMap);
    if (move.dx !== 0 || move.dy !== 0) {
      this.state.dx = move.dx;
      this.state.dy = move.dy;
    } else if (!this.state.touchActive) {
      this.state.dx = 0;
      this.state.dy = 0;
    }
    const now = performance.now();
    this.state.attack = false;
    this.state.interact = false;

    if (this.keyMap["e"] && now - this.lastInteractAt >= this.interactCooldown) {
      this.state.interact = true;
      this.lastInteractAt = now;
    }

    if (this.state.isPointerDown && this.state.pressStartMs !== null) {
      const held = now - this.state.pressStartMs;
      if (held >= this.longPressMs) {
        if (now - this.lastInteractAt >= this.interactCooldown) {
          this.state.interact = true;
          this.lastInteractAt = now;
        }
      } else if (now - this.lastAttackAt >= this.attackCooldown) {
        this.state.attack = true;
        this.lastAttackAt = now;
      }
    }
  }

  consumeKeyQueue(): string[] {
    const queue = [...this.state.keyQueue];
    this.state.keyQueue.length = 0;
    return queue;
  }
}
