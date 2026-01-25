import { InputManager } from "./input";
import { Player, Item } from "../types";

export class UIManager {
    showInventory: boolean = false;
    showCrafting: boolean = false;
    
    // Simple cooldown for toggle
    lastToggle: number = 0;
    
    craftRequest: string | null = null;

    update(input: InputManager) {
        const now = Date.now();
        // Toggle Inventory 'I' (Virtual key check needed? InputManager tracks 'keys')
        // We didn't add 'I' to InputManager keys. Let's assume 'E' opens inventory/crafting for now or add 'I'.
        // Let's add 'I' to input manager locally or just check here if we had access to raw keys.
        // For now, I'll use a hack: click on a button on screen or use 'E'.
        
        // Let's just assume we want an on-screen button for mobile/desktop for now.
        
        // Check clicks
        if (input.mouseLeftDown) {
            // Check UI bounds
        }
    }

    render(ctx: CanvasRenderingContext2D, player: Player, input: InputManager) {
        const w = ctx.canvas.width;
        const h = ctx.canvas.height;

        // Draw HUD Hotbar
        this.drawHotbar(ctx, player, w, h);

        // Toggle Buttons (Top Right)
        this.drawIcon(ctx, w - 50, 20, "INV", this.showInventory);
        this.drawIcon(ctx, w - 110, 20, "CFT", this.showCrafting);

        // Inventory Overlay
        if (this.showInventory) {
            this.drawInventory(ctx, player, w, h);
        }

        // Crafting Overlay
        if (this.showCrafting) {
            this.drawCrafting(ctx, w, h);
        }
    }

    drawHotbar(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number) {
        const slotSize = 50;
        const padding = 10;
        const count = 10; // First 10 slots
        const totalW = count * slotSize + (count - 1) * padding;
        const startX = (w - totalW) / 2;
        const startY = h - slotSize - 20;

        for (let i = 0; i < count; i++) {
            const x = startX + i * (slotSize + padding);
            const item = player.inventory.slots[i];
            
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
            ctx.fillText((i+1).toString(), x + 2, startY + 12);
        }
    }

    drawInventory(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number) {
        const cols = 5;
        const rows = 4; // 20 slots + hotbar?
        // Player has 30 slots. 0-9 Hotbar. 10-29 Bag.
        
        const slotSize = 60;
        const padding = 10;
        const bgW = cols * slotSize + (cols + 1) * padding;
        const bgH = rows * slotSize + (rows + 1) * padding;
        const x = (w - bgW) / 2;
        const y = (h - bgH) / 2;

        ctx.fillStyle = "rgba(0,0,0,0.8)";
        ctx.fillRect(x, y, bgW, bgH);

        for (let i = 0; i < 20; i++) {
            const slotIdx = 10 + i; // Offset by hotbar
            const col = i % cols;
            const row = Math.floor(i / cols);
            const sx = x + padding + col * (slotSize + padding);
            const sy = y + padding + row * (slotSize + padding);

            ctx.fillStyle = "rgba(255,255,255,0.1)";
            ctx.fillRect(sx, sy, slotSize, slotSize);
            
            const item = player.inventory.slots[slotIdx];
            if (item) {
                this.drawItem(ctx, item, sx, sy, slotSize);
            }
        }
    }

    drawCrafting(ctx: CanvasRenderingContext2D, w: number, h: number) {
        const bgW = 300;
        const bgH = 400;
        const x = (w - bgW) / 2;
        const y = (h - bgH) / 2;

        ctx.fillStyle = "rgba(0,0,0,0.9)";
        ctx.fillRect(x, y, bgW, bgH);

        ctx.fillStyle = "white";
        ctx.font = "20px sans-serif";
        ctx.fillText("Crafting", x + 20, y + 30);

        const recipes = [
            { name: "Wood Pickaxe", code: "WoodPickaxe" },
            { name: "Stone Pickaxe", code: "StonePickaxe" },
            { name: "Wood Wall", code: "WoodWall" },
            { name: "Door", code: "Door" },
            { name: "Torch", code: "Torch" },
            { name: "Workbench", code: "Workbench" },
        ];

        let ry = y + 60;
        for (const r of recipes) {
            ctx.fillStyle = "#444";
            ctx.fillRect(x + 20, ry, bgW - 40, 40);
            ctx.fillStyle = "white";
            ctx.fillText(r.name, x + 30, ry + 25);
            ry += 50;
        }
    }

    drawItem(ctx: CanvasRenderingContext2D, item: Item, x: number, y: number, size: number) {
        // Placeholder colors/text
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

    drawIcon(ctx: CanvasRenderingContext2D, x: number, y: number, label: string, active: boolean) {
        ctx.fillStyle = active ? "#4a4" : "#444";
        ctx.fillRect(x, y, 50, 50);
        ctx.fillStyle = "white";
        ctx.textAlign = "center";
        ctx.fillText(label, x + 25, y + 30);
    }

    handleInput(input: InputManager) {
        // We need to consume clicks here if we overlap
        // A simple way is to check if mouse is over buttons
        // Check Top Right Buttons
        const w = window.innerWidth;
        if (input.mouseLeftDown) {
             // Inventory Btn
             if (this.hitTest(input.mouseX, input.mouseY, w - 50, 20, 50, 50)) {
                 this.showInventory = !this.showInventory;
                 this.showCrafting = false;
                 input.mouseLeftDown = false; // Consume
             }
             // Crafting Btn
             else if (this.hitTest(input.mouseX, input.mouseY, w - 110, 20, 50, 50)) {
                 this.showCrafting = !this.showCrafting;
                 this.showInventory = false;
                 input.mouseLeftDown = false;
             }
             
             // Crafting List
             if (this.showCrafting) {
                 const bgW = 300;
                 const bgH = 400;
                 const x = (w - bgW) / 2;
                 const y = (window.innerHeight - bgH) / 2;
                 
                 const recipes = [
                    { name: "Wood Pickaxe", code: "WoodPickaxe" },
                    { name: "Stone Pickaxe", code: "StonePickaxe" },
                    { name: "Wood Wall", code: "WoodWall" },
                    { name: "Door", code: "Door" },
                    { name: "Torch", code: "Torch" },
                    { name: "Workbench", code: "Workbench" },
                ];
                
                let ry = y + 60;
                for (const r of recipes) {
                    if (this.hitTest(input.mouseX, input.mouseY, x + 20, ry, bgW - 40, 40)) {
                        this.craftRequest = r.code;
                        input.mouseLeftDown = false;
                    }
                    ry += 50;
                }
             }
        }
    }

    hitTest(mx: number, my: number, x: number, y: number, w: number, h: number): boolean {
        return mx >= x && mx <= x + w && my >= y && my <= y + h;
    }
}
