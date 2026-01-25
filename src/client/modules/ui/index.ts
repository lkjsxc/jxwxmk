import { InputManager } from "../input";
import { Player, Item, Achievement, AppState } from "../../types";
import { drawInventory, handleInvInput } from "./inventory";
import { drawCrafting, handleCraftInput } from "./crafting";
import { drawAchievements, handleAchInput } from "./achievements";
import { drawProfile, handleProfileInput } from "./profile";
import { drawStart, drawOver } from "./screens";

export { AppState };
export enum MenuTab { Inventory, Crafting, Profile, Guidebook, Achievements }

const ALL_ACHIEVEMENTS: Achievement[] = [
    { id: "NoviceWalker", name: "Novice Walker", description: "Walk 1,000 steps", stat_bonus: ["speed", 0.01] },
    { id: "MarathonRunner", name: "Marathon Runner", description: "Walk 100,000 steps", stat_bonus: ["speed", 0.05] },
    { id: "FirstBlood", name: "First Blood", description: "Kill 1 mob", stat_bonus: ["damage", 0.01] },
    { id: "MonsterHunter", name: "Monster Hunter", description: "Kill 100 mobs", stat_bonus: ["damage", 0.02] },
    { id: "Slayer", name: "Slayer", description: "Kill 1,000 mobs", stat_bonus: ["damage", 0.05] },
    { id: "Lumberjack", name: "Lumberjack", description: "Chop 100 trees", stat_bonus: ["gather", 0.02] },
    { id: "Deforestation", name: "Deforestation", description: "Chop 1,000 trees", stat_bonus: ["gather", 0.05] },
    { id: "Miner", name: "Miner", description: "Mine 100 rocks", stat_bonus: ["gather", 0.02] },
    { id: "DeepDriller", name: "Deep Driller", description: "Mine 1,000 rocks", stat_bonus: ["gather", 0.05] },
    { id: "ApprenticeSmith", name: "Apprentice Smith", description: "Craft 10 items", stat_bonus: ["craft", 0.02] },
    { id: "MasterSmith", name: "Master Smith", description: "Craft 1,000 items", stat_bonus: ["craft", 0.05] },
    { id: "Builder", name: "Builder", description: "Place 50 structures", stat_bonus: ["max_hp", 0.05] },
    { id: "Architect", name: "Architect", description: "Place 500 structures", stat_bonus: ["max_hp", 0.20] },
];

interface DragState { fromIndex: number; item: Item; startX: number; startY: number; }
interface Toast { ach: Achievement; start: number; }

export class UIManager {
    state: AppState = AppState.StartScreen;
    isMenuOpen: boolean = false; activeTab: MenuTab = MenuTab.Inventory;
    joinRequest: boolean = false; craftRequest: string | null = null; slotSelectRequest: number | null = null;
    respawnRequest: boolean = false; nameUpdateRequest: string | null = null; swapRequest: [number, number] | null = null;
    
    private drag: DragState | null = null;
    private nameBuffer: string = ""; private isNameFocused: boolean = false;
    private toast: Toast | null = null;
    private selectedAchId: string | null = null;
    private selectedRecipe: string | null = null;
    scrollY: number = 0;

    showAchievement(ach: Achievement) { this.toast = { ach, start: Date.now() }; }

    render(ctx: CanvasRenderingContext2D, player: Player | null, input: InputManager) {
        const w = ctx.canvas.width; const h = ctx.canvas.height;
        if (player && this.activeTab === MenuTab.Profile && !this.isNameFocused) this.nameBuffer = player.username;

        if (this.state === AppState.StartScreen) drawStart(ctx, w, h, this);
        else if (this.state === AppState.GameOver) drawOver(ctx, w, h, this);
        else if (this.state === AppState.InGame && player) {
            this.drawHotbar(ctx, player, w, h);
            this.drawHUD(ctx, w);
            if (this.isMenuOpen) this.drawMenu(ctx, player, w, h);
            if (this.drag) { ctx.save(); ctx.globalAlpha = 0.7; this.drawItem(ctx, this.drag.item, input.mouseX - 30, input.mouseY - 30, 60); ctx.restore(); }
            if (this.toast) this.drawToast(ctx, w, h);
        }
    }

    drawHUD(ctx: CanvasRenderingContext2D, w: number) {
        const x = w - 60, y = 20, s = 50;
        ctx.fillStyle = this.isMenuOpen ? "rgba(74,164,74,0.6)" : "rgba(68,68,68,0.6)";
        ctx.fillRect(x, y, s, s); ctx.strokeStyle = "#fff"; ctx.strokeRect(x, y, s, s);
        ctx.fillStyle = "white"; 
        ctx.fillRect(x + 10, y + 12, 30, 4);
        ctx.fillRect(x + 10, y + 23, 30, 4);
        ctx.fillRect(x + 10, y + 34, 30, 4);
    }

    drawMenu(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number) {
        ctx.fillStyle = "rgba(0,0,0,0.6)"; ctx.fillRect(0, 0, w, h);
        const m = 20; const px = m; const py = m; const pw = w - m * 2; const ph = h - m * 2;
        ctx.fillStyle = "rgba(34,34,34,0.95)"; ctx.fillRect(px, py, pw, ph); ctx.strokeStyle = "#444"; ctx.strokeRect(px, py, pw, ph);
        this.drawBtn(ctx, px + pw - 40, py + 10, 30, 30, "X", false);
        
        const tabs = ["Bag", "Craft", "Prof", "Help", "Achiev"]; const tw = (pw - 50) / tabs.length;
        for (let i = 0; i < tabs.length; i++) {
            this.drawBtn(ctx, px + i * tw, py, tw, 50, tabs[i], i === this.activeTab);
        }
        
        ctx.save(); ctx.translate(px, py + 50);
        ctx.beginPath(); ctx.rect(0, 0, pw, ph - 50); ctx.clip();

        if (this.activeTab === MenuTab.Inventory) drawInventory(ctx, player, pw, ph - 50, this.drag, this);
        else if (this.activeTab === MenuTab.Crafting) drawCrafting(ctx, player, this.selectedRecipe, pw, ph - 50, this, this.scrollY);
        else if (this.activeTab === MenuTab.Profile) drawProfile(ctx, player, pw, ph - 50, this.nameBuffer, this.isNameFocused, this);
        else if (this.activeTab === MenuTab.Guidebook) { ctx.fillStyle="white"; ctx.textAlign="left"; let gy=40 - this.scrollY; for(const l of ["WASD: Move","LeftClick/Space: Attack/Use","RightClick/Shift: Interact","1-7: Slot"]){ctx.fillText(l,20,gy);gy+=25;} }
        else if (this.activeTab === MenuTab.Achievements) drawAchievements(ctx, player, ALL_ACHIEVEMENTS, this.selectedAchId, pw, ph - 50, this.scrollY);
        ctx.restore();
    }

    drawToast(ctx: CanvasRenderingContext2D, w: number, h: number) {
        if (!this.toast) return; const age = Date.now() - this.toast.start;
        if (age > 3000) { this.toast = null; return; }
        const alpha = age < 200 ? age/200 : age > 2800 ? (3000-age)/200 : 1;
        ctx.save(); ctx.globalAlpha = alpha;
        const tw = 300; const th = 60; const tx = (w - tw) / 2; const ty = h - 150;
        ctx.fillStyle = "rgba(0,0,0,0.8)"; ctx.fillRect(tx, ty, tw, th); ctx.strokeStyle = "#fb4"; ctx.strokeRect(tx, ty, tw, th);
        ctx.fillStyle = "#fb4"; ctx.font = "bold 16px sans-serif"; ctx.textAlign = "center"; ctx.fillText("ACHIEVEMENT UNLOCKED!", tx+tw/2, ty+20);
        ctx.fillStyle = "white"; ctx.font = "14px sans-serif"; ctx.fillText(this.toast.ach.name, tx+tw/2, ty+45);
        ctx.restore();
    }

    drawHotbar(ctx: CanvasRenderingContext2D, p: Player, w: number, h: number) {
        const slots = 7; const ss = Math.min(50, (w - 80) / 7); const pad = 10; const sx = (w - (slots * (ss + pad))) / 2; const sy = h - ss - 20;
        for (let i = 0; i < slots; i++) {
            const x = sx + i * (ss + pad);
            ctx.fillStyle = i === p.active_slot ? "#aa0" : "#000"; ctx.globalAlpha = 0.5; ctx.fillRect(x, sy, ss, ss); ctx.globalAlpha = 1;
            ctx.strokeStyle = "#fff"; ctx.strokeRect(x, sy, ss, ss);
            const item = p.inventory.slots[i]; if (item && (!this.drag || this.drag.fromIndex !== i)) this.drawItem(ctx, item, x, sy, ss);
        }
        const active = p.inventory.slots[p.active_slot];
        if (active) {
             ctx.fillStyle = "white"; ctx.font = "bold 16px sans-serif"; ctx.textAlign = "center";
             ctx.fillText(active.kind, w / 2, sy - 10);
        }
    }

    drawItem(ctx: CanvasRenderingContext2D, item: Item, x: number, y: number, s: number) {
        ctx.fillStyle = item.kind.includes("Wood") ? "#852" : item.kind.includes("Stone") ? "#888" : item.kind.includes("Meat") ? "#f88" : "#e22";
        ctx.beginPath(); ctx.arc(x + s / 2, y + s / 2, s / 3, 0, Math.PI * 2); ctx.fill();
        ctx.fillStyle = "white"; ctx.font = "12px sans-serif"; ctx.textAlign = "right"; ctx.fillText(item.amount.toString(), x + s - 4, y + s - 4);
        if (item.level && item.level > 1) {
            ctx.fillStyle = "#fb4"; ctx.textAlign = "left"; ctx.fillText(`Lv.${item.level}`, x + 4, y + s - 4);
        }
    }

    drawBtn(ctx: CanvasRenderingContext2D, x: number, y: number, w: number, h: number, l: string, a: boolean) {
        ctx.fillStyle = a ? "#4a4" : "#444"; ctx.fillRect(x, y, w, h); ctx.strokeStyle = "#fff"; ctx.strokeRect(x, y, w, h);
        ctx.fillStyle = "white"; ctx.textAlign = "center"; ctx.textBaseline = "middle"; ctx.font = "14px sans-serif"; ctx.fillText(l, x + w / 2, y + h / 2);
    }

    handleInput(input: InputManager, w: number, h: number, p: Player | null) {
        if (this.isNameFocused) {
            const k = input.keyQueue.shift();
            if (k === "Enter") { this.nameUpdateRequest = this.nameBuffer; this.isNameFocused = false; }
            else if (k === "Backspace") this.nameBuffer = this.nameBuffer.slice(0, -1);
            else if (k && k.length === 1) this.nameBuffer += k;
        } else input.keyQueue = [];

        if (this.isMenuOpen) {
            const d = input.getZoomDelta();
            if (d !== 0) { this.scrollY = Math.max(0, this.scrollY + d * 500); }
        }

        if (input.isPointerDown) {
            const mx = input.mouseX; const my = input.mouseY;
            if (this.state === AppState.StartScreen && this.hit(mx, my, (w - 200) / 2, h / 2, 200, 60)) { this.joinRequest = true; }
            else if (this.state === AppState.GameOver && this.hit(mx, my, (w - 300) / 2, h / 2, 300, 80)) { this.respawnRequest = true; }
            else if (this.state === AppState.InGame) {
                if (this.isMenuOpen) {
                    const m = 20; const px = m; const py = m; const pw = w - m * 2; const ph = h - m * 2;
                    if (this.hit(mx, my, px + pw - 40, py + 10, 30, 30)) this.isMenuOpen = false;
                    else if (this.hit(mx, my, px, py, pw, 50)) { this.activeTab = Math.floor((mx - px) / (pw / 5)); this.scrollY = 0; }
                    else {
                        const contentX = mx - px; const contentY = my - (py + 50);
                        if (contentX >= 0 && contentY >= 0 && contentX <= pw && contentY <= ph - 50) {
                            if (this.activeTab === MenuTab.Inventory && p) {
                                const res = handleInvInput(contentX, contentY, pw, ph - 50, p, this);
                                if (res) this.drag = res;
                            } else if (this.activeTab === MenuTab.Crafting && p) {
                                const res = handleCraftInput(contentX, contentY, pw, ph - 50, p, this.selectedRecipe, this.scrollY);
                                if (res.select) this.selectedRecipe = res.select;
                                if (res.craft) this.craftRequest = this.selectedRecipe;
                            } else if (this.activeTab === MenuTab.Profile && p) {
                                const res = handleProfileInput(contentX, contentY, pw, ph - 50, p, this);
                                if (res.focus) { this.isNameFocused = true; this.nameBuffer = p.username; }
                                else if (res.update) { this.nameUpdateRequest = this.nameBuffer; this.isNameFocused = false; }
                                else this.isNameFocused = false;
                            } else if (this.activeTab === MenuTab.Achievements) {
                                const res = handleAchInput(contentX, contentY, pw, ph - 50, ALL_ACHIEVEMENTS, this.scrollY);
                                if (res) this.selectedAchId = res;
                            }
                        }
                    }
                } else {
                    if (this.hit(mx, my, w - 60, 20, 50, 50)) this.isMenuOpen = true;
                    // Hotbar logic
                    const slots = 7; const ss = Math.min(50, (w - 80) / 7); const pad = 10; 
                    const sx = (w - (slots * (ss + pad))) / 2; const sy = h - ss - 20;
                    if (this.hit(mx, my, sx, sy, slots * (ss + pad), ss)) {
                        const idx = Math.floor((mx - sx) / (ss + pad));
                        if (idx >= 0 && idx < 7) { this.slotSelectRequest = idx; }
                    }
                }
            }
            input.isPointerDown = false; // Consume click
        }
        if (this.drag && !input.isPointerDown && p) {
             const mx = input.mouseX; const my = input.mouseY;
             const m = 20; const px = m; const py = m; const pw = w - m * 2; const ph = h - m * 2;
             if (this.activeTab === MenuTab.Inventory && this.hit(mx, my, px, py + 50, pw, ph - 50)) {
                 // Simple drop (swap) if released over inventory
                 // Re-use handleInvInput to find slot
                 const res = handleInvInput(mx - px, my - (py+50), pw, ph - 50, p, this);
                 if (res && res.fromIndex !== this.drag.fromIndex) {
                     this.swapRequest = [this.drag.fromIndex, res.fromIndex];
                 }
             }
            this.drag = null;
        }
    }
    hit(mx: number, my: number, x: number, y: number, w: number, h: number) { return mx >= x && mx <= x + w && my >= y && my <= y + h; }
}