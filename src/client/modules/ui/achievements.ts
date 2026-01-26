import { Achievement, Player } from "../../types";
import { UIManager } from "./index";

export function drawAchievements(ctx: CanvasRenderingContext2D, player: Player, allAchievements: Achievement[], selectedAchId: string | null, w: number, h: number, scrollY: number, ui: UIManager) {
    const listW = w / 2;
    // List
    ctx.fillStyle = "white"; ctx.textAlign = "center"; ctx.font = "20px sans-serif";
    ctx.fillText("Achievements", listW / 2, 30);
    
    let y = 60 - scrollY;
    const unlockedSet = new Set(player.achievements);
    
    for (const ach of allAchievements) {
        const isUnlocked = unlockedSet.has(ach.id);
        const isSelected = selectedAchId === ach.id;
        
        if (isSelected) { ctx.fillStyle = "#444"; ctx.fillRect(10, y - 20, listW - 20, 35); }
        
        ctx.fillStyle = isUnlocked ? "#fb4" : "#888"; 
        ctx.textAlign = "left"; ctx.font = "16px sans-serif";
        ctx.fillText(`${isUnlocked ? "★" : "🔒"} ${ach.name}`, 20, y);

        // Progress mini-bar
        if (!isUnlocked && ach.requirement) {
            const current = getProgressValue(player, ach.requirement.type);
            const pct = Math.min(1, current / ach.requirement.value);
            ctx.fillStyle = "#222"; ctx.fillRect(listW - 60, y - 12, 50, 6);
            ctx.fillStyle = "#888"; ctx.fillRect(listW - 60, y - 12, 50 * pct, 6);
        }

        y += 40;
    }

    // Details (Right side, fixed)
    if (selectedAchId) {
        const ach = allAchievements.find(a => a.id === selectedAchId);
        if (ach) {
            const dx = listW + 20; const dy = 60;
            ctx.fillStyle = "white"; ctx.textAlign = "left"; ctx.font = "bold 20px sans-serif";
            ctx.fillText(ach.name, dx, dy);
            ctx.font = "14px sans-serif"; ctx.fillStyle = "#ccc";
            ctx.fillText(ach.description, dx, dy + 30);
            
            if (ach.requirement) {
                const current = getProgressValue(player, ach.requirement.type);
                ctx.fillStyle = "#aaa";
                ctx.fillText(`Progress: ${current.toLocaleString()} / ${ach.requirement.value.toLocaleString()}`, dx, dy + 55);
                
                // Progress Bar
                const bw = w - dx - 30;
                ctx.fillStyle = "#222"; ctx.fillRect(dx, dy + 65, bw, 10);
                ctx.fillStyle = "#fb4"; ctx.fillRect(dx, dy + 65, bw * Math.min(1, current / ach.requirement.value), 10);
            }

            ctx.fillStyle = "#8f8";
            ctx.fillText(`Reward: +${(ach.stat_bonus[1]*100).toFixed(0)}% ${ach.stat_bonus[0]}`, dx, dy + 95);
            
            const isUnlocked = unlockedSet.has(ach.id);
            ctx.fillStyle = isUnlocked ? "#4f4" : "#f44";
            ctx.fillText(isUnlocked ? "UNLOCKED" : "LOCKED", dx, dy + 120);

            // Pin button
            const isPinned = ui.pinnedAchId === ach.id;
            ui.drawBtn(ctx, dx, dy + 140, 80, 30, isPinned ? "Unpin" : "Pin", isPinned);
        }
    }
}

export function handleAchInput(mx: number, my: number, w: number, h: number, allAchievements: Achievement[], selectedAchId: string | null, scrollY: number): { select?: string, pin?: string } | null {
    const listW = w / 2;
    let y = 60 - scrollY;
    for (const ach of allAchievements) {
        if (mx >= 10 && mx <= listW - 10 && my >= y - 20 && my <= y + 20) {
            return { select: ach.id };
        }
        y += 40;
    }

    if (selectedAchId) {
        const dx = listW + 20; const dy = 60;
        if (mx >= dx && mx <= dx + 80 && my >= dy + 140 && my <= dy + 170) {
            return { pin: selectedAchId };
        }
    }
    return null;
}
