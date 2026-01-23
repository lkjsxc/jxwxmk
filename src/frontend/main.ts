import { Game } from './game/Game';
import { WebSocketService } from './services/WebSocketService';

const canvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
const ctx = canvas.getContext('2d')!;

const game = new Game(canvas);
const wsService = new WebSocketService('ws://localhost:8080/ws');

wsService.onMessage = (data) => {
    game.handleServerMessage(data);
};

wsService.onConnect = () => {
    console.log('Connected to server');
    game.start();
};

wsService.connect();

game.onUpdate = (gameState) => {
    wsService.sendGameState(gameState);
};

function gameLoop() {
    game.update();
    game.render();
    requestAnimationFrame(gameLoop);
}

gameLoop();