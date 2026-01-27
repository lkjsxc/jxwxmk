use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct BalanceConfig {
    pub player: PlayerBalance,
    pub combat: CombatBalance,
    pub tools: ToolBalance,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerBalance {
    pub max_health: f32,
    pub base_speed: f32,
    pub inventory_slots: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct CombatBalance {
    pub base_melee_damage: f32,
    pub base_ranged_damage: f32,
    pub pvp_enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ToolBalance {
    pub base_gather_damage: f32,
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

impl Default for PlayerBalance {
    fn default() -> Self {
        Self {
            max_health: 100.0,
            base_speed: 4.0,
            inventory_slots: 30,
        }
    }
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

impl Default for ToolBalance {
    fn default() -> Self {
        Self {
            base_gather_damage: 10.0,
        }
    }
}
