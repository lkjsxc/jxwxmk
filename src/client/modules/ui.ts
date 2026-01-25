import { InputManager } from "./input";
import { Player, Item } from "../types";

export enum AppState {
    StartScreen,
    InGame,
    GameOver,
}

export enum MenuTab {
    Inventory,
    Crafting,
    Guidebook,
    Settings,
}

export class UIManager {
    state: AppState = AppState.StartScreen;
    isMenuOpen: boolean = false;
    activeTab: MenuTab = MenuTab.Inventory;

    joinRequest: boolean = false;
    craftRequest: string | null = null;
    slotSelectRequest: number | null = null;
    respawnRequest: boolean = false;

    render(ctx: CanvasRenderingContext2D, player: Player | null, input: InputManager) {
        const w = ctx.canvas.width;
        const h = ctx.canvas.height;

        if (this.state === AppState.StartScreen) {
            this.drawStartScreen(ctx, w, h);
        } else if (this.state === AppState.GameOver) {
            this.drawGameOver(ctx, w, h);
        } else if (this.state === AppState.InGame && player) {
            this.drawHotbar(ctx, player, w, h);
            this.drawHUDButtons(ctx, w);
            if (this.isMenuOpen) {
                this.drawMenuOverlay(ctx, player, w, h);
            }
        }
    }

    drawStartScreen(ctx: CanvasRenderingContext2D, w: number, h: number) {
        ctx.fillStyle = "#111"; ctx.fillRect(0, 0, w, h);
        ctx.fillStyle = "#eee"; ctx.font = "bold 60px sans-serif"; ctx.textAlign = "center";
        ctx.fillText("kkmypk", w / 2, h / 3);

        const btnW = 200; const btnH = 60; const btnX = (w - btnW) / 2; const btnY = h / 2;
        ctx.fillStyle = "#4a4"; ctx.fillRect(btnX, btnY, btnW, btnH);
        ctx.fillStyle = "white"; ctx.font = "30px sans-serif"; ctx.fillText("PLAY", w / 2, btnY + 40);
    }

    drawGameOver(ctx: CanvasRenderingContext2D, w: number, h: number) {
        ctx.fillStyle = "rgba(100, 0, 0, 0.8)"; ctx.fillRect(0, 0, w, h);
        ctx.fillStyle = "white"; ctx.font = "bold 60px sans-serif"; ctx.textAlign = "center";
        ctx.fillText("YOU DIED", w / 2, h / 3);
        
        const btnW = 200; const btnH = 60; const btnX = (w - btnW) / 2; const btnY = h / 2;
        ctx.fillStyle = "#a44"; ctx.fillRect(btnX, btnY, btnW, btnH);
        ctx.fillStyle = "white"; ctx.font = "24px sans-serif"; ctx.fillText("RESPAWN", w / 2, btnY + 40);
    }

    drawHUDButtons(ctx: CanvasRenderingContext2D, w: number) {
        this.drawButton(ctx, w - 60, 20, 50, 50, "MENU", this.isMenuOpen);
    }

    drawMenuOverlay(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number) {
        ctx.fillStyle = "rgba(0,0,0,0.8)"; ctx.fillRect(0, 0, w, h);
        const margin = 40; const panelX = margin; const panelY = margin;
        const panelW = w - margin * 2; const panelH = h - margin * 2;

        ctx.fillStyle = "#222"; ctx.fillRect(panelX, panelY, panelW, panelH);
        ctx.strokeStyle = "#444"; ctx.strokeRect(panelX, panelY, panelW, panelH);

        // Close Button (Top Right of Panel)
        this.drawButton(ctx, panelX + panelW - 40, panelY + 10, 30, 30, "X", false);

        const tabs = ["Bag", "Craft", "Help", "Sets"];
        const tabW = (panelW - 50) / tabs.length;
        for (let i = 0; i < tabs.length; i++) {
            const tx = panelX + i * tabW;
            ctx.fillStyle = i === this.activeTab ? "#444" : "#333";
            ctx.fillRect(tx, panelY, tabW, 50);
            ctx.fillStyle = i === this.activeTab ? "white" : "#aaa";
            ctx.font = "18px sans-serif"; ctx.textAlign = "center";
            ctx.fillText(tabs[i], tx + tabW / 2, panelY + 32);
        }

        ctx.save(); ctx.translate(panelX, panelY + 50);
        if (this.activeTab === MenuTab.Inventory) this.drawInventory(ctx, player, panelW, panelH - 50);
        else if (this.activeTab === MenuTab.Crafting) this.drawCrafting(ctx, panelW, panelH - 50);
        else if (this.activeTab === MenuTab.Guidebook) this.drawGuidebook(ctx, panelW, panelH - 50);
        ctx.restore();
    }

    drawInventory(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number) {
        const cols = 5; const slotSize = 60; const padding = 15;
        const gridW = cols * slotSize + (cols - 1) * padding;
        const startX = (w - gridW) / 2;
        for (let i = 0; i < 20; i++) {
            const x = startX + (i % cols) * (slotSize + padding);
            const y = 60 + Math.floor(i / cols) * (slotSize + padding);
            ctx.fillStyle = "rgba(255,255,255,0.1)"; ctx.fillRect(x, y, slotSize, slotSize);
            const item = player.inventory.slots[10 + i];
            if (item) this.drawItem(ctx, item, x, y, slotSize);
        }
    }

    drawCrafting(ctx: CanvasRenderingContext2D, w: number, h: number) {
        const recipes = [
            { name: "Wood Pick", code: "WoodPickaxe", req: "10 Wood" },
            { name: "Stone Pick", code: "StonePickaxe", req: "10W, 10S" },
            { name: "Wood Wall", code: "WoodWall", req: "20 Wood" },
            { name: "Torch", code: "Torch", req: "2 Wood" },
        ];
        let y = 40; const btnW = 260; const x = (w - btnW) / 2;
        for (const r of recipes) {
            ctx.fillStyle = "#444"; ctx.fillRect(x, y, btnW, 45);
            ctx.fillStyle = "white"; ctx.font = "16px sans-serif"; ctx.textAlign = "left";
            ctx.fillText(r.name, x + 10, y + 28);
            ctx.fillStyle = "#aaa"; ctx.textAlign = "right"; ctx.fillText(r.req, x + btnW - 10, y + 28);
            y += 55;
        }
    }

    drawGuidebook(ctx: CanvasRenderingContext2D, w: number, h: number) {
        ctx.fillStyle = "white"; ctx.textAlign = "left"; ctx.font = "14px sans-serif";
        const lines = ["GUIDE", "WASD: Move", "LeftClick: Attack", "RightClick: Build", "1-9: Select Slot"];
        let y = 40; for (const l of lines) { ctx.fillText(l, 20, y); y += 25; }
    }

    drawHotbar(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number) {
        const slotSize = 50; const padding = 10; const startX = (w - (10 * 60)) / 2; const startY = h - 70;
        for (let i = 0; i < 10; i++) {
            const x = startX + i * 60;
            ctx.fillStyle = i === player.active_slot ? "rgba(200,200,0,0.5)" : "rgba(0,0,0,0.5)";
            ctx.fillRect(x, startY, slotSize, slotSize);
            ctx.strokeStyle = "white"; ctx.strokeRect(x, startY, slotSize, slotSize);
            const item = player.inventory.slots[i];
            if (item) this.drawItem(ctx, item, x, startY, slotSize);
        }
    }

    drawItem(ctx: CanvasRenderingContext2D, item: Item, x: number, y: number, size: number) {
        ctx.fillStyle = this.getItemColor(item.kind); ctx.beginPath(); ctx.arc(x + size/2, y + size/2, size/3, 0, Math.PI*2); ctx.fill();
        ctx.fillStyle = "white"; ctx.font = "bold 12px sans-serif"; ctx.textAlign = "right"; ctx.fillText(item.amount.toString(), x + size - 2, y + size - 2);
    }

    getItemColor(kind: string): string {
        switch (kind) {
            case "Wood": return "#852"; case "Stone": return "#888"; case "Berry": return "#e22";
            case "Meat": return "#f88"; case "WoodPickaxe": return "#a74"; case "Torch": return "#ea2";
            default: return "#fff";
        }
    }

    drawButton(ctx: CanvasRenderingContext2D, x: number, y: number, w: number, h: number, label: string, active: boolean) {
        ctx.fillStyle = active ? "#4a4" : "#444"; ctx.fillRect(x, y, w, h);
        ctx.strokeStyle = "white"; ctx.strokeRect(x, y, w, h);
        ctx.fillStyle = "white"; ctx.textAlign = "center"; ctx.font = "12px sans-serif"; ctx.fillText(label, x + w / 2, y + h / 2 + 5);
    }

    handleInput(input: InputManager, w: number, h: number) {
        if (this.state === AppState.InGame && !this.isMenuOpen) {
            for (let i = 0; i <= 9; i++) if (input.keys[`num${i}` as any]) this.slotSelectRequest = i === 0 ? 9 : i - 1;
        }
        if (input.mouseLeftDown) {
            const mx = input.mouseX; const my = input.mouseY;
            if (this.state === AppState.StartScreen) {
                if (this.hitTest(mx, my, (w - 200) / 2, h / 2, 200, 60)) { this.joinRequest = true; input.mouseLeftDown = false; }
            } else if (this.state === AppState.GameOver) {
                if (this.hitTest(mx, my, (w - 200) / 2, h / 2, 200, 60)) { this.respawnRequest = true; input.mouseLeftDown = false; }
            } else if (this.state === AppState.InGame) {
                if (this.isMenuOpen) {
                    const margin = 40; const panelX = margin; const panelY = margin; const panelW = w - margin * 2;
                    if (this.hitTest(mx, my, panelX + panelW - 40, panelY + 10, 30, 30)) { this.isMenuOpen = false; input.mouseLeftDown = false; }
                    else if (this.hitTest(mx, my, panelX, panelY, panelW, 50)) { this.activeTab = Math.floor((mx - panelX) / (panelW / 4)); input.mouseLeftDown = false; }
                    else if (this.activeTab === MenuTab.Crafting) {
                        const recipes = ["WoodPickaxe", "StonePickaxe", "WoodWall", "Torch"];
                        let rY = panelY + 50 + 40;
                        for (const code of recipes) {
                            if (this.hitTest(mx, my, panelX + (panelW - 260)/2, rY, 260, 45)) { this.craftRequest = code; input.mouseLeftDown = false; }
                            rY += 55;
                        }
                    }
                } else {
                    if (this.hitTest(mx, my, w - 60, 20, 50, 50)) { this.isMenuOpen = true; input.mouseLeftDown = false; }
                    const startX = (w - (10 * 60)) / 2; const startY = h - 70;
                    if (this.hitTest(mx, my, startX, startY, 600, 50)) {
                        const idx = Math.floor((mx - startX) / 60);
                        if (idx >= 0 && idx < 10) this.slotSelectRequest = idx;
                    }
                }
            }
        }
    }

    hitTest(mx: number, my: number, x: number, y: number, w: number, h: number): boolean { return mx >= x && mx <= x + w && my >= y && my <= y + h; }
}
