import { GameClient } from '../game/client';

export function renderGame(canvas: HTMLCanvasElement, client: GameClient) {
    const ctx = canvas.getContext('2d');
    if (!ctx) {
        console.error('Could not get 2D context');
        return;
    }

    // Set canvas size
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    // Game loop
    function gameLoop() {
        // Clear canvas
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // Draw game state
        const state = client.getState();
        drawGameState(ctx, state);

        requestAnimationFrame(gameLoop);
    }

    function drawGameState(ctx: CanvasRenderingContext2D, state: any) {
        // Draw background
        ctx.fillStyle = '#2a5c44';
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        // Draw player if exists
        if (state.player) {
            ctx.fillStyle = '#ff0000';
            ctx.fillRect(state.player.x - 10, state.player.y - 10, 20, 20);
        }

        // Draw UI
        ctx.fillStyle = '#000000';
        ctx.font = '16px Arial';
        ctx.fillText('Starve.io Clone', 20, 30);
    }

    // Start game loop
    gameLoop();

    // Handle window resize
    window.addEventListener('resize', () => {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
    });
}