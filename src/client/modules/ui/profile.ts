import { Player } from "../../types";

export function drawProfile(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number, nameBuf: string, focused: boolean, ui: any) {
    ctx.fillStyle = "white"; ctx.textAlign = "center"; ctx.font = "24px sans-serif";
    ctx.fillText("Player Profile", w / 2, 40);
    ctx.font = "18px sans-serif"; ctx.fillText(`ID: ${player.id.substring(0,8)}...`, w / 2, 80);
    
    // Stats
    let sy = 120;
    ctx.textAlign = "left"; ctx.font = "16px monospace";
    const stats = [
        `Kills: ${player.stats?.mobs_killed ?? 0}`,
        `Deaths: ${player.stats?.deaths ?? 0}`,
        `Crafted: ${player.stats?.items_crafted ?? 0}`,
        `Steps: ${player.stats?.steps_taken ?? 0}`
    ];
    for(const s of stats) { ctx.fillText(s, w/2 - 80, sy); sy+=25; }

    const bw = 240; const bh = 45; const bx = (w - bw) / 2; const by = sy + 20;
    ctx.fillStyle = focused ? "#000" : "#111"; ctx.fillRect(bx, by, bw, bh); 
    ctx.strokeStyle = focused ? "#4a4" : "#555"; ctx.strokeRect(bx, by, bw, bh);
    ctx.fillStyle = "white"; ctx.textAlign = "left";
    let d = nameBuf; if (focused && (Date.now() % 1000 < 500)) d += "|";
    ctx.fillText(d || (focused ? "" : "Type Name..."), bx + 10, by + 30);

    ui.drawBtn(ctx, (w - 160) / 2, by + 60, 160, 40, "Update Name", false);
}

export function handleProfileInput(mx: number, my: number, w: number, h: number, player: Player, ui: any): { focus?: boolean, update?: boolean } {
    // Approx height calc
    let sy = 120 + 4 * 25;
    const bw = 240; const bh = 45; const bx = (w - bw) / 2; const by = sy + 20;
    
    if (ui.hit(mx, my, bx, by, bw, bh)) return { focus: true };
    if (ui.hit(mx, my, (w - 160) / 2, by + 60, 160, 40)) return { update: true };
    return { focus: false };
}
