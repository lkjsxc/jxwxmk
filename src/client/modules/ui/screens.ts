export function drawStart(ctx: CanvasRenderingContext2D, w: number, h: number, ui: any) {
    ctx.fillStyle = "rgba(17, 17, 17, 0.7)"; ctx.fillRect(0, 0, w, h); 
    ctx.fillStyle = "#eee"; ctx.font = "bold 60px sans-serif"; ctx.textAlign = "center"; 
    ctx.fillText("kkmypk", w / 2, h / 3);
    ui.drawBtn(ctx, (w - 200) / 2, h / 2, 200, 60, "PLAY", true);
}

export function drawOver(ctx: CanvasRenderingContext2D, w: number, h: number, ui: any) {
    // No red background, per request
    ctx.fillStyle = "rgba(0,0,0,0.6)"; ctx.fillRect(0, 0, w, h); 
    ctx.fillStyle = "white"; ctx.font = "bold 60px sans-serif"; ctx.textAlign = "center"; 
    ctx.fillText("YOU DIED", w / 2, h / 3);
    ui.drawBtn(ctx, (w - 300) / 2, h / 2, 300, 80, "Return to Menu", true);
}
