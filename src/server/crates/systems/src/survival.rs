use config::{SurvivalConfig, BalanceConfig};
use world::PlayerState;

pub struct SurvivalSystem;

impl SurvivalSystem {
    pub fn tick(players: &mut [&mut PlayerState], survival: &SurvivalConfig, balance: &BalanceConfig, dt: f64) {
        for player in players.iter_mut().filter(|p| p.spawned).map(|p| &mut **p) {
            Self::update_player(player, survival, balance, dt);
        }
    }

    fn update_player(player: &mut PlayerState, survival: &SurvivalConfig, balance: &BalanceConfig, dt: f64) {
        // player is already &mut PlayerState
        // Hunger decay
        let hunger_decay = survival.hunger_decay / survival.tick_rate() * dt;
        player.vitals.hunger = (player.vitals.hunger - hunger_decay).max(0.0);

        // Starvation damage
        if player.vitals.hunger <= 0.0 {
            let starve = survival.starve_damage / survival.tick_rate() * dt;
            player.take_damage(starve);
        }

        // Healing (when hunger is above threshold)
        if player.vitals.hunger >= survival.heal_threshold && player.vitals.hp < balance.player.max_health {
            let heal = survival.heal_rate / survival.tick_rate() * dt;
            player.heal(heal);
        }

        // Temperature convergence toward neutral
        let target_temp = survival.neutral_temp;
        let current_temp = player.vitals.temperature;
        let converge = survival.temperature_converge_rate * dt;
        player.vitals.temperature = lerp(current_temp, target_temp, converge);

        // Freeze damage
        if player.vitals.temperature <= 0.0 {
            let freeze = survival.freeze_damage / survival.tick_rate() * dt;
            player.take_damage(freeze);
        }

        // Clamp vitals
        player.vitals.hp = player.vitals.hp.clamp(0.0, balance.player.max_health);
        player.vitals.hunger = player.vitals.hunger.clamp(0.0, 100.0);
        player.vitals.temperature = player.vitals.temperature.clamp(0.0, 100.0);
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

// Helper trait to get tick rate from survival config
trait SurvivalConfigExt {
    fn tick_rate(&self) -> f64;
}

impl SurvivalConfigExt for SurvivalConfig {
    fn tick_rate(&self) -> f64 {
        30.0 // Default tick rate
    }
}
