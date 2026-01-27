export interface RecipeView {
  id: string;
  name: string;
  requirements: string;
}

export const RECIPES: RecipeView[] = [
  { id: "WoodPickaxe", name: "Wood Pick", requirements: "Wood x10" },
  { id: "StonePickaxe", name: "Stone Pick", requirements: "Wood x10, Stone x10" },
  { id: "WoodWall", name: "Wood Wall", requirements: "Wood x20" },
  { id: "Door", name: "Door", requirements: "Wood x15" },
  { id: "Torch", name: "Torch", requirements: "Wood x2" },
  { id: "Workbench", name: "Workbench", requirements: "Wood x50" }
];

export interface CraftingLayout {
  listRects: Array<{ x: number; y: number; w: number; h: number }>
  craftButton: { x: number; y: number; w: number; h: number };
}

export function drawCrafting(ctx: CanvasRenderingContext2D, selectedIndex: number): CraftingLayout {
  const panelX = 80;
  const panelY = 120;
  const panelW = ctx.canvas.width - 160;
  const panelH = ctx.canvas.height - 200;
  const listRects: CraftingLayout["listRects"] = [];

  ctx.save();
  ctx.fillStyle = "rgba(15,23,42,0.9)";
  ctx.fillRect(panelX, panelY, panelW, panelH);
  ctx.fillStyle = "#e2e8f0";
  ctx.font = "16px Space Grotesk";
  ctx.fillText("Crafting", panelX + 20, panelY + 28);

  const listX = panelX + 20;
  const listY = panelY + 50;
  const rowH = 28;
  for (let i = 0; i < RECIPES.length; i += 1) {
    const y = listY + i * rowH;
    listRects.push({ x: listX, y, w: 200, h: rowH });
    ctx.fillStyle = i === selectedIndex ? "#1f2937" : "transparent";
    ctx.fillRect(listX, y - 16, 200, rowH);
    ctx.fillStyle = "#e2e8f0";
    ctx.fillText(RECIPES[i].name, listX + 8, y);
  }

  const detailX = panelX + 260;
  const detailY = panelY + 80;
  const recipe = RECIPES[selectedIndex] ?? RECIPES[0];
  ctx.fillStyle = "#e2e8f0";
  ctx.font = "18px Space Grotesk";
  ctx.fillText(recipe.name, detailX, detailY);
  ctx.font = "14px Space Grotesk";
  ctx.fillText("Requirements:", detailX, detailY + 24);
  ctx.fillText(recipe.requirements, detailX, detailY + 46);

  const craftButton = { x: detailX, y: detailY + 80, w: 140, h: 36 };
  ctx.fillStyle = "#22c55e";
  ctx.fillRect(craftButton.x, craftButton.y, craftButton.w, craftButton.h);
  ctx.fillStyle = "#0f172a";
  ctx.fillText("Craft", craftButton.x + 40, craftButton.y + 24);
  ctx.restore();

  return { listRects, craftButton };
}
