import { GameClient } from './websocket.js';
import { Game } from './game.js';
import { Renderer } from './render.js';
import { InputHandler } from './input.js';

const canvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
const ctx = canvas.getContext('2d')!;

const client = new GameClient('ws://localhost:8080/ws');
const game = new Game();
const renderer = new Renderer(ctx);
const input = new InputHandler(canvas, client);

function gameLoop() {
    input.update();
    renderer.render(game);
    requestAnimationFrame(gameLoop);
}

client.connect();
gameLoop();