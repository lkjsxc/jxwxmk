export interface InputState {
  dx: number;
  dy: number;
  attack: boolean;
  interact: boolean;
  mouseX: number;
  mouseY: number;
  isPointerDown: boolean;
  pressStartMs: number | null;
  keyQueue: string[];
  touchActive: boolean;
}
