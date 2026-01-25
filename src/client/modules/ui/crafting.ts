import { Player } from "../../types";

const RECIPES = [
    { name: "Wood Pick", code: "WoodPickaxe", req: { "Wood": 10 } },
    { name: "Stone Pick", code: "StonePickaxe", req: { "Wood": 10, "Stone": 10 } },
    { name: "Wood Wall", code: "WoodWall", req: { "Wood": 20 } },
    { name: "Torch", code: "Torch", req: { "Wood": 2 } },
    { name: "Workbench", code: "Workbench", req: { "Wood": 50 } },
    { name: "Door", code: "Door", req: { "Wood": 15 } }
];

export function drawCrafting(ctx: CanvasRenderingContext2D, player: Player, selectedRecipe: string | null, w: number, h: number, ui: any) {
    const listW = w * 0.4;
    
    // List
    let y = 40;
    for (const r of RECIPES) {
        const isSelected = selectedRecipe === r.code;
        if (isSelected) { ctx.fillStyle = "#444"; ctx.fillRect(10, y - 5, listW - 20, 40); }
        ctx.fillStyle = "white"; ctx.font = "16px sans-serif"; ctx.textAlign = "left";
        ctx.fillText(r.name, 20, y + 20);
        y += 45;
    }

    // Details
    if (selectedRecipe) {
        const r = RECIPES.find(x => x.code === selectedRecipe);
        if (r) {
            const dx = listW + 20; let dy = 50;
            ctx.fillStyle = "white"; ctx.font = "bold 22px sans-serif";
            ctx.fillText(r.name, dx, dy);
            
            dy += 40;
            ctx.font = "16px sans-serif"; ctx.fillStyle = "#aaa";
            ctx.fillText("Requirements:", dx, dy);
            dy += 25;

            let canCraft = true;
            for (const [mat, count] of Object.entries(r.req)) {
                const has = countItem(player, mat);
                ctx.fillStyle = has >= count ? "#4f4" : "#f44";
                ctx.fillText(`${mat}: ${has}/${count}`, dx, dy);
                if (has < count) canCraft = false;
                dy += 25;
            }

            // Craft Button
            dy += 20;
            ui.drawBtn(ctx, dx, dy, 120, 40, "Craft", canCraft);
        }
    }
}

function countItem(p: Player, kind: string): number {
    let c = 0;
    for (const slot of p.inventory.slots) {
        if (slot && slot.kind === kind) c += slot.amount;
    }
    return c;
}

export function handleCraftInput(mx: number, my: number, w: number, h: number, player: Player, selectedRecipe: string | null): { select?: string, craft?: boolean } {
    const listW = w * 0.4;
    let y = 40;
    for (const r of RECIPES) {
        if (mx >= 10 && mx <= listW - 10 && my >= y - 5 && my < y + 35) {
            return { select: r.code };
        }
        y += 45;
    }

    if (selectedRecipe) {
        const r = RECIPES.find(x => x.code === selectedRecipe);
        if (r) {
            const dx = listW + 20;
            // Recalculate dy based on reqs
            let dy = 50 + 40 + 25 + Object.keys(r.req).length * 25 + 20;
            if (mx >= dx && mx <= dx + 120 && my >= dy && my <= dy + 40) {
                 // Check reqs again? Server validates anyway.
                 return { craft: true };
            }
        }
    }
    return {};
}
