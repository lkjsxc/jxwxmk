use serde::Deserialize;
use std::fs;
use std::sync::OnceLock;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub game: GameConfig,
    pub mechanics: MechanicsConfig,
    pub spawning: SpawningConfig,
    pub leveling: LevelingConfig,
    pub barriers: BarriersConfig,
    pub balance: BalanceConfig,
}

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
    pub attack_cooldown: f64,
    pub interact_cooldown: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpawningConfig {
    pub resource_density: f64,
    pub mob_density: f64,
    pub unit_area: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LevelingConfig {
    pub mob_level_factor: f64,
    pub tool_xp_per_use: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BarriersConfig {
    pub base_range: f64,
    pub level_multiplier: f64,
    pub placement_chance_center: f64,
    pub max_additional_barriers: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BalanceConfig {
    pub player: PlayerBalance,
    pub mobs: MobsBalance,
    pub tools: ToolsBalance,
    pub resources: ResourcesBalance,
    pub structures: StructuresBalance,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PlayerBalance {
    pub base_speed: f64,
    pub max_health: f64,
    pub max_hunger: f64,
    pub neutral_temp: f64,
    pub heal_threshold: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MobsBalance {
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
pub struct ToolsBalance {
    pub base_dmg: f64,
    pub wood_pickaxe_dmg: f64,
    pub stone_pickaxe_dmg: f64,
    pub rock_mult: f64,
    pub tool_level_dmg_bonus: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResourcesBalance {
    pub tree_amount: u32,
    pub rock_amount: u32,
    pub food_amount: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StructuresBalance {
    pub wall_health: f64,
    pub door_health: f64,
    pub workbench_health: f64,
    pub torch_health: f64,
}

pub static CONFIG: OnceLock<AppConfig> = OnceLock::new();

impl AppConfig {
    pub fn load() {
        let path = Path::new("config.json");
        let content = fs::read_to_string(path).expect("Failed to read config.json");
        let config: AppConfig = serde_json::from_str(&content).expect("Failed to parse config.json");
        CONFIG.set(config).expect("Config already set");
    }

    pub fn get() -> &'static AppConfig {
        CONFIG.get().expect("Config not initialized")
    }
}
