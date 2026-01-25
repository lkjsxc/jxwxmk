import { World, Player } from "../types";
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

    render(world: World | null, input: InputManager, myId: string | null, ui: UIManager) {
        // Handle Zoom Input
        const zoomDelta = input.getZoomDelta();
        if (zoomDelta !== 0) {
            this.camera.setZoom(zoomDelta);
        }

        // Camera Follow
        if (world && myId && world.players[myId]) {
            const me = world.players[myId];
            this.camera.follow(me.x, me.y);
        }
        this.camera.update();

        // Clear Screen
        this.ctx.fillStyle = "#222"; // Background (Fog of war?)
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        // --- World Space ---
        this.ctx.save();
        
        // Center Camera: translate to center, scale, translate back
        this.ctx.translate(this.canvas.width / 2, this.canvas.height / 2);
        this.ctx.scale(this.camera.zoom, this.camera.zoom);
        this.ctx.translate(-this.camera.x, -this.camera.y);

        // Draw Map Background (Simple Grid or Rect)
        if (world) {
            this.ctx.fillStyle = "#3a3";
            this.ctx.fillRect(0, 0, world.width, world.height);
            
            // Grid Lines
            this.ctx.strokeStyle = "rgba(0,0,0,0.1)";
            this.ctx.lineWidth = 1;
            const gridSize = 100;
            this.ctx.beginPath();
            for (let x = 0; x <= world.width; x += gridSize) {
                this.ctx.moveTo(x, 0);
                this.ctx.lineTo(x, world.height);
            }
            for (let y = 0; y <= world.height; y += gridSize) {
                this.ctx.moveTo(0, y);
                this.ctx.lineTo(world.width, y);
            }
            this.ctx.stroke();
        }

        if (world) {
            // Draw Resources
            for (const id in world.resources) {
                const r = world.resources[id];
                this.ctx.beginPath();
                if (r.r_type === "Tree") {
                    this.ctx.fillStyle = "#2e2";
                    this.ctx.arc(r.x, r.y, 20, 0, Math.PI * 2);
                } else if (r.r_type === "Rock") {
                    this.ctx.fillStyle = "#888";
                    this.ctx.arc(r.x, r.y, 15, 0, Math.PI * 2);
                } else {
                    this.ctx.fillStyle = "#ea2";
                    this.ctx.arc(r.x, r.y, 10, 0, Math.PI * 2);
                }
                this.ctx.fill();
                this.ctx.stroke();
            }

            // Draw Mobs
            for (const id in world.mobs) {
                const m = world.mobs[id];
                this.ctx.fillStyle = m.m_type === "Wolf" ? "#999" : m.m_type === "Bear" ? "#531" : "#fff";
                this.ctx.beginPath();
                this.ctx.arc(m.x, m.y, 12, 0, Math.PI*2);
                this.ctx.fill();
                this.ctx.stroke();
            }

            // Draw Players
            for (const id in world.players) {
                this.drawPlayer(world.players[id]);
            }
        }
        
        this.ctx.restore();
        // --- End World Space ---

        // --- UI Space ---
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

    drawPlayer(p: Player) {
        this.ctx.fillStyle = "#f00";
        this.ctx.beginPath();
        this.ctx.arc(p.x, p.y, 15, 0, Math.PI * 2);
        this.ctx.fill();
        this.ctx.stroke();

        // Name
        this.ctx.fillStyle = "white";
        this.ctx.font = "12px sans-serif";
        this.ctx.textAlign = "center";
        this.ctx.fillText(p.username, p.x, p.y - 25);
    }

    drawHUD(world: World, myId: string | null) {
        if (!myId || !world.players[myId]) return;
        const p = world.players[myId];

        // Vitals (Top Left)
        const x = 20;
        let y = 20;
        const w = 200;
        const h = 15;

        this.drawBar(x, y, w, h, p.health / 100, "#f00", "#500"); y += 20;
        this.drawBar(x, y, w, h, p.hunger / 100, "#fa0", "#530"); y += 20;
        this.drawBar(x, y, w, h, (100 - p.cold) / 100, "#0af", "#005"); // Invert cold for visual (Full bar = Warm)
    }

    drawBar(x: number, y: number, w: number, h: number, pct: number, fg: string, bg: string) {
        this.ctx.fillStyle = bg;
        this.ctx.fillRect(x, y, w, h);
        this.ctx.fillStyle = fg;
        this.ctx.fillRect(x, y, w * Math.max(0, pct), h);
        this.ctx.strokeStyle = "black";
        this.ctx.lineWidth = 1;
        this.ctx.strokeRect(x, y, w, h);
    }

    drawUI(input: InputManager) {
        // Joystick
        if (input.joystick.active && input.joystick.origin) {
            this.ctx.beginPath();
            this.ctx.arc(input.joystick.origin.x, input.joystick.origin.y, 50, 0, Math.PI*2);
            this.ctx.strokeStyle = "rgba(255,255,255,0.5)";
            this.ctx.lineWidth = 2;
            this.ctx.stroke();

            this.ctx.beginPath();
            this.ctx.arc(input.joystick.current.x, input.joystick.current.y, 20, 0, Math.PI*2);
            this.ctx.fillStyle = "rgba(255,255,255,0.5)";
            this.ctx.fill();
        }

        // Buttons
        this.drawButton(input.btnA.x, input.btnA.y, input.btnA.radius, "A", input.btnA.active, "#d33");
        this.drawButton(input.btnB.x, input.btnB.y, input.btnB.radius, "B", input.btnB.active, "#33d");
    }

    drawButton(x: number, y: number, r: number, label: string, active: boolean, color: string) {
        this.ctx.beginPath();
        this.ctx.arc(x, y, r, 0, Math.PI*2);
        this.ctx.fillStyle = active ? color : "rgba(100,100,100,0.5)";
        this.ctx.fill();
        this.ctx.strokeStyle = "white";
        this.ctx.lineWidth = 2;
        this.ctx.stroke();
        
        this.ctx.fillStyle = "white";
        this.ctx.font = "bold 20px sans-serif";
        this.ctx.textAlign = "center";
        this.ctx.textBaseline = "middle";
        this.ctx.fillText(label, x, y);
    }
}