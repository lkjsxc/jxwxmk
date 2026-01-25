import { World, Player, Resource, Mob, Structure } from "../types";
import { InputManager } from "./input";
import { Camera } from "./camera";
import { UIManager } from "./ui";

export class Renderer {
    canvas: HTMLCanvasElement;
    ctx: CanvasRenderingContext2D;
    camera: Camera;
    
    constructor() {
        this.canvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
        this.ctx = this.canvas.getContext('2d')!;
        this.camera = new Camera();
        this.resize();
        window.addEventListener('resize', () => this.resize());
    }

    resize() {
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
        this.camera.resize();
    }

    private lerp(a: number, b: number, alpha: number): number { return a + (b - a) * alpha; }

    render(world: World | null, prevWorld: World | null, alpha: number, input: InputManager, myId: string | null, ui: UIManager) {
        const zoomDelta = input.getZoomDelta();
        if (zoomDelta !== 0) this.camera.setZoom(zoomDelta);

        if (world && myId && world.players[myId]) {
            const me = world.players[myId];
            const pMe = prevWorld?.players[myId] || me;
            this.camera.follow(this.lerp(pMe.x, me.x, alpha), this.lerp(pMe.y, me.y, alpha));
        }
        this.camera.update();

        this.ctx.fillStyle = "#222";
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        this.ctx.save();
        this.ctx.translate(this.canvas.width / 2, this.canvas.height / 2);
        this.ctx.scale(this.camera.zoom, this.camera.zoom);
        this.ctx.translate(-this.camera.x, -this.camera.y);

        if (world) {
            this.drawMap(world);
            const me = myId ? world.players[myId] : null;
            for (const id in world.resources) this.drawResource(world.resources[id], me);
            for (const id in world.structures) this.drawStructure(world.structures[id], me);
            for (const id in world.mobs) {
                const m = world.mobs[id]; const pM = prevWorld?.mobs[id] || m;
                this.drawMob(m, this.lerp(pM.x, m.x, alpha), this.lerp(pM.y, m.y, alpha), me);
            }
            for (const id in world.players) {
                const p = world.players[id]; const pP = prevWorld?.players[id] || p;
                this.drawPlayer(p, this.lerp(pP.x, p.x, alpha), this.lerp(pP.y, p.y, alpha));
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

    drawMap(world: World) {
        this.ctx.fillStyle = "#3a3"; this.ctx.fillRect(0, 0, world.width, world.height);
        this.ctx.strokeStyle = "rgba(0,0,0,0.05)"; this.ctx.lineWidth = 1;
        for (let x = 0; x <= world.width; x += 100) { this.ctx.beginPath(); this.ctx.moveTo(x, 0); this.ctx.lineTo(x, world.height); this.ctx.stroke(); }
        for (let y = 0; y <= world.height; y += 100) { this.ctx.beginPath(); this.ctx.moveTo(0, y); this.ctx.lineTo(world.width, y); this.ctx.stroke(); }
    }

    drawResource(r: Resource, me: Player | null) {
        const dist = me ? Math.hypot(me.x - r.x, me.y - r.y) : 1000;
        if (dist < 60) { this.drawOutline(r.x, r.y, 22, "yellow"); this.drawInteractionTooltip(r.x, r.y, r.r_type, "[A] Gather", ""); }
        this.ctx.beginPath();
        if (r.r_type === "Tree") this.ctx.fillStyle = "#2e2"; else if (r.r_type === "Rock") this.ctx.fillStyle = "#888"; else this.ctx.fillStyle = "#ea2";
        this.ctx.arc(r.x, r.y, 20, 0, Math.PI * 2); this.ctx.fill();
        this.ctx.strokeStyle = "#000"; this.ctx.stroke();
        // HP Gauge (Resources amount is health) - Max Tree 5, Rock 10, Food 1.
        let max = r.r_type === "Tree" ? 5 : r.r_type === "Rock" ? 10 : 1;
        if (r.amount < max) this.drawGauge(r.x, r.y + 25, 30, 4, r.amount / max);
    }

    drawStructure(s: Structure, me: Player | null) {
        const dist = me ? Math.hypot(me.x - s.x, me.y - s.y) : 1000;
        if (dist < 60) { this.drawOutline(s.x, s.y, 25, "white"); this.drawInteractionTooltip(s.x, s.y, s.s_type, "[A] Attack", "Use"); }
        this.ctx.save(); this.ctx.globalAlpha = 1.0;
        if (s.s_type === "Torch") { this.ctx.fillStyle = "#fa0"; this.ctx.beginPath(); this.ctx.arc(s.x, s.y, 10, 0, Math.PI*2); this.ctx.fill(); this.ctx.strokeStyle = "#fff"; this.ctx.stroke(); }
        else if (s.s_type === "Wall") { this.ctx.fillStyle = "#642"; this.ctx.fillRect(s.x - 20, s.y - 20, 40, 40); this.ctx.strokeRect(s.x - 20, s.y - 20, 40, 40); }
        else { this.ctx.fillStyle = "#444"; this.ctx.fillRect(s.x - 25, s.y - 25, 50, 50); }
        this.ctx.restore();
        // Structure health
        let max = s.s_type === "Wall" ? 200 : s.s_type === "Door" ? 100 : 50;
        if (s.health < max) this.drawGauge(s.x, s.y + 30, 40, 4, s.health / max);
    }

    drawMob(m: Mob, ix: number, iy: number, me: Player | null) {
        const dist = me ? Math.hypot(me.x - ix, me.y - iy) : 1000;
        if (dist < 60) { this.drawOutline(ix, iy, 15, "red"); this.drawInteractionTooltip(ix, iy, m.m_type, "[A] Attack", ""); }
        this.ctx.fillStyle = m.m_type === "Wolf" ? "#999" : m.m_type === "Bear" ? "#531" : "#fff";
        this.ctx.beginPath(); this.ctx.arc(ix, iy, 12, 0, Math.PI*2); this.ctx.fill(); this.ctx.strokeStyle = "#000"; this.ctx.stroke();
        let max = m.m_type === "Wolf" ? 50 : m.m_type === "Bear" ? 200 : 10;
        if (m.health < max) this.drawGauge(ix, iy + 20, 24, 4, m.health / max);
    }

    drawPlayer(p: Player, ix: number, iy: number) {
        this.ctx.fillStyle = "#f00"; this.ctx.beginPath(); this.ctx.arc(ix, iy, 15, 0, Math.PI * 2); this.ctx.fill();
        this.ctx.strokeStyle = "#000"; this.ctx.stroke();
        this.ctx.fillStyle = "white"; this.ctx.font = "12px sans-serif"; this.ctx.textAlign = "center"; this.ctx.fillText(p.username, ix, iy - 25);
    }

    drawOutline(x: number, y: number, r: number, color: string) {
        this.ctx.beginPath(); this.ctx.arc(x, y, r, 0, Math.PI * 2);
        this.ctx.strokeStyle = color; this.ctx.lineWidth = 3; this.ctx.stroke(); this.ctx.lineWidth = 1;
    }

    drawInteractionTooltip(x: number, y: number, name: string, aAction: string, bAction: string) {
        this.ctx.fillStyle = "white"; this.ctx.font = "bold 14px sans-serif"; this.ctx.textAlign = "center";
        this.ctx.fillText(name, x, y - 45);
        this.ctx.font = "12px sans-serif"; let actions = aAction; if (bAction) actions += ` | [B] ${bAction}`;
        this.ctx.fillText(actions, x, y - 30);
    }

    drawGauge(x: number, y: number, w: number, h: number, pct: number) {
        this.ctx.fillStyle = "rgba(0,0,0,0.5)"; this.ctx.fillRect(x - w/2, y, w, h);
        this.ctx.fillStyle = "rgba(0,255,0,0.6)"; this.ctx.fillRect(x - w/2, y, w * Math.max(0, pct), h);
    }

    drawHUD(world: World, myId: string | null) {
        if (!myId || !world.players[myId]) return;
        const p = world.players[myId];
        let y = 20;
        this.drawBarWithLabel(20, y, 200, 15, p.health / 100, "#f00", "#500", "HP"); y += 20;
        this.drawBarWithLabel(20, y, 200, 15, p.hunger / 100, "#fa0", "#530", "HG"); y += 20;
        this.drawBarWithLabel(20, y, 200, 15, (100 - p.cold) / 100, "#0af", "#005", "TP");
    }

    drawBarWithLabel(x: number, y: number, w: number, h: number, pct: number, fg: string, bg: string, label: string) {
        this.ctx.fillStyle = "white"; this.ctx.font = "bold 12px sans-serif"; this.ctx.textAlign = "left"; this.ctx.textBaseline = "middle";
        this.ctx.fillText(label, x, y + h/2);
        const barX = x + 25;
        this.ctx.fillStyle = "rgba(0,0,0,0.4)"; this.ctx.fillRect(barX, y, w, h);
        this.ctx.fillStyle = fg + "99"; // Add alpha to hex
        this.ctx.fillRect(barX, y, w * Math.max(0, pct), h);
        this.ctx.strokeStyle = "rgba(0,0,0,0.5)"; this.ctx.strokeRect(barX, y, w, h);
    }

    drawUI(input: InputManager) {
        if (input.joystick.active && input.joystick.origin) {
            this.ctx.beginPath(); this.ctx.arc(input.joystick.origin.x, input.joystick.origin.y, 50, 0, Math.PI*2);
            this.ctx.strokeStyle = "rgba(255,255,255,0.2)"; this.ctx.stroke();
            this.ctx.beginPath(); this.ctx.arc(input.joystick.current.x, input.joystick.current.y, 20, 0, Math.PI*2);
            this.ctx.fillStyle = "rgba(255,255,255,0.2)"; this.ctx.fill();
        }
        const aScale = 1.0 + input.btnA.pulse * 0.3;
        this.drawButton(input.btnA.x, input.btnA.y, input.btnA.radius * aScale, "A", input.btnA.active, "rgba(211,51,51,0.6)");
        const bScale = 1.0 + input.btnB.pulse * 0.3;
        this.drawButton(input.btnB.x, input.btnB.y, input.btnB.radius * bScale, "B", input.btnB.active, "rgba(51,51,211,0.6)");
    }

    drawButton(x: number, y: number, r: number, label: string, active: boolean, color: string) {
        this.ctx.beginPath(); this.ctx.arc(x, y, r, 0, Math.PI*2);
        this.ctx.fillStyle = active ? color : "rgba(100,100,100,0.4)"; this.ctx.fill();
        this.ctx.strokeStyle = "rgba(255,255,255,0.5)"; this.ctx.stroke();
        this.ctx.fillStyle = "white"; this.ctx.font = "bold 20px sans-serif"; this.ctx.textAlign = "center";
        this.ctx.textBaseline = "middle"; this.ctx.fillText(label, x, y);
    }
}