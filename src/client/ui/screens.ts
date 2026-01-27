export function drawGameOver(
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number
): { respawnBox: { x: number; y: number; w: number; h: number } } {
  ctx.fillStyle = "rgba(0,0,0,0.7)";
  ctx.fillRect(0, 0, width, height);
  ctx.fillStyle = "white";
  ctx.font = "32px sans-serif";
  ctx.textAlign = "center";
  ctx.fillText("YOU DIED", width / 2, height / 2 - 20);

  const respawnBox = { x: width / 2 - 60, y: height / 2 + 10, w: 120, h: 32 };
  ctx.fillStyle = "rgba(255,255,255,0.2)";
  ctx.fillRect(respawnBox.x, respawnBox.y, respawnBox.w, respawnBox.h);
  ctx.fillStyle = "white";
  ctx.font = "14px sans-serif";
  ctx.fillText("RESPAWN", width / 2, respawnBox.y + 22);

  return { respawnBox };
}

export function drawSessionRevoked(
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number
): void {
  ctx.fillStyle = "rgba(0,0,0,0.8)";
  ctx.fillRect(0, 0, width, height);
  ctx.fillStyle = "white";
  ctx.font = "20px sans-serif";
  ctx.textAlign = "center";
  ctx.fillText("Session revoked", width / 2, height / 2 - 10);
  ctx.font = "14px sans-serif";
  ctx.fillText("Log in again to continue", width / 2, height / 2 + 12);
}
