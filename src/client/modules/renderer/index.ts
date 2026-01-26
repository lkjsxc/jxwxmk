import { World, Player, AppState } from "../../types";
import { InputManager } from "../input";
import { Camera } from "../camera";
import { UIManager } from "../ui/index";
import { drawMap, lerp, getScale } from "./utils";
import { drawResource, drawStructure, drawMob, drawPlayer } from "./entities";

export class Renderer {
    canvas: HTMLCanvasElement; ctx: CanvasRenderingContext2D; camera: Camera;

    constructor() {
        this.canvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
        this.ctx = this.canvas.getContext('2d')!;
        this.camera = new Camera();
        this.resize();
        window.addEventListener('resize', () => this.resize());
    }

    resize() { 
        const dpr = window.devicePixelRatio || 1;
        this.canvas.width = window.innerWidth * dpr; 
        this.canvas.height = window.innerHeight * dpr; 
        this.canvas.style.width = `${window.innerWidth}px`;
        this.canvas.style.height = `${window.innerHeight}px`;
        this.ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
        this.camera.resize(); 
    }

    render(world: World | null, prevWorld: World | null, alpha: number, input: InputManager, myId: string | null, ui: UIManager) {
        const dpr = window.devicePixelRatio || 1;
        const w = this.canvas.width / dpr;
        const h = this.canvas.height / dpr;

        if (!this.camera.initialized && world) {
            this.camera.snap(Math.random() * world.width, Math.random() * world.height);
            this.camera.initialized = true;
        }

        const zoomDelta = input.getZoomDelta();
        if (zoomDelta !== 0) this.camera.setZoom(zoomDelta);

        if (world && myId && world.players[myId] && world.players[myId].spawned) {
            const me = world.players[myId]; const pMe = prevWorld?.players[myId] || me;
            const tx = lerp(pMe.x, me.x, alpha);
            const ty = lerp(pMe.y, me.y, alpha);
            
            // Snap if just spawned
            if (!prevWorld?.players[myId]?.spawned) {
                this.camera.snap(tx, ty);
            } else {
                this.camera.follow(tx, ty);
            }
        }
        this.camera.update();

        this.ctx.fillStyle = "#222"; this.ctx.fillRect(0, 0, w, h);
        this.ctx.save();
        this.ctx.translate(w / 2, h / 2);
        this.ctx.scale(this.camera.zoom, this.camera.zoom);
        this.ctx.translate(-this.camera.x, -this.camera.y);

        if (world) {
            drawMap(this.ctx, world);
            const me = myId ? world.players[myId] : null;
            const targetId = this.findClosestTarget(world, me);

            for (const id in world.resources) drawResource(this.ctx, world.resources[id], id === targetId, getScale(id, world, prevWorld));
            for (const id in world.structures) drawStructure(this.ctx, world.structures[id], id === targetId, getScale(id, world, prevWorld));
            for (const id in world.mobs) {
                const m = world.mobs[id]; const pM = prevWorld?.mobs[id] || m;
                drawMob(this.ctx, m, lerp(pM.x, m.x, alpha), lerp(pM.y, m.y, alpha), id === targetId, getScale(id, world, prevWorld));
            }
            for (const id in world.players) {
                const p = world.players[id]; const pP = prevWorld?.players[id] || p;
                drawPlayer(this.ctx, p, lerp(pP.x, p.x, alpha), lerp(pP.y, p.y, alpha), id === targetId, getScale(id, world, prevWorld));
            }
        }
        this.ctx.restore();

        if (!world) {
            this.ctx.fillStyle = "#fff"; 
            this.ctx.font = "bold 24px sans-serif"; 
            this.ctx.textAlign = "center";
            this.ctx.fillText("Connecting...", w / 2, h / 2);
        } else {
            if (ui.state === AppState.InGame) {
                this.drawUI(input);
                this.drawHUD(world, myId);
            }
            // Allow UI to render its overlays (StartScreen, GameOver, Menus)
            // ui.render handles state checking internally for what to draw
            if (myId && world.players[myId]) ui.render(this.ctx, world.players[myId], input);
            else ui.render(this.ctx, null, input);
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

    drawHUD(world: World, myId: string | null) {
        if (!myId || !world.players[myId]) return;
        const p = world.players[myId];
        const dpr = window.devicePixelRatio || 1;
        const w = this.canvas.width / dpr;

        // Mobile optimization: larger bars, better spacing
        const isMobile = w < 600;
        const barW = isMobile ? Math.min(180, w * 0.4) : 200;
        const barH = isMobile ? 18 : 15;
        const spacing = isMobile ? 24 : 20;
        
        let x = isMobile ? 10 : 20;
        let y = isMobile ? 10 : 20;

        this.drawBarWithLabel(x, y, barW, barH, p.health / 100, "rgba(255,0,0,0.6)", "rgba(80,0,0,0.3)", "HP"); y += spacing;
        this.drawBarWithLabel(x, y, barW, barH, p.hunger / 100, "rgba(255,165,0,0.6)", "rgba(80,40,0,0.3)", "HG"); y += spacing;
        this.drawBarWithLabel(x, y, barW, barH, (100 - p.cold) / 100, "rgba(0,170,255,0.6)", "rgba(0,40,80,0.3)", "TP");
    }

    drawBarWithLabel(x: number, y: number, w: number, h: number, pct: number, fg: string, bg: string, label: string) {
        this.ctx.fillStyle = "white"; this.ctx.font = "bold 12px sans-serif"; this.ctx.textAlign = "left"; this.ctx.textBaseline = "middle";
        this.ctx.fillText(label, x, y + h/2);
        const barX = x + 25; this.ctx.fillStyle = bg; this.ctx.fillRect(barX, y, w, h);
        this.ctx.fillStyle = fg; this.ctx.fillRect(barX, y, w * Math.max(0, pct), h);
        this.ctx.strokeStyle = "rgba(255,255,255,0.2)"; this.ctx.strokeRect(barX, y, w, h);
    }

    drawUI(input: InputManager) {
        const now = Date.now();

        // Draw Joystick
        if (input.joystick.active && input.joystick.origin) {
            const { origin, current } = input.joystick;
            this.ctx.save();
            // Outer circle
            this.ctx.beginPath();
            this.ctx.arc(origin.x, origin.y, 50, 0, Math.PI * 2);
            this.ctx.fillStyle = "rgba(255,255,255,0.1)";
            this.ctx.fill();
            this.ctx.strokeStyle = "rgba(255,255,255,0.3)";
            this.ctx.lineWidth = 2;
            this.ctx.stroke();

            // Inner stick
            this.ctx.beginPath();
            this.ctx.arc(current.x, current.y, 25, 0, Math.PI * 2);
            this.ctx.fillStyle = "rgba(255,255,255,0.4)";
            this.ctx.fill();
            this.ctx.strokeStyle = "rgba(255,255,255,0.6)";
            this.ctx.stroke();
            this.ctx.restore();
        }

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
