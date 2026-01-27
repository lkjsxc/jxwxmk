import { Camera } from "./camera";
import { drawEntity, drawTooltip, findClosestTarget } from "./visuals";
import { WorldState } from "../state/world";
import { PlayerSession } from "../state/player";
import { UIManager } from "../ui/ui_manager";

export function startRenderLoop(
  canvas: HTMLCanvasElement,
  world: WorldState,
  session: PlayerSession,
  ui: UIManager,
) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const camera = new Camera();

  const resize = () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    camera.setViewport(canvas.width, canvas.height);
  };
  resize();
  window.addEventListener("resize", resize);
  canvas.addEventListener("wheel", (event) => {
    event.preventDefault();
    camera.adjustZoom(event.deltaY < 0 ? 0.1 : -0.1);
  });

  const frame = () => {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    const localPlayer = session.id ? world.players.get(session.id) : undefined;
    if (localPlayer) {
      camera.follow(localPlayer.x, localPlayer.y);
      session.lastSeenAt = Date.now();
    } else if (Date.now() - session.lastSeenAt > 3000) {
      ui.setGameOver(true);
    }
    camera.update();

    drawGrid(ctx, camera);

    const entities: any[] = [];
    for (const chunk of world.chunks.values()) {
      entities.push(...Object.values(chunk.entities.resources));
      entities.push(...Object.values(chunk.entities.mobs));
      entities.push(...Object.values(chunk.entities.structures));
      entities.push(...Object.values(chunk.entities.npcs));
    }
    entities.push(...world.players.values());

    for (const entity of entities) {
      drawEntity(ctx, camera, entity);
    }

    if (localPlayer) {
      const closest = findClosestTarget(
        entities.filter((e) => e.kind !== "player"),
        localPlayer.x,
        localPlayer.y,
        60,
      );
      if (closest) {
        const label = closest.kind === "npc" ? "Talk" : closest.kind === "resource" ? "Gather" : "Attack";
        drawTooltip(ctx, label);
      }
    }

    ui.render(ctx, session, world, camera);
    requestAnimationFrame(frame);
  };
  requestAnimationFrame(frame);
}

function drawGrid(ctx: CanvasRenderingContext2D, camera: Camera) {
  const spacing = 64 * camera.zoom;
  const originX = camera.pivotX - camera.x * camera.zoom;
  const originY = camera.pivotY - camera.y * camera.zoom;
  const offsetX = mod(originX, spacing);
  const offsetY = mod(originY, spacing);
  ctx.save();
  ctx.strokeStyle = "rgba(148,163,184,0.08)";
  for (let x = offsetX; x < ctx.canvas.width; x += spacing) {
    ctx.beginPath();
    ctx.moveTo(x, 0);
    ctx.lineTo(x, ctx.canvas.height);
    ctx.stroke();
  }
  for (let y = offsetY; y < ctx.canvas.height; y += spacing) {
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(ctx.canvas.width, y);
    ctx.stroke();
  }
  ctx.restore();
}

function mod(value: number, modulus: number) {
  return ((value % modulus) + modulus) % modulus;
}
