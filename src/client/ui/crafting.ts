import { RecipeDisplay } from "../data/recipes";

export function drawCrafting(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  width: number,
  height: number,
  recipes: RecipeDisplay[],
  selectedId: string | null
): { recipeBoxes: { id: string; x: number; y: number; w: number; h: number }[]; craftBox: { x: number; y: number; w: number; h: number } } {
  const listWidth = width * 0.45;
  const detailWidth = width - listWidth - 16;
  const recipeBoxes: { id: string; x: number; y: number; w: number; h: number }[] = [];

  ctx.fillStyle = "rgba(0,0,0,0.6)";
  ctx.fillRect(x, y, width, height);

  recipes.forEach((recipe, index) => {
    const boxY = y + 12 + index * 28;
    const box = { id: recipe.id, x: x + 8, y: boxY, w: listWidth - 16, h: 22 };
    ctx.fillStyle = recipe.id === selectedId ? "rgba(255,255,255,0.2)" : "rgba(255,255,255,0.08)";
    ctx.fillRect(box.x, box.y, box.w, box.h);
    ctx.fillStyle = "white";
    ctx.font = "12px sans-serif";
    ctx.fillText(recipe.name, box.x + 6, box.y + 15);
    recipeBoxes.push(box);
  });

  const detailX = x + listWidth + 8;
  const detailY = y + 12;
  ctx.fillStyle = "rgba(255,255,255,0.05)";
  ctx.fillRect(detailX, detailY, detailWidth, height - 24);

  const recipe = recipes.find((entry) => entry.id === selectedId) || recipes[0];
  ctx.fillStyle = "white";
  ctx.font = "14px sans-serif";
  ctx.fillText(recipe?.name ?? "", detailX + 12, detailY + 20);

  ctx.font = "12px sans-serif";
  recipe?.inputs.forEach((input, idx) => {
    ctx.fillText(`${input.item}: ${input.count}`, detailX + 12, detailY + 44 + idx * 16);
  });

  const craftBox = { x: detailX + 12, y: detailY + height - 70, w: detailWidth - 24, h: 28 };
  ctx.fillStyle = "rgba(255,255,255,0.2)";
  ctx.fillRect(craftBox.x, craftBox.y, craftBox.w, craftBox.h);
  ctx.fillStyle = "white";
  ctx.fillText("Craft", craftBox.x + 12, craftBox.y + 18);

  return { recipeBoxes, craftBox };
}
