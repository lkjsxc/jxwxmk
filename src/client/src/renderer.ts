import { Game } from './game';
import { UIManager, UIPage } from './ui';

export class Renderer {
    private ctx: CanvasRenderingContext2D;
    private ppu: number = 16; // Pixels per world unit
    private ui: UIManager;

    constructor(private canvas: HTMLCanvasElement, private game: Game) {
        this.ctx = canvas.getContext('2d')!;
        this.ui = new UIManager(game);
        window.addEventListener('resize', this.resize.bind(this));
        this.resize();
    }

    private resize() {
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
    }

    render() {
        const ctx = this.ctx;
        ctx.fillStyle = '#111';
        ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        const player = Array.from(this.game.entities.values()).find(e => e.id === this.game.playerID);
        const camX = player ? player.x : 0;
        const camY = player ? player.y : 0;

        ctx.save();
        ctx.translate(this.canvas.width / 2, this.canvas.height / 2);
        ctx.scale(this.ppu, this.ppu);
        ctx.translate(-camX, -camY);

        // Draw Chunks/Grid
        this.drawGrid();

        // Draw Entities
        for (const ent of this.game.entities.values()) {
            this.drawEntity(ent);
        }

        ctx.restore();

        // Draw UI (Overlay)
        this.ui.render(ctx);
    }

    public toggleInventory() {
        this.ui.currentPage = this.ui.currentPage === UIPage.Inventory ? UIPage.None : UIPage.Inventory;
    }

    public toggleCrafting() {
        this.ui.currentPage = this.ui.currentPage === UIPage.Crafting ? UIPage.None : UIPage.Crafting;
    }

    public toggleQuests() {
        this.ui.currentPage = this.ui.currentPage === UIPage.Quests ? UIPage.None : UIPage.Quests;
    }

    public toggleAchievements() {
        this.ui.currentPage = this.ui.currentPage === UIPage.Achievements ? UIPage.None : UIPage.Achievements;
    }

    public isCraftingOpen(): boolean {
        return this.ui.currentPage === UIPage.Crafting;
    }

    public getCraftingRecipeAt(mx: number, my: number): string | null {
        return this.ui.getRecipeAt(mx, my);
    }

    private drawGrid() {
        const ctx = this.ctx;
        ctx.strokeStyle = '#222';
        ctx.lineWidth = 0.05;
        const size = 128; // Chunk size
        for (let x = -2; x <= 2; x++) {
            for (let y = -2; y <= 2; y++) {
                ctx.strokeRect(x * size, y * size, size, size);
            }
        }
    }

    private drawEntity(ent: any) {
        const ctx = this.ctx;
        ctx.fillStyle = this.getKindColor(ent.kind);
        ctx.beginPath();
        ctx.arc(ent.x, ent.y, 0.75, 0, Math.PI * 2);
        ctx.fill();
        
        if (ent.name) {
            ctx.fillStyle = 'white';
            ctx.font = '0.5px sans-serif';
            ctx.fillText(ent.name, ent.x - 1, ent.y - 1);
        }
    }

    private getKindColor(kind: string): string {
        switch (kind) {
            case 'player': return '#44f';
            case 'mob': return '#f44';
            case 'resource': return '#4f4';
            case 'structure': return '#aa4';
            case 'npc': return '#f4f';
            default: return '#fff';
        }
    }
}
