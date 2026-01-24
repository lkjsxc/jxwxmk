const canvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
const ctx = canvas.getContext('2d');

if (!ctx) throw new Error("No 2D context");

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

window.addEventListener('resize', () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
});

interface Inventory {
    wood: number;
    stone: number;
    food: number;
}

interface Player {
    id: string;
    username: string;
    x: number;
    y: number;
    inventory: Inventory;
}

interface Resource {
    id: string;
    r_type: "Tree" | "Rock" | "Food";
    x: number;
    y: number;
    amount: number;
}

interface World {
    width: number;
    height: number;
    players: Record<string, Player>;
    resources: Record<string, Resource>;
}

let world: World | null = null;
let myId: string | null = null; // Need to know who I am, but server doesn't tell yet. 
// For now, assume we just render all.

const protocol = location.protocol === 'https:' ? 'wss' : 'ws';
let ws = new WebSocket(`${protocol}://${location.host}/ws`);

ws.onmessage = (event) => {
    try {
        world = JSON.parse(event.data);
    } catch (e) {
        console.error("Parse error", e);
    }
};

const keys = {
    w: false, a: false, s: false, d: false, attack: false
};

window.addEventListener('keydown', (e) => {
    const k = e.key.toLowerCase();
    if (keys.hasOwnProperty(k)) keys[k as keyof typeof keys] = true;
});

window.addEventListener('keyup', (e) => {
    const k = e.key.toLowerCase();
    if (keys.hasOwnProperty(k)) keys[k as keyof typeof keys] = false;
});

window.addEventListener('mousedown', () => { keys.attack = true; });
window.addEventListener('mouseup', () => { keys.attack = false; });

function sendInput() {
    if (ws.readyState === WebSocket.OPEN) {
        let dx = 0;
        let dy = 0;
        if (keys.w) dy -= 1;
        if (keys.s) dy += 1;
        if (keys.a) dx -= 1;
        if (keys.d) dx += 1;

        if (dx !== 0 && dy !== 0) {
            dx *= 0.707;
            dy *= 0.707;
        }
        
        // Always send if moving or attacking
        if (dx !== 0 || dy !== 0 || keys.attack) {
            ws.send(JSON.stringify({ dx, dy, attack: keys.attack }));
        }
    }
}

function render() {
    ctx!.fillStyle = "#3a3";
    ctx!.fillRect(0, 0, canvas.width, canvas.height);

    if (world) {
        // Draw Resources
        for (const id in world.resources) {
            const r = world.resources[id];
            ctx!.beginPath();
            if (r.r_type === "Tree") {
                ctx!.fillStyle = "#2e2";
                ctx!.arc(r.x, r.y, 20, 0, Math.PI * 2);
            } else if (r.r_type === "Rock") {
                ctx!.fillStyle = "#888";
                ctx!.arc(r.x, r.y, 15, 0, Math.PI * 2);
            } else {
                ctx!.fillStyle = "#ea2";
                ctx!.arc(r.x, r.y, 10, 0, Math.PI * 2);
            }
            ctx!.fill();
            ctx!.strokeStyle = "#000";
            ctx!.lineWidth = 1;
            ctx!.stroke();
        }

        // Draw players
        for (const id in world.players) {
            const p = world.players[id];
            
            ctx!.fillStyle = "#f00";
            ctx!.beginPath();
            ctx!.arc(p.x, p.y, 15, 0, Math.PI * 2);
            ctx!.fill();
            ctx!.lineWidth = 2;
            ctx!.strokeStyle = "#000";
            ctx!.stroke();
            
            ctx!.fillStyle = "white";
            ctx!.font = "12px sans-serif";
            ctx!.textAlign = "center";
            ctx!.fillText(p.username, p.x, p.y - 20);

            // Inventory (Hack to show for everyone for debug)
            ctx!.fillStyle = "rgba(0,0,0,0.5)";
            ctx!.fillText(`W:${p.inventory.wood} S:${p.inventory.stone} F:${p.inventory.food}`, p.x, p.y + 30);
        }
    } else {
        ctx!.fillStyle = "#000";
        ctx!.fillText("Connecting...", 50, 50);
    }

    requestAnimationFrame(render);
}

setInterval(sendInput, 50);
render();
