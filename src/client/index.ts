import { GameClient } from './websocket';
import { Game } from './game';
import { Renderer } from './render';
import { InputHandler } from './input';

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