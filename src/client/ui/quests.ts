import { QuestState } from "../state/types";

export interface QuestLayout {
  pinButtons: Array<{ id: string; x: number; y: number; w: number; h: number }>;
}

export function drawQuests(ctx: CanvasRenderingContext2D, quests: QuestState[]): QuestLayout {
  const panelX = 80;
  const panelY = 120;
  const panelW = ctx.canvas.width - 160;
  const panelH = ctx.canvas.height - 200;
  const pinButtons: QuestLayout["pinButtons"] = [];

  ctx.save();
  ctx.fillStyle = "rgba(15,23,42,0.9)";
  ctx.fillRect(panelX, panelY, panelW, panelH);
  ctx.fillStyle = "#e2e8f0";
  ctx.font = "16px Space Grotesk";
  ctx.fillText("Quests", panelX + 20, panelY + 28);

  let y = panelY + 60;
  for (const quest of quests) {
    ctx.fillStyle = "rgba(30,41,59,0.8)";
    ctx.fillRect(panelX + 20, y - 20, panelW - 40, 60);
    ctx.fillStyle = "#e2e8f0";
    ctx.font = "14px Space Grotesk";
    ctx.fillText(`${quest.name} (${quest.state})`, panelX + 30, y);
    ctx.font = "12px Space Grotesk";
    quest.objectives.forEach((obj, index) => {
      ctx.fillText(
        `${obj.kind} ${obj.target}: ${obj.current}/${obj.count}`,
        panelX + 30,
        y + 18 + index * 14,
      );
    });
    const pinRect = { id: quest.id, x: panelX + panelW - 120, y: y - 14, w: 80, h: 24 };
    pinButtons.push(pinRect);
    ctx.fillStyle = "#38bdf8";
    ctx.fillRect(pinRect.x, pinRect.y, pinRect.w, pinRect.h);
    ctx.fillStyle = "#0f172a";
    ctx.fillText("Pin", pinRect.x + 24, pinRect.y + 16);
    y += 80;
  }

  ctx.restore();
  return { pinButtons };
}
