import { Player, Quest, ObjectiveType } from "../../types";
import { UIManager } from "./index";

export function drawQuests(ctx: CanvasRenderingContext2D, p: Player, w: number, h: number, ui: UIManager, scrollY: number) {
    ctx.fillStyle = "white"; ctx.font = "bold 18px sans-serif"; ctx.textAlign = "left"; ctx.textBaseline = "top";
    ctx.fillText("Quest Log", 20, 10);

    let y = 50 - scrollY;
    p.quests.forEach(quest => {
        const descLines = ui.wrapText(ctx, quest.description, w - 100);
        const cardH = 100 + descLines.length * 20;
        const spacing = cardH + 10;
        if (y + cardH < 0 || y > h) { y += spacing; return; }

        ctx.fillStyle = "rgba(255,255,255,0.1)";
        ctx.fillRect(10, y, w - 20, cardH);
        ctx.strokeStyle = quest.state === "ReadyToTurnIn" ? "#0f0" : "#fff";
        ctx.strokeRect(10, y, w - 20, cardH);

        ctx.fillStyle = "#ff0"; ctx.font = "bold 16px sans-serif";
        ctx.fillText(quest.name, 25, y + 10);

        ctx.fillStyle = "#aaa"; ctx.font = "12px sans-serif"; ctx.textAlign = "right";
        ctx.fillText(quest.state, w - 25, y + 12);
        ctx.textAlign = "left";

        ctx.fillStyle = "white"; ctx.font = "14px sans-serif";
        descLines.forEach((line, i) => {
            ctx.fillText(line, 25, y + 35 + i * 20);
        });

        // Pin Button
        const isPinned = ui.pinnedQuestId === quest.id;
        ui.drawBtn(ctx, w - 80, y + 35, 60, 25, isPinned ? "Unpin" : "Pin", isPinned);

        // Draw objectives
        const objY = y + 40 + descLines.length * 20;
        quest.objectives.forEach((obj, i) => {
            let text = "";
            let progress = 0;
            if ("Gather" in obj) {
                text = `Gather ${obj.Gather.item}: ${obj.Gather.current}/${obj.Gather.count}`;
                progress = obj.Gather.current / obj.Gather.count;
            } else if ("Kill" in obj) {
                text = `Kill ${obj.Kill.mob_type}: ${obj.Kill.current}/${obj.Kill.count}`;
                progress = obj.Kill.current / obj.Kill.count;
            } else if ("TalkTo" in obj) {
                text = `Talk to ${obj.TalkTo.npc_name}`;
                progress = quest.state === "ReadyToTurnIn" || quest.state === "Completed" ? 1 : 0;
            }
            
            ctx.fillStyle = progress >= 1 ? "#4f4" : "#fff";
            ctx.fillText("- " + text, 35, objY + i * 20);
        });

        y += spacing;
    });
    ctx.textBaseline = "alphabetic"; // Reset to default
}

export function handleQuestsInput(mx: number, my: number, w: number, h: number, p: Player, ui: UIManager, scrollY: number): { pin?: string } | null {
    let y = 50 - scrollY;
    for (const quest of p.quests) {
        const descLines = ui.wrapText(ui.ctx, quest.description, w - 100);
        const cardH = 100 + descLines.length * 20;
        const spacing = cardH + 10;
        
        // Pin button hit check
        if (mx >= w - 80 && mx <= w - 20 && my >= y + 35 && my <= y + 60) {
            return { pin: quest.id };
        }
        y += spacing;
    }
    return null;
}
