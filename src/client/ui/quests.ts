import { QuestState } from "../state/quests";

export function drawQuestLog(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  width: number,
  height: number,
  quests: QuestState
): { pinBoxes: { id: string; x: number; y: number; w: number; h: number }[] } {
  ctx.fillStyle = "rgba(0,0,0,0.6)";
  ctx.fillRect(x, y, width, height);

  const pinBoxes: { id: string; x: number; y: number; w: number; h: number }[] = [];
  quests.quests.forEach((quest, index) => {
    const boxY = y + 12 + index * 42;
    ctx.fillStyle = "rgba(255,255,255,0.08)";
    ctx.fillRect(x + 12, boxY, width - 24, 36);
    ctx.fillStyle = "white";
    ctx.font = "12px sans-serif";
    ctx.fillText(`${quest.name} (${quest.state})`, x + 20, boxY + 16);

    const pinBox = { id: quest.id, x: x + width - 70, y: boxY + 8, w: 50, h: 20 };
    ctx.fillStyle = "rgba(255,255,255,0.2)";
    ctx.fillRect(pinBox.x, pinBox.y, pinBox.w, pinBox.h);
    ctx.fillStyle = "white";
    ctx.fillText("Pin", pinBox.x + 12, pinBox.y + 14);
    pinBoxes.push(pinBox);
  });

  return { pinBoxes };
}
