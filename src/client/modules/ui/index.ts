import { InputManager } from "../input";
// UIManager handles UI rendering and input
import { Player, Item, Achievement, AppState, NpcInteraction } from "../../types";
import { drawInventory, handleInvInput } from "./inventory";
import { drawCrafting, handleCraftInput } from "./crafting";
import { drawAchievements, handleAchInput } from "./achievements";
import { drawProfile, handleProfileInput } from "./profile";
import { drawQuests, handleQuestsInput } from "./quests";
import { drawOver } from "./screens";

export { AppState };
export enum MenuTab { Inventory, Crafting, Profile, Quests, Achievements }

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
interface Toast { title: string; message: string; color: string; start: number; }

export class UIManager {
    state: AppState = AppState.InGame;
    isMenuOpen = false;
    pinnedQuestId: string | null = null;
    pinnedAchId: string | null = null;
    npcInteraction: NpcInteraction | null = null;
    npcActionRequest: { npc_id: string, option_index: number } | null = null;
    tradeRequest: { npc_id: string, item_index: number, buy: boolean } | null = null;
    activeTab: MenuTab = MenuTab.Inventory;
    scrollY = 0;
    selectedRecipe: string | null = null;
    selectedAchId: string | null = null;
    isNameFocused = false;
    nameBuffer = "";
    nameUpdateRequest: string | null = null;
    respawnRequest = false;
    craftRequest: string | null = null;
    slotSelectRequest: number | null = null;
    drag: DragState | null = null;
    swapRequest: [number, number] | null = null;
    private toast: Toast | null = null;

    showAchievement(ach: Achievement) { 
        this.toast = { title: "ACHIEVEMENT UNLOCKED!", message: ach.name, color: "#fb4", start: Date.now() }; 
    }

    showNotification(title: string, message: string, color: string) {
        this.toast = { title, message, color, start: Date.now() };
    }

    render(ctx: CanvasRenderingContext2D, p: Player | null, input: InputManager) {
        const dpr = window.devicePixelRatio || 1;
        const w = ctx.canvas.width / dpr;
        const h = ctx.canvas.height / dpr;
        this.handleInput(input, w, h, p);

        if (this.state === AppState.GameOver) drawOver(ctx, w, h, this);
        else if (this.state === AppState.InGame && p) {
            this.drawHotbar(ctx, p, w, h);
            this.drawPinnedTracker(ctx, p, w, h);
            this.drawMenu(ctx, p, w, h);
            if (this.npcInteraction) this.drawNpcInteraction(ctx, w, h);
        }
        this.drawToast(ctx, w, h);
        if (this.drag && input.isPointerDown) {
            this.drawItem(ctx, this.drag.item, input.mouseX - 25, input.mouseY - 25, 50);
        }
    }

    private drawMenu(ctx: CanvasRenderingContext2D, p: Player, w: number, h: number) {
        this.drawBtn(ctx, w - 60, 20, 50, 50, "☰", this.isMenuOpen, 30);
        if (!this.isMenuOpen) return;

        const m = 20; const px = m; const py = m; const pw = w - m * 2; const ph = h - m * 2;
        ctx.fillStyle = "rgba(0,0,0,0.9)"; ctx.fillRect(px, py, pw, ph);
        ctx.strokeStyle = "#fff"; ctx.strokeRect(px, py, pw, ph);

        const tabs = ["INV", "CRAFT", "PROF", "QUESTS", "ACH"];
        const tw = (pw - 60) / tabs.length; // Leave space for close button
        for (let i = 0; i < tabs.length; i++) {
            this.drawBtn(ctx, px + i * tw, py, tw, 50, tabs[i], this.activeTab === i);
        }
        this.drawBtn(ctx, px + pw - 50, py + 5, 40, 40, "X", false, 20);

        ctx.save();
        ctx.beginPath(); ctx.rect(px, py + 50, pw, ph - 50); ctx.clip();
        ctx.translate(px, py + 50);
        const cw = pw; const ch = ph - 50;
        if (this.activeTab === MenuTab.Inventory) drawInventory(ctx, p, cw, ch, this.drag, this);
        else if (this.activeTab === MenuTab.Crafting) drawCrafting(ctx, p, this.selectedRecipe, cw, ch, this, this.scrollY);
        else if (this.activeTab === MenuTab.Profile) drawProfile(ctx, p, cw, ch, this.nameBuffer, this.isNameFocused, this);
        else if (this.activeTab === MenuTab.Quests) drawQuests(ctx, p, cw, ch, this, this.scrollY);
        else if (this.activeTab === MenuTab.Achievements) drawAchievements(ctx, p, ALL_ACHIEVEMENTS, this.selectedAchId, cw, ch, this.scrollY, this);
        ctx.restore();
    }

    drawToast(ctx: CanvasRenderingContext2D, w: number, h: number) {
        if (!this.toast) return; const age = Date.now() - this.toast.start;
        if (age > 3000) { this.toast = null; return; }
        const alpha = age < 200 ? age/200 : age > 2800 ? (3000-age)/200 : 1;
        ctx.save(); ctx.globalAlpha = alpha;
        const tw = 300; const th = 60; const tx = (w - tw) / 2; const ty = h - 150;
        ctx.fillStyle = "rgba(0,0,0,0.8)"; ctx.fillRect(tx, ty, tw, th); 
        ctx.strokeStyle = this.toast.color; ctx.strokeRect(tx, ty, tw, th);
        ctx.fillStyle = this.toast.color; ctx.font = "bold 16px sans-serif"; ctx.textAlign = "center"; ctx.fillText(this.toast.title, tx+tw/2, ty+20);
        ctx.fillStyle = "white"; ctx.font = "14px sans-serif"; ctx.fillText(this.toast.message, tx+tw/2, ty+45);
        ctx.restore();
    }

    drawHotbar(ctx: CanvasRenderingContext2D, p: Player, w: number, h: number) {
        const slots = 7;
        const ss = Math.min(50, (w - 40) / 7);
        const pad = 6;
        const sx = (w - (slots * (ss + pad))) / 2;
        const sy = h - ss - 20;

        for (let i = 0; i < slots; i++) {
            const x = sx + i * (ss + pad);
            ctx.fillStyle = i === p.active_slot ? "rgba(255, 255, 0, 0.3)" : "rgba(0, 0, 0, 0.5)";
            ctx.fillRect(x, sy, ss, ss);
            ctx.strokeStyle = i === p.active_slot ? "#ff0" : "#fff";
            ctx.lineWidth = i === p.active_slot ? 2 : 1;
            ctx.strokeRect(x, sy, ss, ss);
            const item = p.inventory.slots[i];
            if (item && (!this.drag || this.drag.fromIndex !== i)) this.drawItem(ctx, item, x, sy, ss);
        }
        const active = p.inventory.slots[p.active_slot];
        if (active) {
             ctx.fillStyle = "white"; ctx.font = "bold 14px sans-serif"; ctx.textAlign = "center";
             ctx.fillText(active.kind, w / 2, sy - 10);
        }
    }

    drawItem(ctx: CanvasRenderingContext2D, item: Item, x: number, y: number, s: number) {
        ctx.fillStyle = item.kind.includes("Wood") ? "#852" : item.kind.includes("Stone") ? "#888" : item.kind.includes("Meat") ? "#f88" : "#e22";
        ctx.beginPath(); ctx.arc(x + s / 2, y + s / 2, s / 3, 0, Math.PI * 2); ctx.fill();
        ctx.fillStyle = "white"; ctx.font = `${Math.max(10, s/4)}px sans-serif`; ctx.textAlign = "right"; ctx.fillText(item.amount.toString(), x + s - 4, y + s - 4);
        if (item.level && item.level > 1) {
            ctx.fillStyle = "#fb4"; ctx.textAlign = "left"; ctx.fillText(`Lv.${item.level}`, x + 4, y + s - 4);
        }
    }

    drawBtn(ctx: CanvasRenderingContext2D, x: number, y: number, w: number, h: number, l: string, a: boolean, fontSize: number = 14) {
        ctx.fillStyle = a ? "rgba(74, 164, 74, 0.9)" : "rgba(68, 68, 68, 0.8)";
        ctx.fillRect(x, y, w, h);
        ctx.strokeStyle = "#fff";
        ctx.lineWidth = 1;
        ctx.strokeRect(x, y, w, h);
        ctx.fillStyle = "white"; ctx.textAlign = "center"; ctx.textBaseline = "middle"; ctx.font = `bold ${fontSize}px sans-serif`; ctx.fillText(l, x + w / 2, y + h / 2);
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
            let consumed = false;

            if (this.state === AppState.GameOver && this.hit(mx, my, (w - 300) / 2, h / 2, 300, 80)) {
                this.respawnRequest = true;
                consumed = true;
            } else if (this.state === AppState.InGame) {
                if (this.npcInteraction) {
                    // ... (no changes to npcInteraction coordinates needed)
                    consumed = true;
                    const dw = Math.min(400, w - 40);
                    const dh = 250;
                    const dx = (w - dw) / 2;
                    const dy = (h - dh) / 2;
                    const optH = 30;
                    this.npcInteraction.options.forEach((opt, i) => {
                        const oy = dy + dh - (this.npcInteraction!.options.length - i) * (optH + 10) - 10;
                        if (this.hit(mx, my, dx + 20, oy, dw - 40, optH)) {
                            if (opt === "Goodbye" || opt === "Close" || opt === "Okay") {
                                this.npcInteraction = null;
                            } else {
                                this.npcActionRequest = { npc_id: this.npcInteraction!.npc_id, option_index: i };
                            }
                        }
                    });
                } else if (this.isMenuOpen) {
                    consumed = true; // Any click while menu is open is consumed
                    const m = 20; const px = m; const py = m; const pw = w - m * 2; const ph = h - m * 2;
                    if (this.hit(mx, my, px + pw - 50, py + 5, 40, 40)) this.isMenuOpen = false;
                    else if (this.hit(mx, my, px, py, pw - 60, 50)) { this.activeTab = Math.floor((mx - px) / ((pw - 60) / 5)); this.scrollY = 0; }
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
                            } else if (this.activeTab === MenuTab.Quests && p) {
                                const res = handleQuestsInput(contentX, contentY, pw, ph - 50, p, this, this.scrollY);
                                if (res?.pin) this.pinnedQuestId = (this.pinnedQuestId === res.pin) ? null : res.pin;
                            } else if (this.activeTab === MenuTab.Profile && p) {
                                const res = handleProfileInput(contentX, contentY, pw, ph - 50, p, this);
                                if (res.focus) { this.isNameFocused = true; this.nameBuffer = p.username; }
                                else if (res.update) { this.nameUpdateRequest = this.nameBuffer; this.isNameFocused = false; }
                                else this.isNameFocused = false;
                            } else if (this.activeTab === MenuTab.Achievements) {
                                const res = handleAchInput(contentX, contentY, pw, ph - 50, ALL_ACHIEVEMENTS, this.selectedAchId, this.scrollY);
                                if (res?.select) this.selectedAchId = res.select;
                                if (res?.pin) this.pinnedAchId = (this.pinnedAchId === res.pin) ? null : res.pin;
                            }
                        }
                    }
                } else {
                    if (this.hit(mx, my, w - 60, 20, 50, 50)) {
                        this.isMenuOpen = true;
                        consumed = true;
                    }
                    // Hotbar logic
                    const slots = 7;
                    const ss = Math.min(50, (w - 40) / 7);
                    const pad = 6;
                    const sx = (w - (slots * (ss + pad))) / 2;
                    const sy = h - ss - 20;
                    if (this.hit(mx, my, sx, sy, slots * (ss + pad), ss)) {
                        const idx = Math.floor((mx - sx) / (ss + pad));
                        if (idx >= 0 && idx < 7) { this.slotSelectRequest = idx; }
                        consumed = true;
                    }
                }
            }
            if (consumed) input.isPointerDown = false;
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

    private drawNpcInteraction(ctx: CanvasRenderingContext2D, w: number, h: number) {
        if (!this.npcInteraction) return;
        const dw = Math.min(400, w - 40);
        const dh = 250;
        const dx = (w - dw) / 2;
        const dy = (h - dh) / 2;

        ctx.fillStyle = "rgba(0,0,0,0.9)"; ctx.fillRect(dx, dy, dw, dh);
        ctx.strokeStyle = "#0ff"; ctx.strokeRect(dx, dy, dw, dh);

        ctx.fillStyle = "#0ff"; ctx.font = "bold 18px sans-serif"; ctx.textAlign = "left";
        ctx.fillText(this.npcInteraction.name, dx + 20, dy + 35);

        ctx.fillStyle = "white"; ctx.font = "14px sans-serif";
        const lines = this.wrapText(ctx, this.npcInteraction.text, dw - 40);
        lines.forEach((line, i) => ctx.fillText(line, dx + 20, dy + 70 + i * 20));

        const optH = 30;
        this.npcInteraction.options.forEach((opt, i) => {
            const oy = dy + dh - (this.npcInteraction!.options.length - i) * (optH + 10) - 10;
            this.drawBtn(ctx, dx + 20, oy, dw - 40, optH, opt, false);
        });
    }

    private wrapText(ctx: CanvasRenderingContext2D, text: string, maxWidth: number): string[] {
        const words = text.split(' ');
        const lines = [];
        let currentLine = words[0];
        for (let i = 1; i < words.length; i++) {
            const word = words[i];
            const width = ctx.measureText(currentLine + " " + word).width;
            if (width < maxWidth) currentLine += " " + word;
            else { lines.push(currentLine); currentLine = word; }
        }
        lines.push(currentLine);
        return lines;
    }

    private drawPinnedTracker(ctx: CanvasRenderingContext2D, p: Player, w: number, h: number) {
        if (!this.pinnedQuestId && !this.pinnedAchId) return;
        
        ctx.save();
        const tw = 200;
        let ty = 100;
        const tx = w - tw - 20;

        if (this.pinnedQuestId) {
            const q = p.quests.find(q => q.id === this.pinnedQuestId);
            if (q) {
                this.drawTrackerBox(ctx, tx, ty, tw, q.name, q.state);
                q.objectives.forEach((obj, i) => {
                    let txt = ""; let pct = 0;
                    if ("Gather" in obj) { txt = `${obj.Gather.item}: ${obj.Gather.current}/${obj.Gather.count}`; pct = obj.Gather.current/obj.Gather.count; }
                    else if ("Kill" in obj) { txt = `${obj.Kill.mob_type}: ${obj.Kill.current}/${obj.Kill.count}`; pct = obj.Kill.current/obj.Kill.count; }
                    else if ("TalkTo" in obj) { txt = `Talk to ${obj.TalkTo.npc_name}`; pct = q.state === "ReadyToTurnIn" ? 1 : 0; }
                    this.drawTrackerObjective(ctx, tx + 10, ty + 35 + i * 20, tw - 20, txt, pct);
                });
                ty += 40 + q.objectives.length * 20;
            }
        }

        if (this.pinnedAchId) {
            const ach = ALL_ACHIEVEMENTS.find(a => a.id === this.pinnedAchId);
            if (ach) {
                const isUnlocked = p.achievements.includes(ach.id);
                this.drawTrackerBox(ctx, tx, ty, tw, ach.name, isUnlocked ? "Unlocked" : "Progress");
                if (!isUnlocked && ach.requirement) {
                    const current = this.getProgressValue(p, ach.requirement.type);
                    const pct = Math.min(1, current / ach.requirement.value);
                    this.drawTrackerObjective(ctx, tx + 10, ty + 35, tw - 20, `${current}/${ach.requirement.value}`, pct);
                }
            }
        }
        ctx.restore();
    }

    private drawTrackerBox(ctx: CanvasRenderingContext2D, x: number, y: number, w: number, title: string, sub: string) {
        ctx.fillStyle = "rgba(0,0,0,0.5)"; ctx.fillRect(x, y, w, 25);
        ctx.strokeStyle = "rgba(255,255,255,0.2)"; ctx.strokeRect(x, y, w, 25);
        ctx.fillStyle = "#ff0"; ctx.font = "bold 12px sans-serif"; ctx.textAlign = "left";
        ctx.fillText(title, x + 5, y + 12);
        ctx.fillStyle = "#aaa"; ctx.textAlign = "right"; ctx.font = "10px sans-serif";
        ctx.fillText(sub, x + w - 5, y + 12);
    }

    private drawTrackerObjective(ctx: CanvasRenderingContext2D, x: number, y: number, w: number, text: string, pct: number) {
        ctx.fillStyle = "white"; ctx.font = "11px sans-serif"; ctx.textAlign = "left";
        ctx.fillText(text, x, y + 10);
        ctx.fillStyle = "#222"; ctx.fillRect(x, y + 15, w, 4);
        ctx.fillStyle = pct >= 1 ? "#4f4" : "#fb4"; ctx.fillRect(x, y + 15, w * Math.min(1, pct), 4);
    }

    private getProgressValue(p: Player, type: string): number {
        if (!p.stats) return 0;
        switch(type) {
            case "Steps": return p.stats.steps_taken;
            case "Kills": return p.stats.mobs_killed;
            case "Resources": return p.stats.resources_gathered;
            case "Crafts": return p.stats.items_crafted;
            case "Structures": return p.stats.structures_placed;
            case "ToolLevel": {
                let max = 1;
                for (const s of p.inventory.slots) if (s && s.level && s.level > max) max = s.level;
                return max;
            }
            default: return 0;
        }
    }
}