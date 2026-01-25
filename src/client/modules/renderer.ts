import { World, Player, Resource, Mob, Structure } from "../types";
import { InputManager } from "./input";
import { Camera } from "./camera";
import { UIManager } from "./ui";

export class Renderer {
    canvas: HTMLCanvasElement; ctx: CanvasRenderingContext2D; camera: Camera;

    constructor() {
        this.canvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
        this.ctx = this.canvas.getContext('2d')!;
        this.camera = new Camera();
        this.resize();
        window.addEventListener('resize', () => this.resize());
    }

    resize() { this.canvas.width = window.innerWidth; this.canvas.height = window.innerHeight; this.camera.resize(); }
    private lerp(a: number, b: number, alpha: number): number { return a + (b - a) * alpha; }

    render(world: World | null, prevWorld: World | null, alpha: number, input: InputManager, myId: string | null, ui: UIManager) {
        const zoomDelta = input.getZoomDelta();
        if (zoomDelta !== 0) this.camera.setZoom(zoomDelta);

        if (world && myId && world.players[myId]) {
            const me = world.players[myId]; const pMe = prevWorld?.players[myId] || me;
            this.camera.follow(this.lerp(pMe.x, me.x, alpha), this.lerp(pMe.y, me.y, alpha));
        }
        this.camera.update();

        this.ctx.fillStyle = "#222"; this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
        this.ctx.save();
        this.ctx.translate(this.canvas.width / 2, this.canvas.height / 2);
        this.ctx.scale(this.camera.zoom, this.camera.zoom);
        this.ctx.translate(-this.camera.x, -this.camera.y);

        if (world) {
            this.drawMap(world);
            const me = myId ? world.players[myId] : null;
            const targetId = this.findClosestTarget(world, me);

            for (const id in world.resources) this.drawResource(world.resources[id], id === targetId, world, prevWorld);
            for (const id in world.structures) this.drawStructure(world.structures[id], id === targetId, world, prevWorld);
            for (const id in world.mobs) {
                const m = world.mobs[id]; const pM = prevWorld?.mobs[id] || m;
                this.drawMob(m, this.lerp(pM.x, m.x, alpha), this.lerp(pM.y, m.y, alpha), id === targetId, world, prevWorld);
            }
            for (const id in world.players) {
                const p = world.players[id]; const pP = prevWorld?.players[id] || p;
                this.drawPlayer(p, this.lerp(pP.x, p.x, alpha), this.lerp(pP.y, p.y, alpha), id === targetId, world, prevWorld);
            }
        }
        this.ctx.restore();

        if (!world) {
            this.ctx.fillStyle = "#fff"; this.ctx.font = "20px sans-serif"; this.ctx.fillText("Connecting...", 50, 50);
        } else {
            this.drawUI(input);
            this.drawHUD(world, myId);
            if (myId && world.players[myId]) ui.render(this.ctx, world.players[myId], input);
        }
    }

    private findClosestTarget(world: World, me: Player | null): string | null {
        if (!me) return null;
        let closestId: string | null = null;
        let minDist = 60; // Interaction Range

        const check = (id: string, x: number, y: number) => {
            if (id === me.id) return;
            const d = Math.hypot(me.x - x, me.y - y);
            if (d < minDist) { minDist = d; closestId = id; }
        };

        for (const id in world.resources) check(id, world.resources[id].x, world.resources[id].y);
        for (const id in world.structures) check(id, world.structures[id].x, world.structures[id].y);
        for (const id in world.mobs) check(id, world.mobs[id].x, world.mobs[id].y);
        for (const id in world.players) check(id, world.players[id].x, world.players[id].y);

        return closestId;
    }

    private getScale(id: string, world: World | null, prevWorld: World | null): number {
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

    drawMap(world: World) {
        this.ctx.fillStyle = "#3a3"; this.ctx.fillRect(0, 0, world.width, world.height);
        this.ctx.strokeStyle = "rgba(0,0,0,0.05)"; this.ctx.lineWidth = 1;
        for (let x = 0; x <= world.width; x += 100) { this.ctx.beginPath(); this.ctx.moveTo(x, 0); this.ctx.lineTo(x, world.height); this.ctx.stroke(); }
        for (let y = 0; y <= world.height; y += 100) { this.ctx.beginPath(); this.ctx.moveTo(0, y); this.ctx.lineTo(world.width, y); this.ctx.stroke(); }
    }

    drawResource(r: Resource, isTarget: boolean, w: World, pw: World | null) {
        const scale = this.getScale(r.id, w, pw);
        if (isTarget) { this.drawOutline(r.x, r.y, 22 * scale, "yellow"); this.drawInteractionTooltip(r.x, r.y, r.r_type, "[A] Gather", ""); }
        this.ctx.save(); this.ctx.translate(r.x, r.y); this.ctx.scale(scale, scale);
        this.ctx.beginPath();
        if (r.r_type === "Tree") this.ctx.fillStyle = "#2e2"; else if (r.r_type === "Rock") this.ctx.fillStyle = "#888"; else this.ctx.fillStyle = "#ea2";
        this.ctx.arc(0, 0, 20, 0, Math.PI * 2); this.ctx.fill();
        this.ctx.strokeStyle = "#000"; this.ctx.stroke();
        this.ctx.restore();
        let max = r.r_type === "Tree" ? 5 : r.r_type === "Rock" ? 10 : 1;
        if (r.amount < max) this.drawGauge(r.x, r.y - 30, 30, 4, r.amount / max);
    }

    drawStructure(s: Structure, isTarget: boolean, w: World, pw: World | null) {
        const scale = this.getScale(s.id, w, pw);
        if (isTarget) { this.drawOutline(s.x, s.y, 25 * scale, "white"); this.drawInteractionTooltip(s.x, s.y, s.s_type, "[A] Attack", "Use"); }
        this.ctx.save(); this.ctx.translate(s.x, s.y); this.ctx.scale(scale, scale);
        if (s.s_type === "Torch") { this.ctx.fillStyle = "#fa0"; this.ctx.beginPath(); this.ctx.arc(0, 0, 10, 0, Math.PI*2); this.ctx.fill(); this.ctx.strokeStyle = "#fff"; this.ctx.stroke(); }
        else if (s.s_type === "Wall") { this.ctx.fillStyle = "#642"; this.ctx.fillRect(-20, -20, 40, 40); this.ctx.strokeRect(-20, -20, 40, 40); }
        else { this.ctx.fillStyle = "#444"; this.ctx.fillRect(-25, -25, 50, 50); }
        this.ctx.restore();
        let max = s.s_type === "Wall" ? 200 : s.s_type === "Door" ? 100 : 50;
        if (s.health < max) this.drawGauge(s.x, s.y - 35, 40, 4, s.health / max);
    }

    drawMob(m: Mob, ix: number, iy: number, isTarget: boolean, w: World, pw: World | null) {
        const scale = this.getScale(m.id, w, pw);
        if (isTarget) { this.drawOutline(ix, iy, 15 * scale, "red"); this.drawInteractionTooltip(ix, iy, m.m_type, "[A] Attack", ""); }
        this.ctx.save(); this.ctx.translate(ix, iy); this.ctx.scale(scale, scale);
        this.ctx.fillStyle = m.m_type === "Wolf" ? "#999" : m.m_type === "Bear" ? "#531" : "#fff";
        this.ctx.beginPath(); this.ctx.arc(0, 0, 12, 0, Math.PI*2); this.ctx.fill();
        this.ctx.strokeStyle = "#000"; this.ctx.stroke();
        this.ctx.restore();
        let max = m.m_type === "Wolf" ? 50 : m.m_type === "Bear" ? 200 : 10;
        if (m.health < max) this.drawGauge(ix, iy - 25, 24, 4, m.health / max);
    }

    drawPlayer(p: Player, ix: number, iy: number, isTarget: boolean, w: World, pw: World | null) {
        const scale = this.getScale(p.id, w, pw);
        if (isTarget) { this.drawOutline(ix, iy, 18 * scale, "red"); this.drawInteractionTooltip(ix, iy, p.username, "[A] Attack", ""); }
        this.ctx.save(); this.ctx.translate(ix, iy); this.ctx.scale(scale, scale);
        this.ctx.fillStyle = "#f00"; this.ctx.beginPath(); this.ctx.arc(0, 0, 15, 0, Math.PI * 2); this.ctx.fill();
        this.ctx.strokeStyle = "#000"; this.ctx.stroke();
        this.ctx.restore();
        this.ctx.fillStyle = "white"; this.ctx.font = "12px sans-serif"; this.ctx.textAlign = "center";
        this.ctx.fillText(p.username, ix, iy - 25);
        if (p.health < 100) this.drawGauge(ix, iy - 30, 30, 4, p.health / 100);
    }

    drawOutline(x: number, y: number, r: number, color: string) {
        this.ctx.beginPath(); this.ctx.arc(x, y, r, 0, Math.PI * 2);
        this.ctx.strokeStyle = color; this.ctx.lineWidth = 3; this.ctx.stroke(); this.ctx.lineWidth = 1;
    }

    drawInteractionTooltip(x: number, y: number, name: string, aAction: string, bAction: string) {
        this.ctx.fillStyle = "white"; this.ctx.font = "bold 14px sans-serif"; this.ctx.textAlign = "center";
        this.ctx.fillText(name, x, y - 55);
        this.ctx.font = "12px sans-serif"; let actions = aAction; if (bAction) actions += ` | [B] ${bAction}`;
        this.ctx.fillText(actions, x, y - 40);
    }

    drawGauge(x: number, y: number, w: number, h: number, pct: number) {
        this.ctx.fillStyle = "rgba(0,0,0,0.5)"; this.ctx.fillRect(x - w/2, y, w, h);
        this.ctx.fillStyle = "rgba(255,0,0,0.6)"; this.ctx.fillRect(x - w/2, y, w * Math.max(0, pct), h);
    }

    drawHUD(world: World, myId: string | null) {
        if (!myId || !world.players[myId]) return;
        const p = world.players[myId];
        let y = 20;
        this.drawBarWithLabel(20, y, 200, 15, p.health / 100, "rgba(255,0,0,0.5)", "rgba(80,0,0,0.3)", "HP"); y += 20;
        this.drawBarWithLabel(20, y, 200, 15, p.hunger / 100, "rgba(255,165,0,0.5)", "rgba(80,40,0,0.3)", "HG"); y += 20;
        this.drawBarWithLabel(20, y, 200, 15, (100 - p.cold) / 100, "rgba(0,170,255,0.5)", "rgba(0,40,80,0.3)", "TP");
    }

    drawBarWithLabel(x: number, y: number, w: number, h: number, pct: number, fg: string, bg: string, label: string) {
        this.ctx.fillStyle = "white"; this.ctx.font = "bold 12px sans-serif"; this.ctx.textAlign = "left"; this.ctx.textBaseline = "middle";
        this.ctx.fillText(label, x, y + h/2);
        const barX = x + 25; this.ctx.fillStyle = bg; this.ctx.fillRect(barX, y, w, h);
        this.ctx.fillStyle = fg; this.ctx.fillRect(barX, y, w * Math.max(0, pct), h);
        this.ctx.strokeStyle = "rgba(0,0,0,0.5)"; this.ctx.strokeRect(barX, y, w, h);
    }

    drawUI(input: InputManager) {
        const now = Date.now();
        this.drawPieButton(input.btnA.x, input.btnA.y, input.btnA.radius, "A", input.btnA.active, "rgba(211,51,51,0.4)", (now - input.lastAttackAt) / input.attackCooldown);
        this.drawPieButton(input.btnB.x, input.btnB.y, input.btnB.radius, "B", input.btnB.active, "rgba(51,51,211,0.4)", (now - input.lastInteractAt) / input.interactCooldown);
    }

    drawPieButton(x: number, y: number, r: number, label: string, active: boolean, color: string, progress: number) {
        this.ctx.save(); this.ctx.beginPath(); this.ctx.arc(x, y, r, 0, Math.PI*2);
        this.ctx.fillStyle = active ? "rgba(255,255,255,0.2)" : color; this.ctx.fill();
        this.ctx.strokeStyle = "white"; this.ctx.stroke();
        if (progress < 1.0) {
            this.ctx.beginPath(); this.ctx.moveTo(x, y);
            this.ctx.arc(x, y, r, -Math.PI/2, -Math.PI/2 + (1 - progress) * Math.PI * 2, false);
            this.ctx.fillStyle = "rgba(0,0,0,0.4)"; this.ctx.fill();
            this.ctx.fillStyle = "white"; this.ctx.font = "bold 14px sans-serif"; this.ctx.textAlign = "center";
            this.ctx.fillText(((1-progress) * (label === "A" ? 0.5 : 0.3)).toFixed(1), x, y);
        } else {
            this.ctx.fillStyle = "white"; this.ctx.font = "bold 20px sans-serif"; this.ctx.textAlign = "center"; this.ctx.fillText(label, x, y);
        }
        this.ctx.restore();
    }
}
