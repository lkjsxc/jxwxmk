import { InputManager } from "./modules/input";
import { Renderer } from "./modules/renderer";
import { World } from "./types";

const renderer = new Renderer();
const input = new InputManager();

let world: World | null = null;
const protocol = location.protocol === 'https:' ? 'wss' : 'ws';
const ws = new WebSocket(`${protocol}://${location.host}/ws`);

ws.onmessage = (event) => {
    try {
        world = JSON.parse(event.data);
    } catch (e) {
        console.error("Parse error", e);
    }
};

ws.onopen = () => {
    console.log("Connected");
};

function loop() {
    renderer.render(world, input);
    requestAnimationFrame(loop);
}

function sendInput() {
    if (ws.readyState === WebSocket.OPEN) {
        const state = input.getState();
        if (state.dx !== 0 || state.dy !== 0 || state.attack) {
            ws.send(JSON.stringify(state));
        }
    }
}

setInterval(sendInput, 50);
loop();