import { InputManager } from "./modules/input";
import { Renderer } from "./modules/renderer";
import { UIManager, AppState } from "./modules/ui";
import { World } from "./types";

const renderer = new Renderer();
const input = new InputManager();
const ui = new UIManager();

let world: World | null = null;
let prevWorld: World | null = null;
let lastUpdateAt: number = 0;
let myId: string | null = null;
let ws: WebSocket | null = null;
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
                if (msg.token) {
                    localStorage.setItem(STORAGE_KEY, msg.token);
                }
                ui.state = AppState.InGame;
            } else if (msg.type === "world") {
                 prevWorld = world;
                 world = msg.data;
                 lastUpdateAt = Date.now();
                 
                 if (ui.state === AppState.InGame && myId && world && !world.players[myId]) {
                     ui.state = AppState.GameOver;
                 }
            }
        } catch (e) {
            console.error("Parse error", e);
        }
    };

    ws.onopen = () => console.log("Connected");
    ws.onclose = () => {
        ui.state = AppState.StartScreen;
        world = null;
        myId = null;
    };
}

function loop() {
    ui.handleInput(input, renderer.canvas.width, renderer.canvas.height);

    if (ui.state === AppState.StartScreen) {
        ui.render(renderer.ctx, null, input);
        if (ui.joinRequest) {
            connect();
            ui.joinRequest = false;
        }
    } else if (ui.state === AppState.GameOver) {
        ui.render(renderer.ctx, null, input);
        if (ui.respawnRequest) {
            localStorage.removeItem(STORAGE_KEY);
            location.reload();
        }
    } else {
        const now = Date.now();
        const alpha = Math.min(1.0, (now - lastUpdateAt) / 50); 
        renderer.render(world, prevWorld, alpha, input, myId, ui);
        
        // Handle Requests
        if (ws && ws.readyState === WebSocket.OPEN) {
            if (ui.craftRequest) {
                ws.send(JSON.stringify({ craft: ui.craftRequest }));
                ui.craftRequest = null;
            }
            if (ui.slotSelectRequest !== null) {
                ws.send(JSON.stringify({ slot: ui.slotSelectRequest }));
                ui.slotSelectRequest = null;
            }
            if (ui.nameUpdateRequest) {
                ws.send(JSON.stringify({ name: ui.nameUpdateRequest }));
                ui.nameUpdateRequest = null;
            }
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

setInterval(sendInput, 50);
loop();