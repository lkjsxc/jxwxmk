import { Game } from './game';

export enum UIPage {
    None,
    Inventory,
    Crafting,
    Quests,
    Achievements,
}

export class UIManager {
    public currentPage: UIPage = UIPage.None;

    constructor(private game: Game) {}

    render(ctx: CanvasRenderingContext2D) {
        this.renderHUD(ctx);
        this.renderHotbar(ctx);
        
        switch (this.currentPage) {
            case UIPage.Inventory:
                this.renderInventory(ctx);
                break;
            case UIPage.Crafting:
                this.renderCrafting(ctx);
                break;
            case UIPage.Quests:
                this.renderQuests(ctx);
                break;
            case UIPage.Achievements:
                this.renderAchievements(ctx);
                break;
        }

        this.renderToasts(ctx);
    }

    private renderHUD(ctx: CanvasRenderingContext2D) {
        const p = Array.from(this.game.entities.values()).find(e => e.id === this.game.playerID);
        if (!p) return;

        const x = 35;
        const y = 10;
        const w = 200;
        const h = 15;
        const spacing = 20;

        this.drawBar(ctx, x, y, w, h, (p.hp || 0) / (p.max_hp || 100), 'rgba(255, 0, 0, 0.5)', 'HP');
        this.drawBar(ctx, x, y + spacing, w, h, (p.hunger || 0) / 100, 'rgba(255, 165, 0, 0.5)', 'HG');
        this.drawBar(ctx, x, y + spacing * 2, w, h, (p.temp || 0) / 100, 'rgba(0, 100, 255, 0.5)', 'TP');
    }

    private drawBar(ctx: CanvasRenderingContext2D, x: number, y: number, w: number, h: number, fill: number, color: string, label: string) {
        ctx.fillStyle = 'rgba(0,0,0,0.5)';
        ctx.fillRect(x, y, w, h);
        ctx.fillStyle = color;
        ctx.fillRect(x, y, w * fill, h);
        ctx.strokeStyle = 'white';
        ctx.lineWidth = 1;
        ctx.strokeRect(x, y, w, h);
        ctx.fillStyle = 'white';
        ctx.font = '10px sans-serif';
        ctx.fillText(label, x - 25, y + 12);
    }

    private renderHotbar(ctx: CanvasRenderingContext2D) {
        const w = 350;
        const h = 50;
        const x = (ctx.canvas.width - w) / 2;
        const y = ctx.canvas.height - h - 20;

        ctx.fillStyle = 'rgba(0,0,0,0.5)';
        ctx.fillRect(x, y, w, h);

        const slotSize = w / 7;
        for (let i = 0; i < 7; i++) {
            ctx.strokeStyle = 'white';
            ctx.lineWidth = 1;
            ctx.strokeRect(x + i * slotSize, y, slotSize, h);
            ctx.fillStyle = 'rgba(255,255,255,0.3)';
            ctx.font = '10px sans-serif';
            ctx.fillText((i + 1).toString(), x + i * slotSize + 5, y + 15);
        }
    }

    private renderInventory(ctx: CanvasRenderingContext2D) {
        const cols = 7;
        const rows = 4;
        const slotSize = 50;
        const spacing = 5;
        const w = cols * (slotSize + spacing) + spacing;
        const h = rows * (slotSize + spacing) + spacing;
        const x = (ctx.canvas.width - w) / 2;
        const y = (ctx.canvas.height - h) / 2;

        ctx.fillStyle = 'rgba(0,0,0,0.8)';
        ctx.fillRect(x, y, w, h);
        ctx.strokeStyle = 'white';
        ctx.strokeRect(x, y, w, h);

        for (let i = 0; i < 28; i++) {
            const r = Math.floor(i / cols);
            const c = i % cols;
            const sx = x + spacing + c * (slotSize + spacing);
            const sy = y + spacing + r * (slotSize + spacing);

            ctx.fillStyle = 'rgba(255,255,255,0.1)';
            ctx.fillRect(sx, sy, slotSize, slotSize);
            const item = this.game.inventory[i];
            if (item) {
                ctx.fillStyle = 'white';
                ctx.font = '10px sans-serif';
                ctx.fillText(item.item_id, sx + 5, sy + 25);
                ctx.fillText(item.count.toString(), sx + 35, sy + 45);
            }
        }
    }

    private renderCrafting(ctx: CanvasRenderingContext2D) {
        const w = 400;
        const h = 300;
        const x = (ctx.canvas.width - w) / 2;
        const y = (ctx.canvas.height - h) / 2;

        ctx.fillStyle = 'rgba(0,0,0,0.85)';
        ctx.fillRect(x, y, w, h);
        ctx.strokeStyle = 'white';
        ctx.strokeRect(x, y, w, h);

        ctx.fillStyle = 'white';
        ctx.font = '16px sans-serif';
        ctx.fillText('Crafting', x + 10, y + 25);

        const recipes = ['wood_pickaxe', 'stone_axe', 'wall_wood'];
        for (let i = 0; i < recipes.length; i++) {
            ctx.fillStyle = 'rgba(255,255,255,0.1)';
            ctx.fillRect(x + 10, y + 40 + i * 35, 150, 30);
            ctx.fillStyle = 'white';
            ctx.font = '12px sans-serif';
            ctx.fillText(recipes[i], x + 20, y + 60 + i * 35);
        }
    }

    private renderQuests(ctx: CanvasRenderingContext2D) {
        const w = 400;
        const h = 300;
        const x = (ctx.canvas.width - w) / 2;
        const y = (ctx.canvas.height - h) / 2;

        ctx.fillStyle = 'rgba(0,0,50,0.85)';
        ctx.fillRect(x, y, w, h);
        ctx.strokeStyle = 'white';
        ctx.strokeRect(x, y, w, h);

        ctx.fillStyle = 'white';
        ctx.font = '16px sans-serif';
        ctx.fillText('Quests', x + 10, y + 25);

        for (let i = 0; i < this.game.activeQuests.length; i++) {
            const q = this.game.activeQuests[i];
            ctx.fillText(`${q.id} [${q.state}]`, x + 20, y + 60 + i * 35);
        }
    }

    private renderAchievements(ctx: CanvasRenderingContext2D) {
        const w = 400;
        const h = 300;
        const x = (ctx.canvas.width - w) / 2;
        const y = (ctx.canvas.height - h) / 2;

        ctx.fillStyle = 'rgba(50,50,0,0.85)';
        ctx.fillRect(x, y, w, h);
        ctx.strokeStyle = 'white';
        ctx.strokeRect(x, y, w, h);

        ctx.fillStyle = 'white';
        ctx.font = '16px sans-serif';
        ctx.fillText('Achievements', x + 10, y + 25);

        // Simple list of IDs
        const all = Array.from(this.game.unlockedAchievements);
        for (let i = 0; i < all.length; i++) {
            ctx.fillText(all[i], x + 20, y + 60 + i * 35);
        }
    }

    private renderToasts(ctx: CanvasRenderingContext2D) {
        const now = Date.now();
        const active = this.game.notifications.filter(n => now - n.time < 3000);
        
        ctx.font = '14px sans-serif';
        for (let i = 0; i < active.length; i++) {
            const n = active[i];
            const textWidth = ctx.measureText(n.text).width;
            const x = (ctx.canvas.width - textWidth) / 2;
            const y = ctx.canvas.height - 100 - i * 30;

            ctx.fillStyle = 'rgba(0,0,0,0.7)';
            ctx.fillRect(x - 10, y - 20, textWidth + 20, 25);
            ctx.fillStyle = 'white';
            ctx.fillText(n.text, x, y);
        }
    }

    public getRecipeAt(mx: number, my: number): string | null {
        if (this.currentPage !== UIPage.Crafting) return null;
        const w = 400;
        const h = 300;
        const x = (window.innerWidth - w) / 2;
        const y = (window.innerHeight - h) / 2;
        const recipes = ['wood_pickaxe', 'stone_axe', 'wall_wood'];
        for (let i = 0; i < recipes.length; i++) {
            const rx = x + 10;
            const ry = y + 40 + i * 35;
            if (mx >= rx && mx <= rx + 150 && my >= ry && my <= ry + 30) {
                return recipes[i];
            }
        }
        return null;
    }
}