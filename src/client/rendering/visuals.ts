import { EntityUpdate } from "../state/types";
import { Camera } from "./camera";

export function drawEntity(ctx: CanvasRenderingContext2D, camera: Camera, entity: EntityUpdate) {
  const screen = camera.worldToScreen(entity.x, entity.y);
  const screenX = screen.x;
  const screenY = screen.y;

  ctx.save();
  if (entity.kind === "player") {
    ctx.fillStyle = "#f8fafc";
    ctx.beginPath();
    ctx.arc(screenX, screenY, 8 * camera.zoom, 0, Math.PI * 2);
    ctx.fill();
  } else if (entity.kind === "resource") {
    ctx.fillStyle = "#34d399";
    ctx.fillRect(screenX - 6, screenY - 6, 12, 12);
  } else if (entity.kind === "mob") {
    ctx.fillStyle = "#f87171";
    ctx.beginPath();
    ctx.arc(screenX, screenY, 7 * camera.zoom, 0, Math.PI * 2);
    ctx.fill();
  } else if (entity.kind === "structure") {
    ctx.fillStyle = "#93c5fd";
    ctx.fillRect(screenX - 8, screenY - 8, 16, 16);
    if (entity.subtype === "BarrierCore" && entity.range) {
      ctx.strokeStyle = "rgba(56,189,248,0.4)";
      ctx.setLineDash([6, 6]);
      ctx.beginPath();
      ctx.arc(screenX, screenY, entity.range * camera.zoom, 0, Math.PI * 2);
      ctx.stroke();
      ctx.setLineDash([]);
    }
  } else if (entity.kind === "npc") {
    ctx.fillStyle = "#facc15";
    ctx.beginPath();
    ctx.arc(screenX, screenY, 7 * camera.zoom, 0, Math.PI * 2);
    ctx.fill();
  }

  if (entity.hp !== undefined && entity.max_hp !== undefined && entity.max_hp > 0) {
    const barWidth = 20 * camera.zoom;
    const barX = screenX - barWidth / 2;
    const barY = screenY - 16 * camera.zoom;
    ctx.fillStyle = "rgba(15,23,42,0.7)";
    ctx.fillRect(barX, barY, barWidth, 4 * camera.zoom);
    ctx.fillStyle = "#22c55e";
    ctx.fillRect(barX, barY, barWidth * (entity.hp / entity.max_hp), 4 * camera.zoom);
  }

  if (entity.kind === "mob" && entity.level !== undefined) {
    ctx.fillStyle = "#e2e8f0";
    ctx.font = `${10 * camera.zoom}px Space Grotesk`;
    ctx.fillText(`Lv.${entity.level}`, screenX - 10, screenY - 18 * camera.zoom);
  }
  ctx.restore();
}

export function findClosestTarget(
  entities: EntityUpdate[],
  x: number,
  y: number,
  radius: number,
): EntityUpdate | null {
  let closest: EntityUpdate | null = null;
  let closestDist = radius;
  for (const entity of entities) {
    const dx = entity.x - x;
    const dy = entity.y - y;
    const dist = Math.hypot(dx, dy);
    if (dist < closestDist) {
      closestDist = dist;
      closest = entity;
    }
  }
  return closest;
}

export function drawTooltip(ctx: CanvasRenderingContext2D, text: string) {
  ctx.save();
  ctx.fillStyle = "rgba(15,23,42,0.8)";
  ctx.fillRect(ctx.canvas.width / 2 - 60, ctx.canvas.height / 2 + 40, 120, 26);
  ctx.fillStyle = "#f8fafc";
  ctx.font = "12px Space Grotesk";
  ctx.fillText(text, ctx.canvas.width / 2 - 50, ctx.canvas.height / 2 + 58);
  ctx.restore();
}
