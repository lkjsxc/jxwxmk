import { Camera } from "./camera";
import { drawWorld } from "./visuals";
import { WorldState } from "../state/world";
import { UIManager } from "../ui/ui_manager";

export class Renderer {
  private ctx: CanvasRenderingContext2D;
  private camera: Camera;

  constructor(
    private canvas: HTMLCanvasElement,
    private world: WorldState,
    private ui: UIManager
  ) {
    const ctx = canvas.getContext("2d");
    if (!ctx) {
      throw new Error("Canvas2D required");
    }
    this.ctx = ctx;
    this.camera = new Camera();
    window.addEventListener("resize", () => this.resize());
    this.resize();
    canvas.addEventListener("wheel", (event) => {
      this.camera.adjustZoom(event.deltaY > 0 ? -0.1 : 0.1);
    });
  }

  render(): void {
    const { width, height } = this.canvas;
    this.ctx.clearRect(0, 0, width, height);

    const player = this.world.getLocalPlayer();
    if (player) {
      this.camera.follow(player.x, player.y);
    }
    this.camera.update();

    drawWorld(this.ctx, this.world, this.camera, width, height);
    this.ui.render(this.ctx, width, height, player);
  }

  private resize(): void {
    const ratio = window.devicePixelRatio || 1;
    this.canvas.width = Math.floor(window.innerWidth * ratio);
    this.canvas.height = Math.floor(window.innerHeight * ratio);
    this.canvas.style.width = `${window.innerWidth}px`;
    this.canvas.style.height = `${window.innerHeight}px`;
    this.ctx.setTransform(ratio, 0, 0, ratio, 0, 0);
  }
}
