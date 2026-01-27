export interface AchievementView {
  id: string;
  name: string;
  description: string;
}

export const ALL_ACHIEVEMENTS: AchievementView[] = [
  { id: "first_steps", name: "First Steps", description: "Take 100 steps." }
];

export interface AchievementLayout {
  selectRects: Array<{ id: string; x: number; y: number; w: number; h: number }>;
}

export function drawAchievements(
  ctx: CanvasRenderingContext2D,
  unlocked: Set<string>,
  selectedId: string | null,
): AchievementLayout {
  const panelX = 80;
  const panelY = 120;
  const panelW = ctx.canvas.width - 160;
  const panelH = ctx.canvas.height - 200;
  const selectRects: AchievementLayout["selectRects"] = [];

  ctx.save();
  ctx.fillStyle = "rgba(15,23,42,0.9)";
  ctx.fillRect(panelX, panelY, panelW, panelH);
  ctx.fillStyle = "#e2e8f0";
  ctx.font = "16px Space Grotesk";
  ctx.fillText("Achievements", panelX + 20, panelY + 28);

  let y = panelY + 60;
  for (const achievement of ALL_ACHIEVEMENTS) {
    const rect = { id: achievement.id, x: panelX + 20, y: y - 18, w: 220, h: 28 };
    selectRects.push(rect);
    ctx.fillStyle = achievement.id === selectedId ? "#1f2937" : "rgba(30,41,59,0.8)";
    ctx.fillRect(rect.x, rect.y, rect.w, rect.h);
    ctx.fillStyle = unlocked.has(achievement.id) ? "#22c55e" : "#e2e8f0";
    ctx.fillText(achievement.name, rect.x + 10, rect.y + 18);
    y += 36;
  }

  const selected = ALL_ACHIEVEMENTS.find((a) => a.id === selectedId) ?? ALL_ACHIEVEMENTS[0];
  ctx.fillStyle = "#e2e8f0";
  ctx.font = "14px Space Grotesk";
  ctx.fillText(selected.name, panelX + 280, panelY + 80);
  ctx.fillText(selected.description, panelX + 280, panelY + 100);

  ctx.restore();
  return { selectRects };
}
