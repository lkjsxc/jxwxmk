import { Player, Quest, ObjectiveType } from "../../types";
import { UIManager } from "./index";

export function drawQuests(ctx: CanvasRenderingContext2D, p: Player, w: number, h: number, ui: UIManager, scrollY: number) {
    ctx.fillStyle = "white"; ctx.font = "bold 18px sans-serif"; ctx.textAlign = "left";
    ctx.fillText("Quest Log", 20, 30);

    let y = 60 - scrollY;
    p.quests.forEach(quest => {
        if (y + 100 < 0 || y > h) { y += 120; return; }

        ctx.fillStyle = "rgba(255,255,255,0.1)";
        ctx.fillRect(10, y, w - 20, 100);
        ctx.strokeStyle = quest.state === "ReadyToTurnIn" ? "#0f0" : "#fff";
        ctx.strokeRect(10, y, w - 20, 100);

        ctx.fillStyle = "#ff0"; ctx.font = "bold 16px sans-serif";
        ctx.fillText(quest.name, 25, y + 25);

        ctx.fillStyle = "#aaa"; ctx.font = "12px sans-serif";
        ctx.fillText(quest.state, w - 100, y + 25);

        ctx.fillStyle = "white"; ctx.font = "14px sans-serif";
        ctx.fillText(quest.description, 25, y + 50);

        // Draw objectives
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
            ctx.fillText("- " + text, 35, y + 75 + i * 20);
        });

        y += 120;
    });
}
