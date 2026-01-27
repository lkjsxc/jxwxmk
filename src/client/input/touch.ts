import { InputState } from "./state";

interface Joystick {
  id: number | null;
  originX: number;
  originY: number;
  dx: number;
  dy: number;
}

export function bindTouch(input: InputState, canvas: HTMLCanvasElement) {
  const joystick: Joystick = { id: null, originX: 0, originY: 0, dx: 0, dy: 0 };
  let actionTouch: number | null = null;

  const updateMovement = () => {
    input.dx = joystick.dx;
    input.dy = joystick.dy;
  };

  canvas.addEventListener("touchstart", (event) => {
    for (const touch of Array.from(event.changedTouches)) {
      const x = touch.clientX;
      const y = touch.clientY;
      if (x < window.innerWidth * 0.5 && joystick.id === null) {
        joystick.id = touch.identifier;
        joystick.originX = x;
        joystick.originY = y;
        joystick.dx = 0;
        joystick.dy = 0;
        input.touchActive = true;
      } else if (actionTouch === null) {
        actionTouch = touch.identifier;
        input.isPointerDown = true;
        input.pressStartMs = performance.now();
        input.mouseX = x;
        input.mouseY = y;
      }
    }
    updateMovement();
  });

  canvas.addEventListener("touchmove", (event) => {
    for (const touch of Array.from(event.changedTouches)) {
      if (touch.identifier === joystick.id) {
        const dx = touch.clientX - joystick.originX;
        const dy = touch.clientY - joystick.originY;
        const radius = 50;
        const len = Math.hypot(dx, dy);
        joystick.dx = len > 0 ? dx / Math.max(len, radius) : 0;
        joystick.dy = len > 0 ? dy / Math.max(len, radius) : 0;
      }
      if (touch.identifier === actionTouch) {
        input.mouseX = touch.clientX;
        input.mouseY = touch.clientY;
      }
    }
    updateMovement();
  });

  canvas.addEventListener("touchend", (event) => {
    for (const touch of Array.from(event.changedTouches)) {
      if (touch.identifier === joystick.id) {
        joystick.id = null;
        joystick.dx = 0;
        joystick.dy = 0;
        input.touchActive = false;
      }
      if (touch.identifier === actionTouch) {
        actionTouch = null;
        input.isPointerDown = false;
        input.pressStartMs = null;
      }
    }
    updateMovement();
  });

  canvas.addEventListener("touchcancel", () => {
    joystick.id = null;
    joystick.dx = 0;
    joystick.dy = 0;
    input.touchActive = false;
    actionTouch = null;
    input.isPointerDown = false;
    input.pressStartMs = null;
    updateMovement();
  });
}
