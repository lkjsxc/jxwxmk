import { InputManager } from "./input";
import { Player, Item } from "../types";

export enum AppState {
    StartScreen,
    InGame,
}

export enum MenuTab {
    Inventory,
    Crafting,
    Guidebook,
    Settings,
}

export class UIManager {
    state: AppState = AppState.StartScreen;
    isMenuOpen: boolean = false; // Only relevant in InGame state
    activeTab: MenuTab = MenuTab.Inventory;

    // Requests to main loop
    joinRequest: boolean = false;
    craftRequest: string | null = null;
    slotSelectRequest: number | null = null;

    update(input: InputManager) {
        // Toggle Menu with 'E' or 'M' (Mobile button logic handled in click)
        // For simplicity, we rely on click detection in handleInput or external triggers
    }

    render(ctx: CanvasRenderingContext2D, player: Player | null, input: InputManager) {
        const w = ctx.canvas.width;
        const h = ctx.canvas.height;

        if (this.state === AppState.StartScreen) {
            this.drawStartScreen(ctx, w, h);
        } else if (this.state === AppState.InGame && player) {
            // HUD is always drawn in Game
            this.drawHotbar(ctx, player, w, h);
            this.drawHUDButtons(ctx, w);

            // Menu Overlay
            if (this.isMenuOpen) {
                this.drawMenuOverlay(ctx, player, w, h);
            }
        }
    }

    drawStartScreen(ctx: CanvasRenderingContext2D, w: number, h: number) {
        // Background
        ctx.fillStyle = "#111";
        ctx.fillRect(0, 0, w, h);

        // Title
        ctx.fillStyle = "#eee";
        ctx.font = "bold 60px sans-serif";
        ctx.textAlign = "center";
        ctx.fillText("STARVE CLONE", w / 2, h / 3);

        // Play Button
        const btnW = 200;
        const btnH = 60;
        const btnX = (w - btnW) / 2;
        const btnY = h / 2;

        ctx.fillStyle = "#4a4";
        ctx.fillRect(btnX, btnY, btnW, btnH);
        ctx.fillStyle = "white";
        ctx.font = "30px sans-serif";
        ctx.fillText("PLAY", w / 2, btnY + 40);

        // Guidebook Hint
        ctx.font = "16px sans-serif";
        ctx.fillStyle = "#aaa";
        ctx.fillText("Check Guidebook in-game for help!", w / 2, h - 50);
    }

    drawHUDButtons(ctx: CanvasRenderingContext2D, w: number) {
        // Menu Button (Top Right)
        this.drawButton(ctx, w - 60, 20, 50, 50, "MENU", this.isMenuOpen);
    }

    drawMenuOverlay(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number) {
        // Overlay Dim
        ctx.fillStyle = "rgba(0,0,0,0.8)";
        ctx.fillRect(0, 0, w, h);

        const margin = 40;
        const panelX = margin;
        const panelY = margin;
        const panelW = w - margin * 2;
        const panelH = h - margin * 2;

        // Panel BG
        ctx.fillStyle = "#222";
        ctx.fillRect(panelX, panelY, panelW, panelH);
        ctx.strokeStyle = "#444";
        ctx.lineWidth = 2;
        ctx.strokeRect(panelX, panelY, panelW, panelH);

        // Tabs
        const tabs = ["Inventory", "Crafting", "Guidebook", "Settings"];
        const tabW = panelW / tabs.length;
        const tabH = 50;

        for (let i = 0; i < tabs.length; i++) {
            const tx = panelX + i * tabW;
            const isSelected = i === this.activeTab;
            
            ctx.fillStyle = isSelected ? "#444" : "#333";
            ctx.fillRect(tx, panelY, tabW, tabH);
            ctx.strokeRect(tx, panelY, tabW, tabH);
            
            ctx.fillStyle = isSelected ? "white" : "#aaa";
            ctx.font = "20px sans-serif";
            ctx.textAlign = "center";
            ctx.fillText(tabs[i], tx + tabW / 2, panelY + 32);
        }

        // Content Area
        const contentY = panelY + tabH;
        const contentH = panelH - tabH;

        ctx.save();
        ctx.translate(panelX, contentY);
        // Clip?
        
        if (this.activeTab === MenuTab.Inventory) {
            this.drawInventory(ctx, player, panelW, contentH);
        } else if (this.activeTab === MenuTab.Crafting) {
            this.drawCrafting(ctx, panelW, contentH);
        } else if (this.activeTab === MenuTab.Guidebook) {
            this.drawGuidebook(ctx, panelW, contentH);
        } else if (this.activeTab === MenuTab.Settings) {
             this.drawSettings(ctx, panelW, contentH);
        }

        ctx.restore();
    }

    drawInventory(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number) {
        ctx.fillStyle = "white";
        ctx.textAlign = "center";
        ctx.font = "24px sans-serif";
        ctx.fillText("Backpack", w / 2, 40);

        // Slots (Start from index 10, omitting hotbar for now, or show all?)
        // Let's show BAG (10-29)
        const cols = 5;
        const slotSize = 60;
        const padding = 15;
        
        const gridW = cols * slotSize + (cols - 1) * padding;
        const startX = (w - gridW) / 2;
        const startY = 80;

        for (let i = 0; i < 20; i++) {
            const slotIdx = 10 + i;
            const col = i % cols;
            const row = Math.floor(i / cols);
            const x = startX + col * (slotSize + padding);
            const y = startY + row * (slotSize + padding);

            ctx.fillStyle = "rgba(255,255,255,0.1)";
            ctx.fillRect(x, y, slotSize, slotSize);
            ctx.strokeStyle = "#555";
            ctx.strokeRect(x, y, slotSize, slotSize);

            const item = player.inventory.slots[slotIdx];
            if (item) {
                this.drawItem(ctx, item, x, y, slotSize);
            }
        }
    }

    drawCrafting(ctx: CanvasRenderingContext2D, w: number, h: number) {
        ctx.fillStyle = "white";
        ctx.textAlign = "center";
        ctx.font = "24px sans-serif";
        ctx.fillText("Crafting Station", w / 2, 40);

        const recipes = [
            { name: "Wood Pickaxe", code: "WoodPickaxe", req: "10 Wood" },
            { name: "Stone Pickaxe", code: "StonePickaxe", req: "10 Wood, 10 Stone" },
            { name: "Wood Wall", code: "WoodWall", req: "20 Wood" },
            { name: "Door", code: "Door", req: "30 Wood" },
            { name: "Torch", code: "Torch", req: "2 Wood" },
            { name: "Workbench", code: "Workbench", req: "50 Wood" },
        ];

        let y = 80;
        const btnW = 300;
        const btnH = 50;
        const x = (w - btnW) / 2;

        for (const r of recipes) {
            ctx.fillStyle = "#444";
            ctx.fillRect(x, y, btnW, btnH);
            ctx.strokeRect(x, y, btnW, btnH);

            ctx.fillStyle = "white";
            ctx.font = "18px sans-serif";
            ctx.textAlign = "left";
            ctx.fillText(r.name, x + 10, y + 30);

            ctx.fillStyle = "#aaa";
            ctx.font = "12px sans-serif";
            ctx.textAlign = "right";
            ctx.fillText(r.req, x + btnW - 10, y + 30);

            y += 60;
        }
    }

    drawGuidebook(ctx: CanvasRenderingContext2D, w: number, h: number) {
        ctx.fillStyle = "white";
        ctx.textAlign = "left";
        const x = 40;
        let y = 40;

        const lines = [
            "GUIDEBOOK",
            "",
            "Controls:",
            "- WASD / Joystick: Move",
            "- Left Click / A: Attack / Gather",
            "- Right Click / E / B: Interact / Build",
            "- Scroll / Pinch: Zoom",
            "",
            "Survival:",
            "- Maintain Hunger (Eat food) and Temperature (Stay warm)",
            "- Build walls to protect yourself from Wolves and Bears",
            "- Craft tools to gather faster",
        ];

        for (const line of lines) {
            ctx.font = line.startsWith("GUIDEBOOK") ? "bold 30px sans-serif" : 
                       line.startsWith("-") ? "16px sans-serif" : "bold 20px sans-serif";
            ctx.fillText(line, x, y);
            y += 30;
        }
    }

    drawSettings(ctx: CanvasRenderingContext2D, w: number, h: number) {
        ctx.fillStyle = "white";
        ctx.textAlign = "center";
        ctx.font = "24px sans-serif";
        ctx.fillText("Settings", w / 2, 40);
        
        ctx.font = "16px sans-serif";
        ctx.fillStyle = "#aaa";
        ctx.fillText("(No settings available in this version)", w / 2, 100);
    }

    drawHotbar(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number) {
        const slotSize = 50;
        const padding = 10;
        const count = 10;
        const totalW = count * slotSize + (count - 1) * padding;
        const startX = (w - totalW) / 2;
        const startY = h - slotSize - 20;

        for (let i = 0; i < count; i++) {
            const x = startX + i * (slotSize + padding);
            const item = player.inventory.slots[i]; // 0-9
            
            ctx.fillStyle = "rgba(0,0,0,0.5)";
            if (i === player.active_slot) ctx.fillStyle = "rgba(200,200,0,0.5)";
            ctx.fillRect(x, startY, slotSize, slotSize);
            ctx.strokeStyle = "white";
            ctx.strokeRect(x, startY, slotSize, slotSize);

            if (item) {
                this.drawItem(ctx, item, x, startY, slotSize);
            }
            
            // Number
            ctx.fillStyle = "white";
            ctx.font = "10px sans-serif";
            ctx.textAlign = "left";
            ctx.fillText((i === 9 ? 0 : i+1).toString(), x + 2, startY + 12);
        }
    }

    drawItem(ctx: CanvasRenderingContext2D, item: Item, x: number, y: number, size: number) {
        ctx.fillStyle = this.getItemColor(item.kind);
        ctx.beginPath();
        ctx.arc(x + size/2, y + size/2, size/3, 0, Math.PI*2);
        ctx.fill();
        
        ctx.fillStyle = "white";
        ctx.font = "bold 12px sans-serif";
        ctx.textAlign = "right";
        ctx.fillText(item.amount.toString(), x + size - 2, y + size - 2);
    }

    getItemColor(kind: string): string {
        switch (kind) {
            case "Wood": return "#852";
            case "Stone": return "#888";
            case "Berry": return "#e22";
            case "Meat": return "#f88";
            case "WoodPickaxe": return "#a74";
            case "StonePickaxe": return "#777";
            case "WoodWall": return "#852";
            case "Torch": return "#ea2";
            default: return "#fff";
        }
    }

    drawButton(ctx: CanvasRenderingContext2D, x: number, y: number, w: number, h: number, label: string, active: boolean) {
        ctx.fillStyle = active ? "#4a4" : "#444";
        ctx.fillRect(x, y, w, h);
        ctx.strokeStyle = "white";
        ctx.strokeRect(x, y, w, h);
        
        ctx.fillStyle = "white";
        ctx.textAlign = "center";
        ctx.font = "14px sans-serif";
        ctx.fillText(label, x + w / 2, y + h / 2 + 5);
    }

    handleInput(input: InputManager, w: number, h: number) {
        // Hotbar Keys (0-9)
        if (this.state === AppState.InGame && !this.isMenuOpen) {
            for (let i = 0; i <= 9; i++) {
                 const key = `num${i}` as keyof typeof input.keys;
                 if (input.keys[key]) {
                     // 1-9 -> indices 0-8. 0 -> index 9.
                     this.slotSelectRequest = i === 0 ? 9 : i - 1;
                 }
            }
        }

        if (input.mouseLeftDown) {
            const mx = input.mouseX;
            const my = input.mouseY;

            if (this.state === AppState.StartScreen) {
                // Play Button
                const btnW = 200;
                const btnH = 60;
                const btnX = (w - btnW) / 2;
                const btnY = h / 2;
                if (this.hitTest(mx, my, btnX, btnY, btnW, btnH)) {
                    this.joinRequest = true;
                    input.mouseLeftDown = false; // Consume click
                }
            } 
            else if (this.state === AppState.InGame) {
                if (this.isMenuOpen) {
                    const margin = 40;
                    const panelX = margin;
                    const panelY = margin;
                    const panelW = w - margin * 2;
                    // Tabs
                    const tabW = panelW / 4;
                    if (this.hitTest(mx, my, panelX, panelY, panelW, 50)) {
                         const tabIdx = Math.floor((mx - panelX) / tabW);
                         if (tabIdx >= 0 && tabIdx < 4) {
                             this.activeTab = tabIdx;
                         }
                    } else if (this.activeTab === MenuTab.Crafting) {
                         // Check recipes
                         // Re-calculate positions (copy-paste logic for simplicity, ideal: store rects)
                         const contentY = panelY + 50;
                         const btnW = 300;
                         const btnH = 50;
                         const rx = (panelW - btnW) / 2 + panelX;
                         let ry = contentY + 30; // 80 relative to contentY? drawCrafting says y=80 relative to 0? No, drawMenuOverlay translates.
                         // Actually drawCrafting logic used local coords from 0?? No, drawCrafting used `drawCrafting(ctx, panelW, contentH)`
                         // And inside: `let y = 80;` ... `ctx.fillRect(x, y...)` where x = (w - btnW)/2.
                         // And `ctx.translate(panelX, contentY)`.
                         // So HitTest must be relative to (panelX, contentY)
                         
                         const localMx = mx - panelX;
                         const localMy = my - contentY;
                         
                         const recipes = ["WoodPickaxe", "StonePickaxe", "WoodWall", "Door", "Torch", "Workbench"];
                         let rY = 80;
                         for (const code of recipes) {
                             if (this.hitTest(localMx, localMy, (panelW - btnW)/2, rY, btnW, btnH)) {
                                 this.craftRequest = code;
                                 input.mouseLeftDown = false;
                             }
                             rY += 60;
                         }
                    }
                    
                    // Click outside to close?
                    if (!this.hitTest(mx, my, panelX, panelY, panelW, h - margin * 2)) {
                        // this.isMenuOpen = false; 
                    }
                } else {
                    // HUD Buttons
                    // Menu Btn
                    if (this.hitTest(mx, my, w - 60, 20, 50, 50)) {
                        this.isMenuOpen = true;
                        input.mouseLeftDown = false;
                    }

                    // Hotbar Clicks
                    const slotSize = 50;
                    const padding = 10;
                    const count = 10;
                    const totalW = count * slotSize + (count - 1) * padding;
                    const startX = (w - totalW) / 2;
                    const startY = h - slotSize - 20;
                    
                    if (this.hitTest(mx, my, startX, startY, totalW, slotSize)) {
                        // Determine index
                        const idx = Math.floor((mx - startX) / (slotSize + padding));
                        if (idx >= 0 && idx < 10) {
                            // Check padding gap
                            const relativeX = (mx - startX) - idx * (slotSize + padding);
                            if (relativeX <= slotSize) {
                                this.slotSelectRequest = idx;
                            }
                        }
                    }
                }
            }
        }
    }

    hitTest(mx: number, my: number, x: number, y: number, w: number, h: number): boolean {
        return mx >= x && mx <= x + w && my >= y && my <= y + h;
    }
}