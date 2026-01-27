export function drawInventory(ctx: CanvasRenderingContext2D) {
  const width = ctx.canvas.width;
  const columns = width < 600 ? 3 : width < 900 ? 5 : 7;
  const rows = Math.ceil(30 / columns);
  const slotSize = 48;
  const startX = ctx.canvas.width / 2 - (columns * slotSize + (columns - 1) * 6) / 2;
  const startY = 120;

  ctx.save();
  ctx.fillStyle = "rgba(15,23,42,0.9)";
  ctx.fillRect(startX - 16, startY - 40, columns * slotSize + 32, rows * slotSize + 60);
  ctx.fillStyle = "#e2e8f0";
  ctx.font = "16px Space Grotesk";
  ctx.fillText("Inventory", startX, startY - 16);

  for (let row = 0; row < rows; row += 1) {
    for (let col = 0; col < columns; col += 1) {
      const x = startX + col * (slotSize + 6);
      const y = startY + row * (slotSize + 6);
      ctx.strokeStyle = "rgba(148,163,184,0.4)";
      ctx.strokeRect(x, y, slotSize, slotSize);
    }
  }
  ctx.restore();
}
