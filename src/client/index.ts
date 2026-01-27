interface World {
    width: number;
    height: number;
    players: Record<string, any>;
    resources: Record<string, any>;
    mobs: Record<string, any>;
    structures: Record<string, any>;
    npcs: Record<string, any>;
    barrier_cores: Record<string, any>;
}

let world: World | null = null;
let myId: string | null = null;
let spawned = false;

const canvas = document.getElementById("gameCanvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;

window.addEventListener("resize", () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
});
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

const storedToken = localStorage.getItem("game_token");
const wsUrl = `ws://${window.location.host}/ws` + (storedToken ? `?token=${storedToken}` : "");
const ws = new WebSocket(wsUrl);

const keys: Record<string, boolean> = {};

window.addEventListener("keydown", (e) => {
    keys[e.code] = true;
    if (e.code === "KeyC") {
        document.getElementById("craftingMenu")!.style.display = 
            document.getElementById("craftingMenu")!.style.display === "none" ? "block" : "none";
    }
    if (e.code === "KeyE") {
        ws.send(JSON.stringify({ interact: true }));
    }
    // Hotbar selection
    if (e.code.startsWith("Digit")) {
        const num = parseInt(e.code.replace("Digit", ""));
        if (num >= 1 && num <= 7) {
            ws.send(JSON.stringify({ slot: num - 1 }));
        }
    }
});

let currentNpcId: string | null = null;

ws.onmessage = (event) => {
    const msg = JSON.parse(event.data);
    if (msg.type === "welcome") {
        myId = msg.id;
        spawned = msg.spawned;
        localStorage.setItem("game_token", msg.token);
        if (!spawned) {
            ws.send(JSON.stringify({ spawn: true }));
        }
    } else if (msg.type === "world") {
        world = msg.data;
    } else if (msg.type === "npcInteraction") {
        currentNpcId = msg.data.npc_id;
        const dialog = document.getElementById("npcDialog")!;
        dialog.style.display = "block";
        document.getElementById("npcName")!.innerText = msg.data.name;
        document.getElementById("npcText")!.innerText = msg.data.text;
        
        const opts = document.getElementById("npcOptions")!;
        opts.innerHTML = "";
        msg.data.options.forEach((opt: string, idx: number) => {
            const btn = document.createElement("button");
            btn.innerText = opt;
            btn.onclick = () => {
                ws.send(JSON.stringify({ npcAction: [currentNpcId, idx] }));
                dialog.style.display = "none";
            };
            opts.appendChild(btn);
        });
    } else if (msg.type === "questUpdate") {
        const qList = document.getElementById("questList")!;
        // Simple append/update logic
        let el = document.getElementById(`q-${msg.data.id}`);
        if (!el) {
            el = document.createElement("div");
            el.id = `q-${msg.data.id}`;
            qList.appendChild(el);
        }
        el.innerText = `${msg.data.name}: ${msg.data.state}`;
        // Add objectives detail if needed
    } else if (msg.type === "achievement") {
        const notify = document.createElement("div");
        notify.style.cssText = "position: fixed; top: 20px; right: 20px; background: #fa0; color: black; padding: 15px; border: 2px solid white; z-index: 100;";
        notify.innerHTML = `<strong>Achievement Unlocked!</strong><br>${msg.data.name}<br><small>${msg.data.description}</small>`;
        document.body.appendChild(notify);
        setTimeout(() => notify.remove(), 5000);
    }
};
window.addEventListener("keyup", (e) => keys[e.code] = false);

window.addEventListener("mousedown", (e) => {
    // Inventory Grid Interaction
    const invX = canvas.width - 220;
    const invY = 100;
    const slotS = 40;
    const cols = 5;

    if (e.clientX >= invX && e.clientX <= invX + cols * slotS && e.clientY >= invY) {
        const col = Math.floor((e.clientX - invX) / slotS);
        const row = Math.floor((e.clientY - invY) / slotS);
        const gridIndex = row * cols + col;
        const invIndex = gridIndex + 7;

        if (world && myId && world.players[myId]) {
            const me = world.players[myId];
            if (invIndex < me.inventory.slots.length) {
                // Swap with active hotbar slot
                ws.send(JSON.stringify({ swapSlots: [invIndex, me.active_slot] }));
                return;
            }
        }
    }

    ws.send(JSON.stringify({ attack: true }));
});

function craft(item: string) {
    ws.send(JSON.stringify({ craft: item }));
}
(window as any).craft = craft;

function loop() {
    // Input
    let dx = 0;
    let dy = 0;
    if (keys["KeyW"]) dy -= 1;
    if (keys["KeyS"]) dy += 1;
    if (keys["KeyA"]) dx -= 1;
    if (keys["KeyD"]) dx += 1;

    if (dx !== 0 || dy !== 0) {
        ws.send(JSON.stringify({ dx, dy }));
    }

    // Render
    ctx.fillStyle = "#222";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    if (world) {
        const me = myId ? world.players[myId] : null;
        const camX = me ? me.x - canvas.width / 2 : 0;
        const camY = me ? me.y - canvas.height / 2 : 0;

        ctx.save();
        ctx.translate(-camX, -camY);

        // Draw barriers
        if (world.barrier_cores) {
            for (const b of Object.values(world.barrier_cores)) {
                const range = b.base_range + ((b.level - 1) * 50); // Hardcoded multiplier for vis
                ctx.strokeStyle = "rgba(100, 100, 255, 0.3)";
                ctx.lineWidth = 2;
                ctx.beginPath();
                ctx.arc(b.x, b.y, range, 0, Math.PI * 2);
                ctx.stroke();
                
                ctx.fillStyle = "#aaf";
                ctx.fillRect(b.x - 5, b.y - 5, 10, 10);
            }
        }

        // Draw NPCs
        if (world.npcs) {
            for (const n of Object.values(world.npcs)) {
                ctx.fillStyle = "#fa0";
                ctx.beginPath();
                ctx.arc(n.x, n.y, 12, 0, Math.PI * 2);
                ctx.fill();
                ctx.fillStyle = "white";
                ctx.font = "12px Arial";
                ctx.fillText(n.name, n.x - 15, n.y - 15);
            }
        }

        // Draw resources
        for (const r of Object.values(world.resources)) {
            ctx.fillStyle = r.r_type === "Tree" ? "#2a2" : (r.r_type === "Rock" ? "#777" : "#a22");
            ctx.beginPath();
            ctx.arc(r.x, r.y, 15, 0, Math.PI * 2);
            ctx.fill();
        }

        // Draw mobs
        for (const m of Object.values(world.mobs)) {
            ctx.fillStyle = m.m_type === "Rabbit" ? "#fff" : (m.m_type === "Wolf" ? "#555" : "#422");
            ctx.fillRect(m.x - 10, m.y - 10, 20, 20);
            // Draw Level
            ctx.fillStyle = "white";
            ctx.font = "10px Arial";
            ctx.fillText(`Lvl ${m.level}`, m.x - 10, m.y - 15);
        }

        // Draw players
        for (const p of Object.values(world.players)) {
            ctx.fillStyle = p.id === myId ? "#3af" : "#f3a";
            ctx.beginPath();
            ctx.arc(p.x, p.y, 20, 0, Math.PI * 2);
            ctx.fill();
            ctx.fillStyle = "white";
            ctx.fillText(p.username, p.x - 20, p.y - 30);
        }

        // Draw Effects
        if (world.effects) {
            for (const e of Object.values(world.effects)) {
                ctx.fillStyle = e.color;
                ctx.globalAlpha = e.ttl / 20;
                ctx.font = "bold 16px Arial";
                ctx.fillText(e.text, e.x, e.y);
                ctx.globalAlpha = 1.0;
            }
        }

        ctx.restore();

        // UI
        if (me) {
            // HUD Bars
            const barWidth = 200;
            const barHeight = 20;
            const startY = 20;
            
            // Health
            ctx.fillStyle = "#500";
            ctx.fillRect(20, startY, barWidth, barHeight);
            ctx.fillStyle = "#f00";
            ctx.fillRect(20, startY, barWidth * (me.health / 100), barHeight);
            ctx.strokeStyle = "white";
            ctx.lineWidth = 1;
            ctx.strokeRect(20, startY, barWidth, barHeight);
            ctx.fillStyle = "white";
            ctx.font = "12px Arial";
            ctx.fillText(`Health: ${Math.round(me.health)}`, 25, startY + 14);

            // Hunger
            ctx.fillStyle = "#530";
            ctx.fillRect(20, startY + 30, barWidth, barHeight);
            ctx.fillStyle = "#fa0";
            ctx.fillRect(20, startY + 30, barWidth * (me.hunger / 100), barHeight);
            ctx.strokeRect(20, startY + 30, barWidth, barHeight);
            ctx.fillStyle = "white";
            ctx.fillText(`Hunger: ${Math.round(me.hunger)}`, 25, startY + 44);

            // Inventory Grid (Key I to toggle?) - For now always visible on side if big screen
            // Actually let's just make a toggleable overlay or keep drawing on side
            const invX = canvas.width - 220;
            const invY = 100;
            const cols = 5;
            const slotS = 40;
            
            ctx.fillStyle = "rgba(0,0,0,0.8)";
            ctx.fillRect(invX - 10, invY - 30, 220, 300);
            ctx.fillStyle = "white";
            ctx.font = "16px Arial";
            ctx.fillText("Inventory (Click to swap)", invX, invY - 10);

            if (me.inventory && me.inventory.slots) {
                me.inventory.slots.forEach((slot: any, i: number) => {
                    if (i < 7) return; // Skip hotbar
                    
                    const gridIndex = i - 7;
                    const col = gridIndex % cols;
                    const row = Math.floor(gridIndex / cols);
                    
                    const x = invX + col * slotS;
                    const y = invY + row * slotS;
                    
                    ctx.strokeStyle = "#777";
                    ctx.strokeRect(x, y, slotS, slotS);
                    
                    if (slot) {
                        ctx.fillStyle = "white";
                        ctx.font = "10px Arial";
                        // Draw icon placeholder (first letter)
                        ctx.fillStyle = "#aaa";
                        ctx.fillRect(x+2, y+2, slotS-4, slotS-4);
                        
                        ctx.fillStyle = "black";
                        ctx.fillText(slot.kind.substring(0, 2), x + 5, y + 20);
                        
                        ctx.fillStyle = "white";
                        ctx.fillText(`${slot.amount}`, x + 2, y + 35);
                    }
                });
            }

            // Render Hotbar
            const slotSize = 50;
            const startX = canvas.width / 2 - (slotSize * 7) / 2;
            const hotbarY = canvas.height - 70;

            for (let i = 0; i < 7; i++) {
                ctx.strokeStyle = i === me.active_slot ? "yellow" : "#555";
                ctx.lineWidth = i === me.active_slot ? 3 : 1;
                ctx.fillStyle = "rgba(0,0,0,0.5)";
                ctx.fillRect(startX + i * slotSize, hotbarY, slotSize, slotSize);
                ctx.strokeRect(startX + i * slotSize, hotbarY, slotSize, slotSize);

                const slotItem = me.inventory.slots[i];
                if (slotItem) {
                    ctx.fillStyle = "white";
                    ctx.font = "10px Arial";
                    ctx.fillText(slotItem.kind.substring(0, 5), startX + i * slotSize + 5, hotbarY + 20);
                    ctx.fillText(`x${slotItem.amount}`, startX + i * slotSize + 5, hotbarY + 40);
                }
                ctx.fillStyle = "white";
                ctx.font = "12px Arial";
                ctx.fillText((i + 1).toString(), startX + i * slotSize + 2, hotbarY + 12);
            }
        }
    }

    requestAnimationFrame(loop);
}

loop();