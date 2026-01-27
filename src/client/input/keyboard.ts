import { InputState } from "./state";

export function bindKeyboard(input: InputState, canvas: HTMLCanvasElement) {
  const keyMap: Record<string, boolean> = {};

  window.addEventListener("keydown", (event) => {
    keyMap[event.key.toLowerCase()] = true;
    if (event.key === "Backspace") {
      input.keyQueue.push("\b");
    } else if (event.key.length === 1 && !event.metaKey && !event.ctrlKey) {
      input.keyQueue.push(event.key);
    }
  });

  window.addEventListener("keyup", (event) => {
    keyMap[event.key.toLowerCase()] = false;
  });

  canvas.addEventListener("mousemove", (event) => {
    input.mouseX = event.clientX;
    input.mouseY = event.clientY;
  });

  canvas.addEventListener("mousedown", (event) => {
    if (event.button === 0) {
      input.isPointerDown = true;
      input.pressStartMs = performance.now();
    }
  });

  canvas.addEventListener("mouseup", (event) => {
    if (event.button === 0) {
      input.isPointerDown = false;
      input.pressStartMs = null;
    }
  });

  canvas.addEventListener("contextmenu", (event) => {
    event.preventDefault();
  });

  return keyMap;
}

export function computeMovement(keyMap: Record<string, boolean>) {
  const dx = (keyMap["d"] ? 1 : 0) - (keyMap["a"] ? 1 : 0);
  const dy = (keyMap["s"] ? 1 : 0) - (keyMap["w"] ? 1 : 0);
  return { dx, dy };
}
