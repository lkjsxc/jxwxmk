import { WorldState, EntitySnapshot } from "../state/world";
import { Camera } from "./camera";

export function drawWorld(
  ctx: CanvasRenderingContext2D,
  world: WorldState,
  camera: Camera,
  width: number,
  height: number
): void {
  ctx.save();
  ctx.translate(width / 2, height / 2);
  ctx.scale(camera.zoom, camera.zoom);
  ctx.translate(-camera.x, -camera.y);

  drawGrid(ctx, width, height, camera);

  const entities: EntitySnapshot[] = [];
  for (const chunk of world.chunks.values()) {
    for (const entity of chunk.entities.values()) {
      entities.push(entity);
    }
  }

  let closest: EntitySnapshot | null = null;
  let closestDist = 60;

  entities.forEach((entity) => {
    const { x, y } = entity;
    ctx.fillStyle = pickColor(entity.kind);
    ctx.beginPath();
    ctx.arc(x, y, 6, 0, Math.PI * 2);
    ctx.fill();

    if (entity.hp !== undefined && entity.max_hp !== undefined && entity.hp < entity.max_hp) {
      drawHpBar(ctx, x, y - 12, entity.hp, entity.max_hp);
    }

    const dx = x - camera.x;
    const dy = y - camera.y;
    const dist = Math.hypot(dx, dy);
    if (dist < closestDist) {
      closestDist = dist;
      closest = entity;
    }
  });

  if (closest) {
    ctx.strokeStyle = "rgba(255,255,255,0.8)";
    ctx.beginPath();
    ctx.arc(closest.x, closest.y, 10, 0, Math.PI * 2);
    ctx.stroke();
    drawTooltip(ctx, closest);
  }

  ctx.restore();
}

function drawGrid(ctx: CanvasRenderingContext2D, width: number, height: number, camera: Camera): void {
  const size = 64;
  const left = camera.x - width;
  const right = camera.x + width;
  const top = camera.y - height;
  const bottom = camera.y + height;
  ctx.strokeStyle = "rgba(255,255,255,0.05)";
  ctx.lineWidth = 1;
  for (let x = Math.floor(left / size) * size; x < right; x += size) {
    ctx.beginPath();
    ctx.moveTo(x, top);
    ctx.lineTo(x, bottom);
    ctx.stroke();
  }
  for (let y = Math.floor(top / size) * size; y < bottom; y += size) {
    ctx.beginPath();
    ctx.moveTo(left, y);
    ctx.lineTo(right, y);
    ctx.stroke();
  }
}

function drawHpBar(ctx: CanvasRenderingContext2D, x: number, y: number, hp: number, max: number): void {
  const width = 24;
  const height = 4;
  const ratio = Math.max(0, Math.min(1, hp / max));
  ctx.fillStyle = "rgba(255,0,0,0.5)";
  ctx.fillRect(x - width / 2, y, width * ratio, height);
  ctx.strokeStyle = "rgba(0,0,0,0.4)";
  ctx.strokeRect(x - width / 2, y, width, height);
}

function drawTooltip(ctx: CanvasRenderingContext2D, entity: EntitySnapshot): void {
  let label = "Interact";
  switch (entity.kind) {
    case "resource":
      label = "Gather";
      break;
    case "mob":
      label = "Attack";
      break;
    case "npc":
      label = "Talk";
      break;
    case "structure":
      label = "Use";
      break;
  }
  ctx.fillStyle = "rgba(0,0,0,0.6)";
  ctx.fillRect(entity.x - 30, entity.y - 30, 60, 16);
  ctx.fillStyle = "white";
  ctx.font = "10px sans-serif";
  ctx.textAlign = "center";
  ctx.fillText(label, entity.x, entity.y - 18);
}

function pickColor(kind: string): string {
  switch (kind) {
    case "player":
      return "#7dd3fc";
    case "resource":
      return "#34d399";
    case "mob":
      return "#f87171";
    case "structure":
      return "#fbbf24";
    case "npc":
      return "#a78bfa";
    default:
      return "#d1d5db";
  }
}
