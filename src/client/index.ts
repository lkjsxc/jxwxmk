import { InputManager } from "./modules/input";
import { Renderer } from "./modules/renderer";
import { UIManager } from "./modules/ui";
import { World } from "./types";

const renderer = new Renderer();
const input = new InputManager();
const ui = new UIManager();

let world: World | null = null;
let myId: string | null = null;

const protocol = location.protocol === 'https:' ? 'wss' : 'ws';
const ws = new WebSocket(`${protocol}://${location.host}/ws`);

ws.onmessage = (event) => {
    try {
        const msg = JSON.parse(event.data);
        if (msg.type === "welcome") {
            myId = msg.id;
            console.log("My ID:", myId);
        } else if (msg.type === "world") {
             world = msg.data;
        } else {
            // Legacy/Direct world dump fallback
            if (msg.width) world = msg; 
        }
    } catch (e) {
        console.error("Parse error", e);
    }
};

ws.onopen = () => {
    console.log("Connected");
};

function loop() {
    ui.handleInput(input);
    renderer.render(world, input, myId, ui);
    
    // Handle Crafting Requests
    if (ui.craftRequest) {
        if (ws.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify({ craft: ui.craftRequest }));
        }
        ui.craftRequest = null;
    }

    requestAnimationFrame(loop);
}

function sendInput() {
    if (ws.readyState === WebSocket.OPEN) {
        const state = input.getState();
        // Only send if active input
        if (state.dx !== 0 || state.dy !== 0 || state.attack || state.interact) {
            ws.send(JSON.stringify(state));
        }
    }
}

setInterval(sendInput, 50);
loop();