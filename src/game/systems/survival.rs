use crate::game::entities::player::Player;

pub struct SurvivalSystem;

impl SurvivalSystem {
    pub fn tick(player: &mut Player) {
        // Constants (derived from docs)
        const HUNGER_DECAY: f64 = 0.1 / 20.0; // 0.1 per sec (was 0.5)
        const HEAL_RATE: f64 = 1.0 / 20.0;
        const STARVE_DMG: f64 = 5.0 / 20.0;
        const FREEZE_DMG: f64 = 2.0 / 20.0;
        const AMBIENT_TEMP: f64 = 20.0; // Assume global for now

        // Hunger
        player.hunger -= HUNGER_DECAY;
        if player.hunger < 0.0 {
            player.hunger = 0.0;
            player.health -= STARVE_DMG;
        } else if player.hunger > 90.0 && player.health < 100.0 {
            player.health += HEAL_RATE;
        }

        // Temperature (Simplified linear approach)
        // Move internal temp towards ambient
        let diff = AMBIENT_TEMP - player.cold; // player.cold is actually 'Temp' in docs (0-100)
        // Wait, docs say "0 is Freezing, 100 is Overheat".
        // Let's assume ambient is effectively mapped to this 0-100 scale.
        // Let's say 50 is neutral (20C). 0 is -20C. 100 is 60C.
        // Ambient 50.
        let target_temp = 50.0; 
        
        let change_rate = 1.0 / 20.0;
        if player.cold < target_temp {
            player.cold += change_rate;
        } else {
            player.cold -= change_rate;
        }

        // Freeze Check
        if player.cold <= 0.0 {
            player.health -= FREEZE_DMG;
        }

        // Cap Health
        player.health = player.health.clamp(0.0, 100.0);
    }
}
