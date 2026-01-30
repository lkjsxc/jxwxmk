use config::SettlementsConfig;
use world::{World, PlayerState};

pub struct BarrierSystem;

impl BarrierSystem {
    pub fn is_in_safe_zone(
        player: &PlayerState,
        world: &World,
        config: &SettlementsConfig,
    ) -> bool {
        if !player.spawned {
            return false;
        }

        for settlement in &world.settlements {
            if settlement.is_in_safe_zone(
                player.x,
                player.y,
                config.barrier.base_range_wu,
                config.barrier.level_multiplier_wu,
            ) {
                return true;
            }
        }

        false
    }

    pub fn can_pvp(attacker: &PlayerState, target: &PlayerState, world: &World, config: &SettlementsConfig) -> bool {
        // PvP is disabled if either player is in a safe zone
        if Self::is_in_safe_zone(attacker, world, config) {
            return false;
        }
        if Self::is_in_safe_zone(target, world, config) {
            return false;
        }
        true
    }
}
