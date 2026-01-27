import { EntityUpdate } from "../state/types";

export function drawHUD(ctx: CanvasRenderingContext2D, player: EntityUpdate | undefined) {
  const x = 20;
  const y = 20;
  const barWidth = 140;
  const barHeight = 10;
  const hp = player?.hp ?? 100;
  const hpMax = player?.max_hp ?? 100;

  drawBar(ctx, x, y, barWidth, barHeight, hp / hpMax, "#ef4444", "HP");
  drawBar(ctx, x, y + 18, barWidth, barHeight, 0.8, "#f97316", "Hunger");
  drawBar(ctx, x, y + 36, barWidth, barHeight, 0.6, "#38bdf8", "Temp");

  drawHotbar(ctx);
}

function drawBar(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  w: number,
  h: number,
  fill: number,
  color: string,
  label: string,
) {
  ctx.save();
  ctx.fillStyle = "rgba(15,23,42,0.8)";
  ctx.fillRect(x, y, w, h);
  ctx.fillStyle = color;
  ctx.fillRect(x, y, w * Math.max(0, Math.min(1, fill)), h);
  ctx.fillStyle = "#e2e8f0";
  ctx.font = "10px Space Grotesk";
  ctx.fillText(label, x + w + 8, y + h - 1);
  ctx.restore();
}

function drawHotbar(ctx: CanvasRenderingContext2D) {
  const slots = 7;
  const size = 40;
  const totalWidth = slots * size + (slots - 1) * 6;
  const startX = ctx.canvas.width / 2 - totalWidth / 2;
  const y = ctx.canvas.height - 60;
  ctx.save();
  for (let i = 0; i < slots; i += 1) {
    const x = startX + i * (size + 6);
    ctx.fillStyle = "rgba(15,23,42,0.8)";
    ctx.fillRect(x, y, size, size);
    ctx.strokeStyle = i === 0 ? "#facc15" : "rgba(148,163,184,0.4)";
    ctx.strokeRect(x, y, size, size);
  }
  ctx.restore();
}
