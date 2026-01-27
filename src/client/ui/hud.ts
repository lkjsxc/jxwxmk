import { EntitySnapshot } from "../state/world";
import { InventoryState } from "../state/inventory";

export function drawHud(
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number,
  player: EntitySnapshot | null,
  inventory: InventoryState
): void {
  const hp = player?.hp ?? 100;
  const maxHp = player?.max_hp ?? 100;
  drawBar(ctx, 20, 20, 140, 10, hp / maxHp, "rgba(255,0,0,0.5)", "HP");
  drawBar(ctx, 20, 36, 140, 10, 1, "rgba(255,165,0,0.5)", "HG");
  drawBar(ctx, 20, 52, 140, 10, 1, "rgba(0,128,255,0.5)", "TP");

  drawHotbar(ctx, width, height, inventory);
}

function drawBar(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  w: number,
  h: number,
  ratio: number,
  color: string,
  label: string
): void {
  ctx.fillStyle = "rgba(0,0,0,0.4)";
  ctx.fillRect(x, y, w, h);
  ctx.fillStyle = color;
  ctx.fillRect(x, y, w * Math.max(0, Math.min(1, ratio)), h);
  ctx.fillStyle = "white";
  ctx.font = "10px sans-serif";
  ctx.fillText(label, x - 16, y + h - 1);
}

function drawHotbar(
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number,
  inventory: InventoryState
): void {
  const slots = 7;
  const slotSize = 36;
  const gap = 6;
  const totalWidth = slots * slotSize + (slots - 1) * gap;
  const startX = (width - totalWidth) / 2;
  const y = height - 60;

  for (let i = 0; i < slots; i += 1) {
    const x = startX + i * (slotSize + gap);
    ctx.fillStyle = "rgba(0,0,0,0.4)";
    ctx.fillRect(x, y, slotSize, slotSize);
    ctx.strokeStyle = i === inventory.activeSlot ? "yellow" : "rgba(255,255,255,0.2)";
    ctx.strokeRect(x, y, slotSize, slotSize);
    const slot = inventory.slots[i];
    if (slot?.item) {
      ctx.fillStyle = "white";
      ctx.font = "10px sans-serif";
      ctx.fillText(slot.item.item, x + 4, y + 16);
      ctx.fillText(slot.item.count.toString(), x + slotSize - 12, y + slotSize - 4);
    }
  }
}
