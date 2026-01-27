import { QuestState } from "../state/types";

export interface Toast {
  text: string;
  until: number;
}

export interface NpcInteraction {
  npc_id: string;
  name: string;
  text: string;
  options: string[];
}

export function drawToast(ctx: CanvasRenderingContext2D, toast: Toast | null) {
  if (!toast) return;
  if (Date.now() > toast.until) return;
  ctx.save();
  ctx.fillStyle = "rgba(15,23,42,0.8)";
  ctx.fillRect(ctx.canvas.width / 2 - 120, ctx.canvas.height - 80, 240, 32);
  ctx.fillStyle = "#e2e8f0";
  ctx.font = "14px Space Grotesk";
  ctx.fillText(toast.text, ctx.canvas.width / 2 - 100, ctx.canvas.height - 58);
  ctx.restore();
}

export function drawNpcModal(
  ctx: CanvasRenderingContext2D,
  modal: NpcInteraction | null,
): Array<{ option: number; x: number; y: number; w: number; h: number }> {
  if (!modal) return [];
  const width = 320;
  const height = 200;
  const x = ctx.canvas.width / 2 - width / 2;
  const y = ctx.canvas.height / 2 - height / 2;
  ctx.save();
  ctx.fillStyle = "rgba(15,23,42,0.95)";
  ctx.fillRect(x, y, width, height);
  ctx.fillStyle = "#e2e8f0";
  ctx.font = "16px Space Grotesk";
  ctx.fillText(modal.name, x + 20, y + 30);
  ctx.font = "14px Space Grotesk";
  ctx.fillText(modal.text, x + 20, y + 54);
  const buttons: Array<{ option: number; x: number; y: number; w: number; h: number }> = [];
  modal.options.forEach((option, index) => {
    const btn = { option: index, x: x + 20, y: y + 80 + index * 34, w: width - 40, h: 26 };
    buttons.push(btn);
    ctx.fillStyle = "#38bdf8";
    ctx.fillRect(btn.x, btn.y, btn.w, btn.h);
    ctx.fillStyle = "#0f172a";
    ctx.fillText(option, btn.x + 10, btn.y + 18);
  });
  ctx.restore();
  return buttons;
}

export function drawPinnedTracker(ctx: CanvasRenderingContext2D, quest?: QuestState) {
  if (!quest) return;
  ctx.save();
  const x = ctx.canvas.width - 260;
  const y = 20;
  ctx.fillStyle = "rgba(15,23,42,0.85)";
  ctx.fillRect(x, y, 240, 80);
  ctx.fillStyle = "#e2e8f0";
  ctx.font = "13px Space Grotesk";
  ctx.fillText(quest.name, x + 10, y + 20);
  quest.objectives.forEach((obj, index) => {
    ctx.fillText(
      `${obj.kind}: ${obj.current}/${obj.count}`,
      x + 10,
      y + 40 + index * 14,
    );
  });
  ctx.restore();
}
