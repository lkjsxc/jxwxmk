import { InputManager } from "./input";
import { Player, Item } from "../types";

export enum AppState { StartScreen, InGame, GameOver }
export enum MenuTab { Inventory, Crafting, Profile, Guidebook }

interface DragState {
    fromIndex: number;
    item: Item;
    startX: number;
    startY: number;
}

export class UIManager {
    state: AppState = AppState.StartScreen;
    isMenuOpen: boolean = false;
    activeTab: MenuTab = MenuTab.Inventory;

    joinRequest: boolean = false;
    craftRequest: string | null = null;
    slotSelectRequest: number | null = null;
    respawnRequest: boolean = false;
    nameUpdateRequest: string | null = null;
    swapRequest: [number, number] | null = null;

    private drag: DragState | null = null;

    render(ctx: CanvasRenderingContext2D, player: Player | null, input: InputManager) {
        const w = ctx.canvas.width; const h = ctx.canvas.height;
        if (this.state === AppState.StartScreen) this.drawStartScreen(ctx, w, h);
        else if (this.state === AppState.GameOver) this.drawGameOver(ctx, w, h);
        else if (this.state === AppState.InGame && player) {
            this.drawHotbar(ctx, player, w, h);
            this.drawHUDButtons(ctx, w);
            if (this.isMenuOpen) this.drawMenuOverlay(ctx, player, w, h);
            if (this.drag) this.drawDraggedItem(ctx, input);
        }
    }

    drawStartScreen(ctx: CanvasRenderingContext2D, w: number, h: number) {
        ctx.fillStyle = "#111"; ctx.fillRect(0, 0, w, h);
        ctx.fillStyle = "#eee"; ctx.font = "bold 60px sans-serif"; ctx.textAlign = "center";
        ctx.textBaseline = "middle"; ctx.fillText("kkmypk", w / 2, h / 3);
        const btnW = 200; const btnH = 60; const btnX = (w - btnW) / 2; const btnY = h / 2;
        ctx.fillStyle = "#4a4"; ctx.fillRect(btnX, btnY, btnW, btnH);
        ctx.fillStyle = "white"; ctx.font = "30px sans-serif"; ctx.fillText("PLAY", w / 2, btnY + btnH / 2);
    }

    drawGameOver(ctx: CanvasRenderingContext2D, w: number, h: number) {
        ctx.fillStyle = "rgba(100, 0, 0, 0.8)"; ctx.fillRect(0, 0, w, h);
        ctx.fillStyle = "white"; ctx.font = "bold 60px sans-serif"; ctx.textAlign = "center";
        ctx.textBaseline = "middle"; ctx.fillText("YOU DIED", w / 2, h / 3);
        const btnW = 300; const btnH = 80; const btnX = (w - btnW) / 2; const btnY = h / 2;
        ctx.fillStyle = "#a44"; ctx.fillRect(btnX, btnY, btnW, btnH);
        ctx.strokeStyle = "white"; ctx.lineWidth = 4; ctx.strokeRect(btnX, btnY, btnW, btnH);
        ctx.fillStyle = "white"; ctx.font = "bold 32px sans-serif"; ctx.fillText("RESPAWN", w / 2, btnY + btnH / 2);
    }

    drawHUDButtons(ctx: CanvasRenderingContext2D, w: number) {
        this.drawButton(ctx, w - 60, 20, 50, 50, "MENU", this.isMenuOpen);
    }

    drawMenuOverlay(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number) {
        ctx.fillStyle = "rgba(0,0,0,0.6)"; ctx.fillRect(0, 0, w, h);
        const margin = 40; const panelX = margin; const panelY = margin;
        const panelW = w - margin * 2; const panelH = h - margin * 2;
        ctx.fillStyle = "rgba(34, 34, 34, 0.85)"; ctx.fillRect(panelX, panelY, panelW, panelH);
        ctx.strokeStyle = "rgba(255,255,255,0.2)"; ctx.strokeRect(panelX, panelY, panelW, panelH);
        this.drawButton(ctx, panelX + panelW - 40, panelY + 10, 30, 30, "X", false);

        const tabs = ["Bag", "Craft", "Prof", "Help"];
        const tabW = (panelW - 50) / tabs.length;
        ctx.textBaseline = "middle";
        for (let i = 0; i < tabs.length; i++) {
            const tx = panelX + i * tabW;
            ctx.fillStyle = i === this.activeTab ? "rgba(68,68,68,0.9)" : "rgba(51,51,51,0.9)";
            ctx.fillRect(tx, panelY, tabW, 50);
            ctx.fillStyle = i === this.activeTab ? "white" : "#aaa";
            ctx.font = "18px sans-serif"; ctx.textAlign = "center";
            ctx.fillText(tabs[i], tx + tabW / 2, panelY + 25);
        }

        ctx.save(); ctx.translate(panelX, panelY + 50);
        if (this.activeTab === MenuTab.Inventory) this.drawInventory(ctx, player, panelW, panelH - 50);
        else if (this.activeTab === MenuTab.Crafting) this.drawCrafting(ctx, panelW, panelH - 50);
        else if (this.activeTab === MenuTab.Profile) this.drawProfile(ctx, player, panelW, panelH - 50);
        else if (this.activeTab === MenuTab.Guidebook) this.drawGuidebook(ctx, panelW, panelH - 50);
        ctx.restore();
    }

    drawInventory(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number) {
        const slotSize = 60; const padding = 10; const cols = 7;
        const totalW = cols * (slotSize + padding);
        const startX = (w - totalW) / 2;
        
        // Render 30 slots (7x4 grid roughly)
        for (let i = 0; i < 30; i++) {
            const col = i % cols; const row = Math.floor(i / cols);
            const x = startX + col * (slotSize + padding);
            const y = 40 + row * (slotSize + padding);
            
            // Minecraft Style
            ctx.fillStyle = "#111"; ctx.fillRect(x, y, slotSize, slotSize);
            ctx.strokeStyle = "#555"; ctx.lineWidth = 2; ctx.strokeRect(x, y, slotSize, slotSize);
            ctx.strokeStyle = "#000"; ctx.lineWidth = 1; ctx.strokeRect(x+2, y+2, slotSize-4, slotSize-4);

            const item = player.inventory.slots[i];
            // Don't draw the item if it's currently being dragged from this slot
            if (item && (!this.drag || this.drag.fromIndex !== i)) {
                this.drawItem(ctx, item, x, y, slotSize);
            }
        }
    }

    drawDraggedItem(ctx: CanvasRenderingContext2D, input: InputManager) {
        if (!this.drag) return;
        ctx.save();
        ctx.globalAlpha = 0.7;
        this.drawItem(ctx, this.drag.item, input.mouseX - 30, input.mouseY - 30, 60);
        ctx.restore();
    }

    drawCrafting(ctx: CanvasRenderingContext2D, w: number, h: number) {
        const recipes = [{ name: "Wood Pick", code: "WoodPickaxe", req: "10 Wood" }, { name: "Stone Pick", code: "StonePickaxe", req: "10W, 10S" }, { name: "Wood Wall", code: "WoodWall", req: "20 Wood" }, { name: "Torch", code: "Torch", req: "2 Wood" }];
        let y = 40; const btnW = 260; const x = (w - btnW) / 2;
        ctx.textBaseline = "middle";
        for (const r of recipes) {
            ctx.fillStyle = "rgba(68,68,68,0.8)"; ctx.fillRect(x, y, btnW, 45);
            ctx.fillStyle = "white"; ctx.font = "16px sans-serif"; ctx.textAlign = "left";
            ctx.fillText(r.name, x + 10, y + 22);
            ctx.fillStyle = "#aaa"; ctx.textAlign = "right"; ctx.fillText(r.req, x + btnW - 10, y + 22);
            y += 55;
        }
    }

    drawProfile(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number) {
        ctx.fillStyle = "white"; ctx.textAlign = "center"; ctx.font = "24px sans-serif";
        ctx.fillText("Player Profile", w / 2, 40);
        ctx.font = "18px sans-serif"; ctx.fillText(`Current Name: ${player.username}`, w / 2, 100);
        const btnW = 200; const btnH = 40; const btnX = (w - btnW) / 2;
        ctx.fillStyle = "rgba(68,68,170,0.8)"; ctx.fillRect(btnX, 140, btnW, btnH);
        ctx.fillStyle = "white"; ctx.fillText("Change Name", w / 2, 160);
    }

    drawGuidebook(ctx: CanvasRenderingContext2D, w: number, h: number) {
        ctx.fillStyle = "white"; ctx.textAlign = "left"; ctx.font = "14px sans-serif";
        const lines = ["GUIDE", "WASD: Move", "LeftClick: Attack", "RightClick: Build/Eat", "1-7: Select Slot"];
        let y = 40; for (const l of lines) { ctx.fillText(l, 20, y); y += 25; }
    }

    drawHotbar(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number) {
        const slots = 7; const slotSize = 50; const totalW = slots * 60;
        const startX = (w - totalW) / 2; const startY = h - 70;
        for (let i = 0; i < slots; i++) {
            const x = startX + i * 60;
            ctx.fillStyle = i === player.active_slot ? "rgba(200,200,0,0.4)" : "rgba(0,0,0,0.4)";
            ctx.fillRect(x, startY, slotSize, slotSize);
            ctx.strokeStyle = "rgba(255,255,255,0.5)"; ctx.strokeRect(x, startY, slotSize, slotSize);
            const item = player.inventory.slots[i];
            if (item && (!this.drag || this.drag.fromIndex !== i)) this.drawItem(ctx, item, x, startY, slotSize);
        }
        const activeItem = player.inventory.slots[player.active_slot];
        if (activeItem) {
            ctx.fillStyle = "white"; ctx.font = "bold 16px sans-serif"; ctx.textAlign = "center";
            ctx.fillText(`${activeItem.kind} ${this.getItemAction(activeItem.kind)}`, w / 2, startY - 20);
        }
    }

    getItemAction(kind: string): string {
        if (["Berry", "Meat", "CookedMeat"].includes(kind)) return "[A] Eat";
        if (["WoodWall", "Door", "Torch", "Workbench"].includes(kind)) return "[A] Place";
        return "[A] Use";
    }

    drawItem(ctx: CanvasRenderingContext2D, item: Item, x: number, y: number, size: number) {
        ctx.fillStyle = this.getItemColor(item.kind); ctx.beginPath(); ctx.arc(x + size/2, y + size/2, size/3, 0, Math.PI*2); ctx.fill();
        ctx.fillStyle = "white"; ctx.font = "bold 12px sans-serif"; ctx.textAlign = "right"; ctx.textBaseline = "alphabetic";
        ctx.fillText(item.amount.toString(), x + size - 4, y + size - 4);
    }

    getItemColor(kind: string): string {
        switch (kind) {
            case "Wood": return "#852"; case "Stone": return "#888"; case "Berry": return "#e22";
            case "Meat": return "#f88"; case "WoodPickaxe": return "#a74"; case "Torch": return "#ea2";
            default: return "#fff";
        }
    }

    drawButton(ctx: CanvasRenderingContext2D, x: number, y: number, w: number, h: number, label: string, active: boolean) {
        ctx.fillStyle = active ? "rgba(74, 164, 74, 0.6)" : "rgba(68, 68, 68, 0.6)";
        ctx.fillRect(x, y, w, h);
        ctx.strokeStyle = "rgba(255,255,255,0.4)"; ctx.strokeRect(x, y, w, h);
        ctx.fillStyle = "white"; ctx.textAlign = "center"; ctx.textBaseline = "middle";
        ctx.font = "12px sans-serif"; ctx.fillText(label, x + w / 2, y + h / 2);
    }

    handleInput(input: InputManager, w: number, h: number, player: Player | null) {
        if (this.state === AppState.InGame && !this.isMenuOpen) {
            for (let i = 1; i <= 7; i++) if (input.keys[`num${i}` as any]) this.slotSelectRequest = i - 1;
        }
        if (input.isPointerDown) {
            const mx = input.mouseX; const my = input.mouseY;
            if (this.state === AppState.StartScreen) {
                if (this.hitTest(mx, my, (w - 200) / 2, h / 2, 200, 60)) { this.joinRequest = true; input.isPointerDown = false; }
            } else if (this.state === AppState.GameOver) {
                if (this.hitTest(mx, my, (w - 300) / 2, h / 2, 300, 80)) { this.respawnRequest = true; input.isPointerDown = false; }
            } else if (this.state === AppState.InGame) {
                if (this.isMenuOpen) {
                    const margin = 40; const panelX = margin; const panelY = margin; const panelW = w - margin * 2;
                    if (this.hitTest(mx, my, panelX + panelW - 40, panelY + 10, 30, 30)) { this.isMenuOpen = false; input.isPointerDown = false; }
                    else if (this.hitTest(mx, my, panelX, panelY, panelW, 50)) { this.activeTab = Math.floor((mx - panelX) / (panelW / 4)); input.isPointerDown = false; }
                    else if (this.activeTab === MenuTab.Inventory && player) {
                        const slotSize = 60; const padding = 10; const cols = 7;
                        const totalW = cols * (slotSize + padding); const startX = panelX + (panelW - totalW) / 2;
                        for (let i = 0; i < 30; i++) {
                            const x = startX + (i % cols) * (slotSize + padding);
                            const y = panelY + 50 + 40 + Math.floor(i / cols) * (slotSize + padding);
                            if (this.hitTest(mx, my, x, y, slotSize, slotSize)) {
                                if (!this.drag && player.inventory.slots[i]) {
                                    this.drag = { fromIndex: i, item: player.inventory.slots[i]!, startX: mx, startY: my };
                                }
                                break;
                            }
                        }
                    } else if (this.activeTab === MenuTab.Crafting) {
                        const recipes = ["WoodPickaxe", "StonePickaxe", "WoodWall", "Torch"];
                        let rY = panelY + 50 + 40;
                        for (const code of recipes) {
                            if (this.hitTest(mx, my, panelX + (panelW - 260)/2, rY, 260, 45)) { this.craftRequest = code; input.isPointerDown = false; }
                            rY += 55;
                        }
                    } else if (this.activeTab === MenuTab.Profile) {
                        if (this.hitTest(mx, my, panelX + (panelW - 200)/2, panelY + 50 + 140, 200, 40)) {
                            const newName = prompt("Enter new name:", "Survivor"); if (newName) this.nameUpdateRequest = newName; input.isPointerDown = false;
                        }
                    }
                } else {
                    if (this.hitTest(mx, my, w - 60, 20, 50, 50)) { this.isMenuOpen = true; input.isPointerDown = false; }
                    // Hotbar Selection Click check (0-6)
                    const startX = (w - (7 * 60)) / 2; const startY = h - 70;
                    if (this.hitTest(mx, my, startX, startY, 7 * 60, 50)) {
                        const idx = Math.floor((mx - startX) / 60);
                        if (idx >= 0 && idx < 7) { this.slotSelectRequest = idx; input.isPointerDown = false; }
                    }
                }
            }
        } else if (this.drag) {
            // Drop logic
            const mx = input.mouseX; const my = input.mouseY;
            const margin = 40; const panelX = margin; const panelY = margin; const panelW = w - margin * 2;
            const slotSize = 60; const padding = 10; const cols = 7;
            const totalW = cols * (slotSize + padding); const startX = panelX + (panelW - totalW) / 2;
            
            let dropped = false;
            for (let i = 0; i < 30; i++) {
                const x = startX + (i % cols) * (slotSize + padding);
                const y = panelY + 50 + 40 + Math.floor(i / cols) * (slotSize + padding);
                if (this.hitTest(mx, my, x, y, slotSize, slotSize)) {
                    if (i !== this.drag.fromIndex) {
                        this.swapRequest = [this.drag.fromIndex, i];
                    }
                    dropped = true; break;
                }
            }
            this.drag = null;
        }
    }

    hitTest(mx: number, my: number, x: number, y: number, w: number, h: number): boolean { return mx >= x && mx <= x + w && my >= y && my <= y + h; }
}