export interface Toast {
  text: string;
  expiresAt: number;
}

export interface NpcModal {
  npcId: string;
  name: string;
  text: string;
  options: string[];
}

export function drawToast(
  ctx: CanvasRenderingContext2D,
  toast: Toast | null,
  width: number,
  height: number
): void {
  if (!toast || Date.now() > toast.expiresAt) {
    return;
  }
  const boxWidth = Math.min(300, width - 40);
  const x = (width - boxWidth) / 2;
  const y = height - 80;
  ctx.fillStyle = "rgba(0,0,0,0.7)";
  ctx.fillRect(x, y, boxWidth, 40);
  ctx.fillStyle = "white";
  ctx.font = "14px sans-serif";
  ctx.textAlign = "center";
  ctx.fillText(toast.text, width / 2, y + 24);
}

export function drawNpcModal(
  ctx: CanvasRenderingContext2D,
  modal: NpcModal | null,
  width: number,
  height: number
): { optionBoxes: { index: number; x: number; y: number; w: number; h: number }[] } {
  if (!modal) {
    return { optionBoxes: [] };
  }
  const w = Math.min(400, width - 40);
  const h = 160;
  const x = (width - w) / 2;
  const y = (height - h) / 2;
  ctx.fillStyle = "rgba(0,0,0,0.8)";
  ctx.fillRect(x, y, w, h);
  ctx.fillStyle = "white";
  ctx.font = "16px sans-serif";
  ctx.textAlign = "left";
  ctx.fillText(modal.name, x + 16, y + 28);
  ctx.font = "13px sans-serif";
  ctx.fillText(modal.text, x + 16, y + 52);

  const optionBoxes = modal.options.map((option, index) => {
    const boxY = y + 80 + index * 28;
    ctx.fillStyle = "rgba(255,255,255,0.1)";
    ctx.fillRect(x + 16, boxY, w - 32, 22);
    ctx.fillStyle = "white";
    ctx.fillText(option, x + 24, boxY + 16);
    return { index, x: x + 16, y: boxY, w: w - 32, h: 22 };
  });

  return { optionBoxes };
}
