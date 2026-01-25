import { InputManager } from "./input";
import { Player, Item, Achievement } from "../types";

export enum AppState { StartScreen, InGame, GameOver }
export enum MenuTab { Inventory, Crafting, Profile, Guidebook, Achievements }

interface DragState { fromIndex: number; item: Item; startX: number; startY: number; }
interface Toast { ach: Achievement; start: number; }

export class UIManager {
    state: AppState = AppState.StartScreen;
    isMenuOpen: boolean = false;
    activeTab: MenuTab = MenuTab.Inventory;
    joinRequest: boolean = false; craftRequest: string | null = null; slotSelectRequest: number | null = null;
    respawnRequest: boolean = false; nameUpdateRequest: string | null = null; swapRequest: [number, number] | null = null;
    private drag: DragState | null = null; private nameBuffer: string = ""; private isNameFocused: boolean = false;
    private toast: Toast | null = null;

    showAchievement(ach: Achievement) { this.toast = { ach, start: Date.now() }; }

    render(ctx: CanvasRenderingContext2D, player: Player | null, input: InputManager) {
        const w = ctx.canvas.width; const h = ctx.canvas.height;
        if (player && this.activeTab === MenuTab.Profile && !this.isNameFocused) this.nameBuffer = player.username;
        if (this.state === AppState.StartScreen) this.drawStart(ctx, w, h);
        else if (this.state === AppState.GameOver) this.drawOver(ctx, w, h);
        else if (this.state === AppState.InGame && player) {
            this.drawHotbar(ctx, player, w, h); this.drawHUD(ctx, w);
            if (this.isMenuOpen) this.drawMenu(ctx, player, w, h);
            if (this.drag) { ctx.save(); ctx.globalAlpha = 0.7; this.drawItem(ctx, this.drag.item, input.mouseX - 30, input.mouseY - 30, 60); ctx.restore(); }
            if (this.toast) this.drawToast(ctx, w, h);
        }
    }

    drawStart(ctx: CanvasRenderingContext2D, w: number, h: number) {
        ctx.fillStyle = "#111"; ctx.fillRect(0, 0, w, h); ctx.fillStyle = "#eee"; ctx.font = "bold 60px sans-serif"; ctx.textAlign = "center"; ctx.fillText("kkmypk", w / 2, h / 3);
        this.drawBtn(ctx, (w - 200) / 2, h / 2, 200, 60, "PLAY", true);
    }
    drawOver(ctx: CanvasRenderingContext2D, w: number, h: number) {
        ctx.fillStyle = "rgba(100,0,0,0.8)"; ctx.fillRect(0, 0, w, h); ctx.fillStyle = "white"; ctx.font = "bold 60px sans-serif"; ctx.textAlign = "center"; ctx.fillText("YOU DIED", w / 2, h / 3);
        this.drawBtn(ctx, (w - 300) / 2, h / 2, 300, 80, "RESPAWN", true);
    }
    drawHUD(ctx: CanvasRenderingContext2D, w: number) { this.drawBtn(ctx, w - 60, 20, 50, 50, "MENU", this.isMenuOpen); }
    
    drawMenu(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number) {
        ctx.fillStyle = "rgba(0,0,0,0.6)"; ctx.fillRect(0, 0, w, h);
        const m = 20; const px = m; const py = m; const pw = w - m * 2; const ph = h - m * 2;
        ctx.fillStyle = "rgba(34,34,34,0.9)"; ctx.fillRect(px, py, pw, ph); ctx.strokeStyle = "#444"; ctx.strokeRect(px, py, pw, ph);
        this.drawBtn(ctx, px + pw - 40, py + 10, 30, 30, "X", false);
        const tabs = ["Bag", "Craft", "Prof", "Help", "Achiev"]; const tw = (pw - 50) / tabs.length;
        for (let i = 0; i < tabs.length; i++) {
            this.drawBtn(ctx, px + i * tw, py, tw, 50, tabs[i], i === this.activeTab);
        }
        ctx.save(); ctx.translate(px, py + 50);
        if (this.activeTab === MenuTab.Inventory) this.drawInv(ctx, player, pw);
        else if (this.activeTab === MenuTab.Crafting) this.drawCraft(ctx, pw);
        else if (this.activeTab === MenuTab.Profile) this.drawProf(ctx, player, pw);
        else if (this.activeTab === MenuTab.Guidebook) this.drawGuide(ctx);
        else if (this.activeTab === MenuTab.Achievements) this.drawAch(ctx, player, pw, ph - 50);
        ctx.restore();
    }
    drawInv(ctx: CanvasRenderingContext2D, p: Player, w: number) {
        const { cols, size, pad } = this.getInvLayout(w); const startX = (w - (cols * size + (cols - 1) * pad)) / 2;
        for (let i = 0; i < 30; i++) {
            const x = startX + (i % cols) * (size + pad); const y = 40 + Math.floor(i / cols) * (size + pad);
            ctx.fillStyle = "#111"; ctx.fillRect(x, y, size, size); ctx.strokeStyle = "#555"; ctx.strokeRect(x, y, size, size);
            const item = p.inventory.slots[i]; if (item && (!this.drag || this.drag.fromIndex !== i)) this.drawItem(ctx, item, x, y, size);
        }
    }
    drawCraft(ctx: CanvasRenderingContext2D, w: number) {
        const r = [{n:"Pick(W)",c:"WoodPickaxe",q:"10W"}, {n:"Pick(S)",c:"StonePickaxe",q:"10W,10S"}, {n:"Wall",c:"WoodWall",q:"20W"}, {n:"Torch",c:"Torch",q:"2W"}];
        let y = 40; const bw = Math.min(260, w - 40); const x = (w - bw) / 2;
        for (const i of r) { this.drawBtn(ctx, x, y, bw, 45, `${i.n} (${i.q})`, false); y += 55; }
    }
    drawProf(ctx: CanvasRenderingContext2D, p: Player, w: number) {
        ctx.fillStyle = "white"; ctx.textAlign = "center"; ctx.fillText(p.username, w / 2, 80);
        const bw = 240; const bh = 45; const bx = (w - bw) / 2;
        ctx.fillStyle = this.isNameFocused ? "#000" : "#111"; ctx.fillRect(bx, 120, bw, bh); ctx.strokeStyle = this.isNameFocused ? "#4a4" : "#555"; ctx.strokeRect(bx, 120, bw, bh);
        ctx.fillStyle = "white"; ctx.textAlign = "left"; ctx.fillText((this.nameBuffer||"Type...") + (this.isNameFocused && Date.now()%1000<500?"|":""), bx+10, 150);
        this.drawBtn(ctx, (w - 160) / 2, 185, 160, 40, "Update Name", false);
    }
    drawGuide(ctx: CanvasRenderingContext2D) {
        ctx.fillStyle = "white"; ctx.textAlign = "left"; let y = 40; for (const l of ["WASD: Move", "A: Attack/Use", "B: Interact", "1-7: Slot"]) { ctx.fillText(l, 20, y); y += 25; }
    }
    drawAch(ctx: CanvasRenderingContext2D, p: Player, w: number, h: number) {
        ctx.fillStyle = "white"; ctx.textAlign = "center"; ctx.font = "20px sans-serif"; ctx.fillText(`Unlocked: ${p.achievements.length}`, w/2, 30);
        let y = 60; const achs = p.achievements;
        ctx.font = "16px sans-serif"; ctx.textAlign = "left";
        if (achs.length === 0) { ctx.textAlign = "center"; ctx.fillStyle = "#aaa"; ctx.fillText("No achievements yet.", w/2, y); return; }
        for (const id of achs) {
            ctx.fillStyle = "#fb4"; ctx.fillText(`★ ${id}`, 40, y); y += 30;
            if (y > h - 20) break;
        }
    }
    drawToast(ctx: CanvasRenderingContext2D, w: number, h: number) {
        if (!this.toast) return; const age = Date.now() - this.toast.start;
        if (age > 3000) { this.toast = null; return; }
        const alpha = age < 200 ? age/200 : age > 2800 ? (3000-age)/200 : 1;
        ctx.save(); ctx.globalAlpha = alpha;
        const tw = 300; const th = 60; const tx = (w - tw) / 2; const ty = h - 150;
        ctx.fillStyle = "rgba(0,0,0,0.8)"; ctx.fillRect(tx, ty, tw, th); ctx.strokeStyle = "#fb4"; ctx.lineWidth = 2; ctx.strokeRect(tx, ty, tw, th);
        ctx.fillStyle = "#fb4"; ctx.font = "bold 16px sans-serif"; ctx.textAlign = "center"; ctx.fillText("ACHIEVEMENT UNLOCKED!", tx + tw/2, ty + 20);
        ctx.fillStyle = "white"; ctx.font = "14px sans-serif"; ctx.fillText(this.toast.ach.name, tx + tw/2, ty + 45);
        ctx.restore();
    }
    drawHotbar(ctx: CanvasRenderingContext2D, p: Player, w: number, h: number) {
        const slots = 7; const ss = Math.min(50, (w - 80) / 7); const pad = 10; const sx = (w - (slots*(ss+pad))) / 2; const sy = h - ss - 20;
        for (let i = 0; i < slots; i++) {
            const x = sx + i * (ss + pad); ctx.fillStyle = i === p.active_slot ? "#aa0" : "#000"; ctx.globalAlpha=0.5; ctx.fillRect(x, sy, ss, ss); ctx.globalAlpha=1;
            ctx.strokeStyle = "#fff"; ctx.strokeRect(x, sy, ss, ss); const item = p.inventory.slots[i]; if (item && (!this.drag || this.drag.fromIndex !== i)) this.drawItem(ctx, item, x, sy, ss);
        }
    }
    drawItem(ctx: CanvasRenderingContext2D, item: Item, x: number, y: number, s: number) {
        ctx.fillStyle = item.kind.includes("Wood")?"#852":item.kind.includes("Stone")?"#888":item.kind.includes("Meat")?"#f88":"#e22";
        ctx.beginPath(); ctx.arc(x + s/2, y + s/2, s/3, 0, Math.PI*2); ctx.fill();
        ctx.fillStyle = "white"; ctx.font = "12px sans-serif"; ctx.textAlign = "right"; ctx.fillText(item.amount.toString(), x + s - 4, y + s - 4);
    }
    drawBtn(ctx: CanvasRenderingContext2D, x: number, y: number, w: number, h: number, l: string, a: boolean) {
        ctx.fillStyle = a ? "#4a4" : "#444"; ctx.fillRect(x, y, w, h); ctx.strokeStyle = "#fff"; ctx.strokeRect(x, y, w, h);
        ctx.fillStyle = "white"; ctx.textAlign = "center"; ctx.textBaseline = "middle"; ctx.font = "14px sans-serif"; ctx.fillText(l, x + w / 2, y + h / 2);
    }
    getInvLayout(w: number) { const c = w > 600 ? 7 : w > 400 ? 5 : 3; const p = 10; const s = Math.min(60, (w - (c+1)*p)/c); return { cols: c, size: s, pad: p }; }
    handleInput(input: InputManager, w: number, h: number, p: Player | null) {
        if (this.isNameFocused) { /* keyboard logic omitted for brevity, assuming standard */ 
             const k = input.keyQueue.shift(); if (k === "Enter") { this.nameUpdateRequest = this.nameBuffer; this.isNameFocused = false; } else if (k === "Backspace") this.nameBuffer = this.nameBuffer.slice(0, -1); else if (k && k.length === 1) this.nameBuffer += k;
        } else input.keyQueue = [];
        if (this.state === AppState.InGame && !this.isMenuOpen) { for(let i=1;i<=7;i++) if(input.keys[`num${i}` as any]) this.slotSelectRequest = i-1; }
        if (input.isPointerDown) {
            const mx = input.mouseX; const my = input.mouseY;
            if (this.state === AppState.StartScreen && this.hit(mx, my, (w-200)/2, h/2, 200, 60)) { this.joinRequest = true; input.isPointerDown = false; }
            if (this.state === AppState.GameOver && this.hit(mx, my, (w-300)/2, h/2, 300, 80)) { this.respawnRequest = true; input.isPointerDown = false; }
            if (this.state === AppState.InGame) {
                if (this.isMenuOpen) {
                    const m = 20; const px = m; const py = m; const pw = w - m*2;
                    if (this.hit(mx, my, px+pw-40, py+10, 30, 30)) this.isMenuOpen = false;
                    else if (this.hit(mx, my, px, py, pw, 50)) this.activeTab = Math.floor((mx-px)/(pw/5));
                    else if (this.activeTab === MenuTab.Inventory && p) {
                        const {cols, size, pad} = this.getInvLayout(pw); const sx = px + (pw - (cols*size + (cols-1)*pad))/2;
                        for(let i=0;i<30;i++) { const x = sx + (i%cols)*(size+pad); const y = py+90 + Math.floor(i/cols)*(size+pad); if(this.hit(mx, my, x, y, size, size) && p.inventory.slots[i]) this.drag = {fromIndex: i, item: p.inventory.slots[i]!, startX: mx, startY: my}; }
                    } else if (this.activeTab === MenuTab.Crafting) {
                         const r = ["WoodPickaxe", "StonePickaxe", "WoodWall", "Torch"]; let ry = py+90; const bw = Math.min(260, pw-40);
                         for (const c of r) { if (this.hit(mx, my, px+(pw-bw)/2, ry, bw, 45)) this.craftRequest = c; ry+=55; }
                    } else if (this.activeTab === MenuTab.Profile) {
                        if (this.hit(mx, my, px+(pw-160)/2, py+235, 160, 40)) { this.nameUpdateRequest = this.nameBuffer; this.isNameFocused = false; }
                        else if (this.hit(mx, my, px+(pw-240)/2, py+170, 240, 45)) { this.isNameFocused = true; this.nameBuffer = p?.username||""; }
                    }
                } else {
                    if (this.hit(mx, my, w-60, 20, 50, 50)) this.isMenuOpen = true;
                }
                input.isPointerDown = false;
            }
        }
        if (this.drag && !input.isPointerDown) { /* swap logic */ this.drag = null; }
    }
    hit(mx: number, my: number, x: number, y: number, w: number, h: number) { return mx>=x && mx<=x+w && my>=y && my<=y+h; }
}