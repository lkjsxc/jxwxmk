import { AchievementInfo } from "../data/achievements";

export function drawAchievements(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  width: number,
  height: number,
  achievements: AchievementInfo[],
  selectedId: string | null
): { selectionBoxes: { id: string; x: number; y: number; w: number; h: number }[]; pinBox: { x: number; y: number; w: number; h: number } } {
  ctx.fillStyle = "rgba(0,0,0,0.6)";
  ctx.fillRect(x, y, width, height);

  const selectionBoxes: { id: string; x: number; y: number; w: number; h: number }[] = [];
  achievements.forEach((entry, index) => {
    const boxY = y + 12 + index * 26;
    const box = { id: entry.id, x: x + 12, y: boxY, w: width * 0.45, h: 20 };
    ctx.fillStyle = entry.id === selectedId ? "rgba(255,255,255,0.2)" : "rgba(255,255,255,0.08)";
    ctx.fillRect(box.x, box.y, box.w, box.h);
    ctx.fillStyle = "white";
    ctx.font = "12px sans-serif";
    ctx.fillText(entry.name, box.x + 6, box.y + 14);
    selectionBoxes.push(box);
  });

  const detailX = x + width * 0.5;
  const detailY = y + 12;
  const selected = achievements.find((entry) => entry.id === selectedId) || achievements[0];
  ctx.fillStyle = "white";
  ctx.font = "14px sans-serif";
  ctx.fillText(selected?.name ?? "", detailX + 12, detailY + 20);
  ctx.font = "12px sans-serif";
  ctx.fillText(selected?.description ?? "", detailX + 12, detailY + 40);

  const pinBox = { x: detailX + 12, y: detailY + 70, w: 80, h: 22 };
  ctx.fillStyle = "rgba(255,255,255,0.2)";
  ctx.fillRect(pinBox.x, pinBox.y, pinBox.w, pinBox.h);
  ctx.fillStyle = "white";
  ctx.fillText("Pin", pinBox.x + 22, pinBox.y + 15);

  return { selectionBoxes, pinBox };
}
