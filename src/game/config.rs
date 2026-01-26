use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub tick_rate: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameConfig {
    pub world_width: f64,
    pub world_height: f64,
    pub interact_range: f64,
    pub spawn_radius: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MechanicsConfig {
    pub hunger_decay: f64,
    pub cold_decay: f64,
    pub heal_rate: f64,
    pub starve_dmg: f64,
    pub freeze_dmg: f64,
    pub food_value: f64,
    pub attack_cooldown: u64,
    pub interact_cooldown: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpawnConfig {
    pub resource_density: f64, // Resources per 10000 sq units
    pub mob_density: f64,      // Mobs per 10000 sq units
}

#[derive(Debug, Deserialize, Clone)]
pub struct LevelingConfig {
    pub mob_level_factor: f64,  // Level increase per unit distance from center
    pub tool_xp_per_use: f64,   // XP gained per tool use
}

#[derive(Debug, Deserialize, Clone)]
pub struct BarrierConfig {
    pub base_range: f64,
    pub level_multiplier: f64,
    pub placement_chance_center: f64,
    pub max_additional_barriers: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PlayerConfig {
    pub base_speed: f64,
    pub max_health: f64,
    pub max_hunger: f64,
    pub neutral_temp: f64,
    pub heal_threshold: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MobConfig {
    pub rabbit_health: f64,
    pub wolf_health: f64,
    pub bear_health: f64,
    pub wolf_dmg: f64,
    pub bear_dmg: f64,
    pub level_hp_mult: f64,
    pub level_dmg_mult: f64,
    pub aggression_range: f64,
    pub attack_range: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ToolConfig {
    pub base_dmg: f64,
    pub wood_pickaxe_dmg: f64,
    pub stone_pickaxe_dmg: f64,
    pub rock_mult: f64,
    pub tool_level_dmg_bonus: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResourceConfig {
    pub tree_amount: i32,
    pub rock_amount: i32,
    pub food_amount: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StructureConfig {
    pub wall_health: f64,
    pub door_health: f64,
    pub workbench_health: f64,
    pub torch_health: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BalanceConfig {
    pub player: PlayerConfig,
    pub mobs: MobConfig,
    pub tools: ToolConfig,
    pub resources: ResourceConfig,
    pub structures: StructureConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub game: GameConfig,
    pub mechanics: MechanicsConfig,
    pub spawning: SpawnConfig,
    pub leveling: LevelingConfig,
    pub barriers: BarrierConfig,
    pub balance: BalanceConfig,
}

impl AppConfig {
    pub fn load() -> Self {
        let content = fs::read_to_string("config.json").expect("config.json missing");
        serde_json::from_str(&content).expect("Failed to parse config")
    }
}