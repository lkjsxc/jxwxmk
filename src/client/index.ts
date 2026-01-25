import { InputManager } from "./modules/input";
import { Renderer } from "./modules/renderer/index";
import { UIManager, AppState } from "./modules/ui/index";
import { World } from "./types";

const renderer = new Renderer();
const input = new InputManager();
const ui = new UIManager();

let world: World | null = null;
let prevWorld: World | null = null;
let lastUpdateAt: number = 0;
let myId: string | null = null;
let ws: WebSocket | null = null;
let inputInterval: any = null;
const STORAGE_KEY = "kkmypk_token";

function connect() {
    const protocol = location.protocol === 'https:' ? 'wss' : 'ws';
    const token = localStorage.getItem(STORAGE_KEY);
    const url = `${protocol}://${location.host}/ws${token ? `?token=${token}` : ''}`;
    ws = new WebSocket(url);
    ws.onmessage = (event) => {
        try {
            const msg = JSON.parse(event.data);
            if (msg.type === "welcome") {
                myId = msg.id;
                if (msg.token) localStorage.setItem(STORAGE_KEY, msg.token);
                // Keep state as StartScreen until player clicks Play
            } else if (msg.type === "world") {
                 prevWorld = world; world = msg.data; lastUpdateAt = Date.now();
                 if (ui.state === AppState.InGame && myId && world && !world.players[myId]) { ui.state = AppState.GameOver; }
            } else if (msg.type === "achievement") {
                ui.showAchievement(msg.data);
            }
        } catch (e) { console.error("Parse error", e); }
    };
    ws.onopen = () => {
        console.log("Connected");
        if (inputInterval) clearInterval(inputInterval);
        inputInterval = setInterval(sendInput, 50);
    };
    ws.onclose = () => {
        ui.state = AppState.StartScreen; world = null; myId = null;
        if (inputInterval) { clearInterval(inputInterval); inputInterval = null; }
    };
}

function loop() {
    const player = myId && world ? world.players[myId] : null;
    ui.handleInput(input, renderer.canvas.width, renderer.canvas.height, player);
    input.updateAnimations(16);

    const now = Date.now();
    const alpha = Math.min(1.0, (now - lastUpdateAt) / 50); 
    renderer.render(world, prevWorld, alpha, input, myId, ui);

    if (ui.state === AppState.StartScreen) {
        if (ui.joinRequest) { 
            if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ spawn: true }));
            ui.state = AppState.InGame; 
            ui.joinRequest = false; 
        }
    } else if (ui.state === AppState.GameOver) {
        if (ui.respawnRequest) {
            ui.respawnRequest = false; if (ws) { ws.close(); ws = null; }
            if (inputInterval) { clearInterval(inputInterval); inputInterval = null; }
            localStorage.removeItem(STORAGE_KEY); location.reload();
        }
    } else {
        if (ws && ws.readyState === WebSocket.OPEN) {
            if (ui.craftRequest) { ws.send(JSON.stringify({ craft: ui.craftRequest })); ui.craftRequest = null; }
            if (ui.slotSelectRequest !== null) { ws.send(JSON.stringify({ slot: ui.slotSelectRequest })); ui.slotSelectRequest = null; }
            if (ui.nameUpdateRequest) { ws.send(JSON.stringify({ name: ui.nameUpdateRequest })); ui.nameUpdateRequest = null; }
            if (ui.swapRequest) { ws.send(JSON.stringify({ swapSlots: ui.swapRequest })); ui.swapRequest = null; }
        }
    }
    requestAnimationFrame(loop);
}

function sendInput() {
    if (ws && ws.readyState === WebSocket.OPEN && ui.state === AppState.InGame) {
        const state = input.getState();
        if (state.dx !== 0 || state.dy !== 0 || state.attack || state.interact || input.isPointerDown) {
            ws.send(JSON.stringify(state));
        }
    }
}
connect();
loop();