use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceConfig {
    pub player: PlayerBalance,
    pub combat: CombatBalance,
    pub tools: ToolBalance,
}

impl Default for BalanceConfig {
    fn default() -> Self {
        Self {
            player: PlayerBalance::default(),
            combat: CombatBalance::default(),
            tools: ToolBalance::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBalance {
    pub max_health: f32,
    pub base_speed: f32,
    pub inventory_slots: usize,
}

impl Default for PlayerBalance {
    fn default() -> Self {
        Self {
            max_health: 100.0,
            base_speed: 4.0,
            inventory_slots: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatBalance {
    pub base_melee_damage: f32,
    pub base_ranged_damage: f32,
    pub pvp_enabled: bool,
}

impl Default for CombatBalance {
    fn default() -> Self {
        Self {
            base_melee_damage: 8.0,
            base_ranged_damage: 6.0,
            pvp_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolBalance {
    pub base_gather_damage: f32,
}

impl Default for ToolBalance {
    fn default() -> Self {
        Self { base_gather_damage: 10.0 }
    }
}
