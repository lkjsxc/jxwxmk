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

    private lerp(a: number, b: number, alpha: number): number {
        return a + (b - a) * alpha;
    }

    render(world: World | null, prevWorld: World | null, alpha: number, input: InputManager, myId: string | null, ui: UIManager) {
        const zoomDelta = input.getZoomDelta();
        if (zoomDelta !== 0) this.camera.setZoom(zoomDelta);

        // Camera Follow (Interpolated)
        if (world && myId && world.players[myId]) {
            const me = world.players[myId];
            const pMe = prevWorld?.players[myId] || me;
            const ix = this.lerp(pMe.x, me.x, alpha);
            const iy = this.lerp(pMe.y, me.y, alpha);
            this.camera.follow(ix, iy);
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

            // Draw Resources
            for (const id in world.resources) {
                const r = world.resources[id];
                this.drawResource(r, me);
            }

            // Draw Structures
            for (const id in world.structures) {
                const s = world.structures[id];
                this.drawStructure(s, me);
            }

            // Draw Mobs
            for (const id in world.mobs) {
                const m = world.mobs[id];
                const pM = prevWorld?.mobs[id] || m;
                const ix = this.lerp(pM.x, m.x, alpha);
                const iy = this.lerp(pM.y, m.y, alpha);
                this.drawMob(m, ix, iy, me);
            }

            // Draw Players
            for (const id in world.players) {
                const p = world.players[id];
                const pP = prevWorld?.players[id] || p;
                const ix = this.lerp(pP.x, p.x, alpha);
                const iy = this.lerp(pP.y, p.y, alpha);
                this.drawPlayer(p, ix, iy);
            }
        }
        
        this.ctx.restore();

        if (!world) {
            this.ctx.fillStyle = "#fff";
            this.ctx.font = "20px sans-serif";
            this.ctx.fillText("Connecting...", 50, 50);
        } else {
            this.drawUI(input);
            this.drawHUD(world, myId);
            if (myId && world.players[myId]) {
                 ui.render(this.ctx, world.players[myId], input);
            }
        }
    }

    drawMap(world: World) {
        this.ctx.fillStyle = "#3a3";
        this.ctx.fillRect(0, 0, world.width, world.height);
        this.ctx.strokeStyle = "rgba(0,0,0,0.1)";
        this.ctx.lineWidth = 1;
        for (let x = 0; x <= world.width; x += 100) {
            this.ctx.beginPath(); this.ctx.moveTo(x, 0); this.ctx.lineTo(x, world.height); this.ctx.stroke();
        }
        for (let y = 0; y <= world.height; y += 100) {
            this.ctx.beginPath(); this.ctx.moveTo(0, y); this.ctx.lineTo(world.width, y); this.ctx.stroke();
        }
    }

    drawResource(r: Resource, me: Player | null) {
        const dist = me ? Math.hypot(me.x - r.x, me.y - r.y) : 1000;
        if (dist < 60) this.drawOutline(r.x, r.y, 22, "yellow");

        this.ctx.beginPath();
        if (r.r_type === "Tree") this.ctx.fillStyle = "#2e2";
        else if (r.r_type === "Rock") this.ctx.fillStyle = "#888";
        else this.ctx.fillStyle = "#ea2";
        this.ctx.arc(r.x, r.y, 20, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.strokeStyle = "#000";
        this.ctx.stroke();
    }

    drawStructure(s: Structure, me: Player | null) {
        const dist = me ? Math.hypot(me.x - s.x, me.y - s.y) : 1000;
        if (dist < 60) this.drawOutline(s.x, s.y, 25, "white");

        this.ctx.save();
        this.ctx.globalAlpha = 1.0; // Force Opaque
        if (s.s_type === "Torch") {
            this.ctx.fillStyle = "#fa0";
            this.ctx.beginPath(); this.ctx.arc(s.x, s.y, 10, 0, Math.PI*2); this.ctx.fill();
            this.ctx.strokeStyle = "#fff"; this.ctx.stroke();
        } else if (s.s_type === "Wall") {
            this.ctx.fillStyle = "#642";
            this.ctx.fillRect(s.x - 20, s.y - 20, 40, 40);
            this.ctx.strokeRect(s.x - 20, s.y - 20, 40, 40);
        } else {
            this.ctx.fillStyle = "#444";
            this.ctx.fillRect(s.x - 25, s.y - 25, 50, 50);
        }
        this.ctx.restore();
    }

    drawMob(m: Mob, ix: number, iy: number, me: Player | null) {
        const dist = me ? Math.hypot(me.x - ix, me.y - iy) : 1000;
        if (dist < 60) this.drawOutline(ix, iy, 15, "red");

        this.ctx.fillStyle = m.m_type === "Wolf" ? "#999" : m.m_type === "Bear" ? "#531" : "#fff";
        this.ctx.beginPath(); this.ctx.arc(ix, iy, 12, 0, Math.PI*2); this.ctx.fill();
        this.ctx.strokeStyle = "#000"; this.ctx.stroke();
    }

    drawPlayer(p: Player, ix: number, iy: number) {
        this.ctx.fillStyle = "#f00";
        this.ctx.beginPath(); this.ctx.arc(ix, iy, 15, 0, Math.PI * 2); this.ctx.fill();
        this.ctx.strokeStyle = "#000"; this.ctx.stroke();
        this.ctx.fillStyle = "white"; this.ctx.font = "12px sans-serif"; this.ctx.textAlign = "center";
        this.ctx.fillText(p.username, ix, iy - 25);
    }

    drawOutline(x: number, y: number, r: number, color: string) {
        this.ctx.beginPath();
        this.ctx.arc(x, y, r, 0, Math.PI * 2);
        this.ctx.strokeStyle = color;
        this.ctx.lineWidth = 3;
        this.ctx.stroke();
        this.ctx.lineWidth = 1;
    }

    drawHUD(world: World, myId: string | null) {
        if (!myId || !world.players[myId]) return;
        const p = world.players[myId];
        let y = 20;
        this.drawBar(20, y, 200, 15, p.health / 100, "#f00", "#500"); y += 20;
        this.drawBar(20, y, 200, 15, p.hunger / 100, "#fa0", "#530"); y += 20;
        this.drawBar(20, y, 200, 15, (100 - p.cold) / 100, "#0af", "#005");
    }

    drawBar(x: number, y: number, w: number, h: number, pct: number, fg: string, bg: string) {
        this.ctx.fillStyle = bg; this.ctx.fillRect(x, y, w, h);
        this.ctx.fillStyle = fg; this.ctx.fillRect(x, y, w * Math.max(0, pct), h);
        this.ctx.strokeStyle = "black"; this.ctx.strokeRect(x, y, w, h);
    }

    drawUI(input: InputManager) {
        if (input.joystick.active && input.joystick.origin) {
            this.ctx.beginPath(); this.ctx.arc(input.joystick.origin.x, input.joystick.origin.y, 50, 0, Math.PI*2);
            this.ctx.strokeStyle = "rgba(255,255,255,0.3)"; this.ctx.stroke();
            this.ctx.beginPath(); this.ctx.arc(input.joystick.current.x, input.joystick.current.y, 20, 0, Math.PI*2);
            this.ctx.fillStyle = "rgba(255,255,255,0.3)"; this.ctx.fill();
        }
        this.drawButton(input.btnA.x, input.btnA.y, input.btnA.radius, "A", input.btnA.active, "#d33");
        this.drawButton(input.btnB.x, input.btnB.y, input.btnB.radius, "B", input.btnB.active, "#33d");
    }

    drawButton(x: number, y: number, r: number, label: string, active: boolean, color: string) {
        this.ctx.beginPath(); this.ctx.arc(x, y, r, 0, Math.PI*2);
        this.ctx.fillStyle = active ? color : "rgba(100,100,100,0.5)"; this.ctx.fill();
        this.ctx.strokeStyle = "white"; this.ctx.stroke();
        this.ctx.fillStyle = "white"; this.ctx.font = "bold 20px sans-serif"; this.ctx.textAlign = "center";
        this.ctx.textBaseline = "middle"; this.ctx.fillText(label, x, y);
    }
}
