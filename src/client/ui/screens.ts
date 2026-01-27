export function drawGameOver(ctx: CanvasRenderingContext2D): { respawn: { x: number; y: number; w: number; h: number } } {
  ctx.save();
  ctx.fillStyle = "rgba(15,23,42,0.8)";
  ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);
  ctx.fillStyle = "#f87171";
  ctx.font = "48px Space Grotesk";
  ctx.fillText("YOU DIED", ctx.canvas.width / 2 - 120, ctx.canvas.height / 2 - 20);
  const respawn = { x: ctx.canvas.width / 2 - 70, y: ctx.canvas.height / 2 + 10, w: 140, h: 40 };
  ctx.fillStyle = "#22c55e";
  ctx.fillRect(respawn.x, respawn.y, respawn.w, respawn.h);
  ctx.fillStyle = "#0f172a";
  ctx.font = "18px Space Grotesk";
  ctx.fillText("Respawn", respawn.x + 28, respawn.y + 26);
  ctx.restore();
  return { respawn };
}

export function drawSessionRevoked(ctx: CanvasRenderingContext2D): void {
  ctx.save();
  ctx.fillStyle = "rgba(15,23,42,0.9)";
  ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);
  ctx.fillStyle = "#f97316";
  ctx.font = "24px Space Grotesk";
  ctx.fillText("Session revoked", ctx.canvas.width / 2 - 90, ctx.canvas.height / 2 - 10);
  ctx.font = "14px Space Grotesk";
  ctx.fillStyle = "#e2e8f0";
  ctx.fillText("Logged in elsewhere.", ctx.canvas.width / 2 - 80, ctx.canvas.height / 2 + 16);
  ctx.restore();
}
