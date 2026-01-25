import { Achievement, Player } from "../../types";

export function drawAchievements(ctx: CanvasRenderingContext2D, player: Player, allAchievements: Achievement[], selectedAchId: string | null, w: number, h: number, scrollY: number) {
    const listW = w / 2;
    // List
    ctx.fillStyle = "white"; ctx.textAlign = "center"; ctx.font = "20px sans-serif";
    ctx.fillText("Achievements", listW / 2, 30);
    
    let y = 60 - scrollY;
    const unlockedSet = new Set(player.achievements);
    
    for (const ach of allAchievements) {
        const isUnlocked = unlockedSet.has(ach.id);
        const isSelected = selectedAchId === ach.id;
        
        if (isSelected) { ctx.fillStyle = "#444"; ctx.fillRect(10, y - 20, listW - 20, 30); }
        
        ctx.fillStyle = isUnlocked ? "#fb4" : "#888"; 
        ctx.textAlign = "left"; ctx.font = "16px sans-serif";
        ctx.fillText(`${isUnlocked ? "★" : "🔒"} ${ach.name}`, 20, y);
        y += 35;
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
            
            ctx.fillStyle = "#8f8";
            ctx.fillText(`Reward: +${(ach.stat_bonus[1]*100).toFixed(0)}% ${ach.stat_bonus[0]}`, dx, dy + 60);
            
            const isUnlocked = unlockedSet.has(ach.id);
            ctx.fillStyle = isUnlocked ? "#4f4" : "#f44";
            ctx.fillText(isUnlocked ? "UNLOCKED" : "LOCKED", dx, dy + 90);
        }
    }
}

export function handleAchInput(mx: number, my: number, w: number, h: number, allAchievements: Achievement[], scrollY: number): string | null {
    const listW = w / 2;
    let y = 60 - scrollY;
    for (const ach of allAchievements) {
        if (mx >= 10 && mx <= listW - 10 && my >= y - 20 && my <= y + 10) {
            return ach.id;
        }
        y += 35;
    }
    return null;
}
