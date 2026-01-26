import { Resource, Structure, Mob, Player, Npc, World } from "../../types";

export function drawNpc(ctx: CanvasRenderingContext2D, n: Npc, ix: number, iy: number, isTarget: boolean, scale: number) {
    if (isTarget) { drawOutline(ctx, ix, iy, 18 * scale, "cyan"); drawTooltip(ctx, ix, iy, n.name, "Talk", ""); }
    ctx.save(); ctx.translate(ix, iy); ctx.scale(scale, scale);
    ctx.fillStyle = n.n_type === "Elder" ? "#48f" : n.n_type === "Merchant" ? "#fa0" : "#777";
    ctx.beginPath(); ctx.arc(0, 0, 15, 0, Math.PI * 2); ctx.fill(); ctx.strokeStyle = "#fff"; ctx.stroke();
    ctx.restore();
    ctx.fillStyle = "white"; ctx.font = "bold 12px sans-serif"; ctx.textAlign = "center";
    ctx.fillText(n.name, ix, iy - 25);
}

export function drawResource(ctx: CanvasRenderingContext2D, r: Resource, isTarget: boolean, scale: number) {
    if (isTarget) { drawOutline(ctx, r.x, r.y, 22 * scale, "yellow"); drawTooltip(ctx, r.x, r.y, r.r_type, "Gather", ""); }
    ctx.save(); ctx.translate(r.x, r.y); ctx.scale(scale, scale);
    ctx.beginPath();
    ctx.fillStyle = r.r_type === "Tree" ? "#2e2" : r.r_type === "Rock" ? "#888" : "#ea2";
    ctx.arc(0, 0, 20, 0, Math.PI * 2); ctx.fill(); ctx.strokeStyle = "#000"; ctx.stroke();
    ctx.restore();
    let max = r.r_type === "Tree" ? 5 : r.r_type === "Rock" ? 10 : 1;
    if (r.amount < max) drawGauge(ctx, r.x, r.y - 30, 30, 4, r.amount / max);
}

export function drawStructure(ctx: CanvasRenderingContext2D, s: Structure, isTarget: boolean, scale: number) {
    if (isTarget) { drawOutline(ctx, s.x, s.y, 25 * scale, "white"); drawTooltip(ctx, s.x, s.y, s.s_type, "Attack", "Use"); }
    ctx.save(); ctx.translate(s.x, s.y); ctx.scale(scale, scale);
    if (s.s_type === "Torch") { ctx.fillStyle = "#fa0"; ctx.beginPath(); ctx.arc(0, 0, 10, 0, Math.PI*2); ctx.fill(); ctx.strokeStyle = "#fff"; ctx.stroke(); }
    else if (s.s_type === "Wall") { ctx.fillStyle = "#642"; ctx.fillRect(-20, -20, 40, 40); ctx.strokeRect(-20, -20, 40, 40); }
    else { ctx.fillStyle = "#444"; ctx.fillRect(-25, -25, 50, 50); }
    ctx.restore();
    let max = s.s_type === "Wall" ? 200 : s.s_type === "Door" ? 100 : 50;
    if (s.health < max) drawGauge(ctx, s.x, s.y - 35, 40, 4, s.health / max);
}

export function drawMob(ctx: CanvasRenderingContext2D, m: Mob, ix: number, iy: number, isTarget: boolean, scale: number) {

    if (isTarget) { drawOutline(ctx, ix, iy, 15 * scale, "red"); drawTooltip(ctx, ix, iy, m.m_type, "Attack", ""); }

    ctx.save(); ctx.translate(ix, iy); ctx.scale(scale, scale);

        ctx.fillStyle = m.m_type === "Wolf" ? "#999" : m.m_type === "Bear" ? "#531" : "#fff";

        ctx.beginPath(); ctx.arc(0, 0, 12, 0, Math.PI*2); ctx.fill();

        ctx.strokeStyle = "#000"; ctx.stroke();

        ctx.restore();

    let max = m.m_type === "Wolf" ? 50 : m.m_type === "Bear" ? 200 : 10;

    // Scale max HP by level for display?

    // Backend: `health *= 1.0 + (level * 0.2)`.

    // So max should also be scaled to show correct bar.

    let lvl = m.level || 1;

    max *= 1.0 + (lvl as number * 0.2);

    

    if (m.health < max) drawGauge(ctx, ix, iy - 25, 24, 4, m.health / max);

    

        // Level Display

    

        ctx.fillStyle = "#fff"; ctx.font = "10px sans-serif"; ctx.textAlign = "center";

    

        ctx.fillText(`Lv.${lvl}`, ix, iy - 15);

}

export function drawPlayer(ctx: CanvasRenderingContext2D, p: Player, ix: number, iy: number, isTarget: boolean, scale: number) {
    if (isTarget) { drawOutline(ctx, ix, iy, 18 * scale, "red"); drawTooltip(ctx, ix, iy, p.username, "Attack", ""); }
    ctx.save(); ctx.translate(ix, iy); ctx.scale(scale, scale);
    ctx.fillStyle = "#f00"; ctx.beginPath(); ctx.arc(0, 0, 15, 0, Math.PI * 2); ctx.fill(); ctx.strokeStyle = "#000"; ctx.stroke();
    ctx.restore();
    ctx.fillStyle = "white"; ctx.font = "12px sans-serif"; ctx.textAlign = "center";
    ctx.fillText(p.username, ix, iy - 25);
    if (p.health < 100) drawGauge(ctx, ix, iy - 30, 30, 4, p.health / 100);
}

function drawOutline(ctx: CanvasRenderingContext2D, x: number, y: number, r: number, color: string) {
    ctx.beginPath(); ctx.arc(x, y, r, 0, Math.PI * 2);
    ctx.strokeStyle = color; ctx.lineWidth = 3; ctx.stroke(); ctx.lineWidth = 1;
}

function drawTooltip(ctx: CanvasRenderingContext2D, x: number, y: number, name: string, aAction: string, bAction: string) {
    ctx.fillStyle = "white"; ctx.font = "bold 14px sans-serif"; ctx.textAlign = "center";
    ctx.fillText(name, x, y - 55);
    ctx.font = "12px sans-serif"; let actions = `[A/Click] ${aAction}`; if (bAction) actions += ` | [B/KeyE] ${bAction}`;
    ctx.fillText(actions, x, y - 40);
}

function drawGauge(ctx: CanvasRenderingContext2D, x: number, y: number, w: number, h: number, pct: number) {
    ctx.fillStyle = "rgba(0,0,0,0.5)"; ctx.fillRect(x - w/2, y, w, h);
    ctx.fillStyle = "rgba(255,0,0,0.5)"; ctx.fillRect(x - w/2, y, w * Math.max(0, pct), h);
}
