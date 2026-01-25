import { World } from "../../types";

export function drawMap(ctx: CanvasRenderingContext2D, world: World) {
    ctx.fillStyle = "#3a3"; ctx.fillRect(0, 0, world.width, world.height);
    ctx.strokeStyle = "rgba(0,0,0,0.05)"; ctx.lineWidth = 1;
    for (let x = 0; x <= world.width; x += 100) { ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, world.height); ctx.stroke(); }
    for (let y = 0; y <= world.height; y += 100) { ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(world.width, y); ctx.stroke(); }
}

export function lerp(a: number, b: number, alpha: number): number { return a + (b - a) * alpha; }

export function getScale(id: string, world: World | null, prevWorld: World | null): number {
    const curr = (world as any)?.players[id] || world?.resources[id] || world?.mobs[id] || world?.structures[id];
    const prev = (prevWorld as any)?.players[id] || prevWorld?.resources[id] || prevWorld?.mobs[id] || prevWorld?.structures[id];
    if (!curr || !prev) return 1.0;
    const currHp = curr.health ?? curr.amount; const prevHp = prev.health ?? prev.amount;
    if (currHp < prevHp) curr.lastHitAt = Date.now();
    if (!curr.lastHitAt) return 1.0;
    const elapsed = Date.now() - curr.lastHitAt;
    if (elapsed > 250) return 1.0;
    return 1.0 + Math.sin((elapsed / 250) * Math.PI) * 0.2;
}
