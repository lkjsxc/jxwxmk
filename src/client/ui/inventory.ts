import { InventoryState } from "../state/inventory";

export function drawInventory(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  width: number,
  inventory: InventoryState
): void {
  const columns = width > 520 ? 7 : width > 360 ? 5 : 3;
  const slotSize = 36;
  const gap = 6;
  const rows = Math.ceil(inventory.slots.length / columns);

  ctx.fillStyle = "rgba(0,0,0,0.6)";
  ctx.fillRect(x, y, width, rows * (slotSize + gap) + gap);

  inventory.slots.forEach((slot, index) => {
    const col = index % columns;
    const row = Math.floor(index / columns);
    const slotX = x + gap + col * (slotSize + gap);
    const slotY = y + gap + row * (slotSize + gap);
    ctx.fillStyle = "rgba(255,255,255,0.06)";
    ctx.fillRect(slotX, slotY, slotSize, slotSize);
    ctx.strokeStyle = "rgba(255,255,255,0.2)";
    ctx.strokeRect(slotX, slotY, slotSize, slotSize);

    if (slot.item) {
      ctx.fillStyle = "white";
      ctx.font = "10px sans-serif";
      ctx.fillText(slot.item.item, slotX + 3, slotY + 16);
      ctx.fillText(slot.item.count.toString(), slotX + slotSize - 12, slotY + slotSize - 4);
    }
  });
}
