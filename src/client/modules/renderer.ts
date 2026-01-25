import { World, Player } from "../types";
import { InputManager } from "./input";

export class Renderer {
    canvas: HTMLCanvasElement;
    ctx: CanvasRenderingContext2D;
    
    constructor() {
        this.canvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
        this.ctx = this.canvas.getContext('2d')!;
        this.resize();
        window.addEventListener('resize', () => this.resize());
    }

    resize() {
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
    }

    render(world: World | null, input: InputManager) {
        // Clear
        this.ctx.fillStyle = "#3a3"; // Biome color (Forest)
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        if (!world) {
            this.ctx.fillStyle = "#fff";
            this.ctx.fillText("Connecting...", 50, 50);
            return;
        }

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

        // Draw Players
        for (const id in world.players) {
            this.drawPlayer(world.players[id]);
        }

        // Draw UI
        this.drawUI(input);
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

        // Bars (HP, Hunger, Cold)
        const barW = 30;
        const barH = 4;
        const yOff = -20;
        
        // HP (Red)
        this.ctx.fillStyle = "#500";
        this.ctx.fillRect(p.x - barW/2, p.y + yOff, barW, barH);
        this.ctx.fillStyle = "#f00";
        this.ctx.fillRect(p.x - barW/2, p.y + yOff, barW * (p.health / 100), barH);

        // Hunger (Orange)
        this.ctx.fillStyle = "#530";
        this.ctx.fillRect(p.x - barW/2, p.y + yOff + 5, barW, barH);
        this.ctx.fillStyle = "#fa0";
        this.ctx.fillRect(p.x - barW/2, p.y + yOff + 5, barW * (p.hunger / 100), barH);

        // Cold (Blue)
        this.ctx.fillStyle = "#005";
        this.ctx.fillRect(p.x - barW/2, p.y + yOff + 10, barW, barH);
        this.ctx.fillStyle = "#0af";
        this.ctx.fillRect(p.x - barW/2, p.y + yOff + 10, barW * (1 - p.cold / 100), barH); 
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

        // Attack Button
        this.ctx.beginPath();
        this.ctx.arc(input.attackBtn.x, input.attackBtn.y, input.attackBtn.radius, 0, Math.PI*2);
        this.ctx.fillStyle = input.attackBtn.active ? "rgba(200,50,50,0.8)" : "rgba(200,50,50,0.5)";
        this.ctx.fill();
        this.ctx.strokeStyle = "white";
        this.ctx.lineWidth = 2;
        this.ctx.stroke();
    }
}
