export class Renderer {
    constructor(private ctx: CanvasRenderingContext2D) {}

    render(game: any) {
        this.ctx.clearRect(0, 0, 800, 600);
        // Draw player
        this.ctx.fillRect(game.player.x, game.player.y, 20, 20);
        // Draw resources
        for (const res of game.resources) {
            this.ctx.fillRect(res.x, res.y, 10, 10);
        }
    }
}