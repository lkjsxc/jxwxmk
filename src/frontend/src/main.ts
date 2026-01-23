// Main game entry point
import { GameClient } from './game/client';
import { renderGame } from './render/renderer';

const gameClient = new GameClient();
const canvas = document.getElementById('game-canvas') as HTMLCanvasElement;

if (canvas) {
    renderGame(canvas, gameClient);
    gameClient.connect();
} else {
    console.error('Game canvas not found');
}