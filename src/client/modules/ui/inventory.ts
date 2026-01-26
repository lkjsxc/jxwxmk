import { Player, Item } from "../../types";

export function drawInventory(ctx: CanvasRenderingContext2D, player: Player, w: number, h: number, drag: any, ui: any) {
    const { cols, size, pad } = getInvLayout(w);
    const startX = (w - (cols * size + (cols - 1) * pad)) / 2;
    for (let i = 0; i < 30; i++) {
        const x = startX + (i % cols) * (size + pad);
        const y = 40 + Math.floor(i / cols) * (size + pad);
        ctx.fillStyle = "#111"; ctx.fillRect(x, y, size, size);
        ctx.strokeStyle = "#555"; ctx.strokeRect(x, y, size, size);
        const item = player.inventory.slots[i];
        if (item && (!drag || drag.fromIndex !== i)) ui.drawItem(ctx, item, x, y, size);
    }
}

export function getInvLayout(w: number) {
    const c = w > 600 ? 7 : w > 400 ? 5 : 3;
    const p = 10;
    const s = Math.min(60, (w - (c + 1) * p) / c);
    return { cols: c, size: s, pad: p };
}

export function handleInvInput(mx: number, my: number, w: number, h: number, player: Player, ui: any): any {
    const { cols, size, pad } = getInvLayout(w);
    const startX = (w - (cols * size + (cols - 1) * pad)) / 2;
    for (let i = 0; i < 30; i++) {
        const x = startX + (i % cols) * (size + pad);
        const y = 40 + Math.floor(i / cols) * (size + pad);
        if (ui.hit(mx, my, x, y, size, size) && player.inventory.slots[i]) {
            return { fromIndex: i, item: player.inventory.slots[i]!, startX: mx, startY: my };
        }
    }
    return null;
}
