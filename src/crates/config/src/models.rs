use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- Server Config ---
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ServerConfig {
    pub version: u32,
    pub bind_http: String,
    pub protocol_version: u32,
    pub tick_rate: u32,
    pub limits: ServerLimits,
    pub rate_limits: ServerRateLimits,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ServerLimits {
    pub ws_max_message_bytes: usize,
    pub ws_messages_per_sec: u32,
    pub ws_burst: u32,
    pub max_name_len: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ServerRateLimits {
    pub session_claim_per_ip_per_minute: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            version: 1,
            bind_http: "0.0.0.0:8080".to_string(),
            protocol_version: 3,
            tick_rate: 30,
            limits: ServerLimits {
                ws_max_message_bytes: 16384,
                ws_messages_per_sec: 30,
                ws_burst: 60,
                max_name_len: 24,
            },
            rate_limits: ServerRateLimits {
                session_claim_per_ip_per_minute: 10,
            },
        }
    }
}

// --- World Config ---
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorldConfig {
    pub version: u32,
    #[serde(default = "default_seed")]
    pub seed: String,
    pub chunk_size_wu: u32,
    pub view_radius_chunks: u32,
    pub sim_radius_chunks: u32,
    pub max_active_chunks: u32,
}

fn default_seed() -> String {
    "12345".to_string()
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            version: 1,
            seed: default_seed(),
            chunk_size_wu: 128,
            view_radius_chunks: 3,
            sim_radius_chunks: 2,
            max_active_chunks: 512,
        }
    }
}

// --- Balance Config ---
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BalanceConfig {
    pub version: u32,
    pub player: PlayerBalance,
    pub resources: ResourceBalance,
    pub tools: ToolBalance,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerBalance {
    pub max_health: f32,
    pub base_speed: f32,
    pub interaction_range_wu: f32,
    pub hotbar_slots: u32,
    pub inventory_slots: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceBalance {
    pub tree_amount: f32,
    pub rock_amount: f32,
    pub food_amount: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolBalance {
    pub rock_mult: f32,
}

impl Default for BalanceConfig {
    fn default() -> Self {
        Self {
            version: 1,
            player: PlayerBalance {
                max_health: 100.0,
                base_speed: 6.0,
                interaction_range_wu: 4.0,
                hotbar_slots: 7,
                inventory_slots: 28,
            },
            resources: ResourceBalance {
                tree_amount: 30.0,
                rock_amount: 30.0,
                food_amount: 10.0,
            },
            tools: ToolBalance {
                rock_mult: 1.5,
            },
        }
    }
}

// --- Survival Config ---
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SurvivalConfig {
    pub version: u32,
    pub hunger_decay: f32,
    pub starve_damage: f32,
    pub heal_threshold: f32,
    pub heal_rate: f32,
    pub neutral_temp: f32,
    pub temperature_converge_rate: f32,
    pub freeze_damage: f32,
    pub thirst_enabled: bool,
    pub thirst_decay: f32,
    pub dehydrate_damage: f32,
}

impl Default for SurvivalConfig {
    fn default() -> Self {
        Self {
            version: 1,
            hunger_decay: 2.0,
            starve_damage: 5.0,
            heal_threshold: 60.0,
            heal_rate: 2.0,
            neutral_temp: 50.0,
            temperature_converge_rate: 0.5,
            freeze_damage: 5.0,
            thirst_enabled: false,
            thirst_decay: 2.0,
            dehydrate_damage: 5.0,
        }
    }
}

// --- Crafting Config ---
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CraftingConfig {
    pub version: u32,
    pub recipes: Vec<Recipe>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    pub id: String,
    pub station: String,
    pub inputs: Vec<RecipeInput>,
    pub output: RecipeOutput,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipeInput {
    pub item: String,
    pub count: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecipeOutput {
    pub item: String,
    pub count: u32,
}

impl Default for CraftingConfig {
    fn default() -> Self {
        Self {
            version: 1,
            recipes: vec![],
        }
    }
}

// --- Spawning Config ---
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpawningConfig {
    pub version: u32,
    pub resource_respawn_seconds: HashMap<String, u32>,
    pub mob_respawn_seconds: HashMap<String, u32>,
    pub chunk_budgets: HashMap<String, ChunkBudget>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChunkBudget {
    pub resources: HashMap<String, u32>,
    pub mobs: HashMap<String, u32>,
}

impl Default for SpawningConfig {
    fn default() -> Self {
        Self {
            version: 1,
            resource_respawn_seconds: HashMap::new(),
            mob_respawn_seconds: HashMap::new(),
            chunk_budgets: HashMap::new(),
        }
    }
}

// --- Biomes Config ---
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiomesConfig {
    pub version: u32,
    pub biomes: Vec<Biome>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Biome {
    pub id: String,
    pub temperature_modifier: f32,
    pub hunger_modifier: f32,
}

impl Default for BiomesConfig {
    fn default() -> Self {
        Self {
            version: 1,
            biomes: vec![],
        }
    }
}

// --- Settlements Config ---
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SettlementsConfig {
    pub version: u32,
    pub barrier: BarrierConfig,
    pub tiers: Vec<SettlementTier>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BarrierConfig {
    pub base_range_wu: f32,
    pub level_multiplier_wu: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SettlementTier {
    pub min_core_level: u32,
    pub name: String,
}

impl Default for SettlementsConfig {
    fn default() -> Self {
        Self {
            version: 1,
            barrier: BarrierConfig {
                base_range_wu: 24.0,
                level_multiplier_wu: 6.0,
            },
            tiers: vec![
                SettlementTier { min_core_level: 1, name: "outpost".to_string() },
                SettlementTier { min_core_level: 2, name: "village".to_string() },
                SettlementTier { min_core_level: 4, name: "town".to_string() },
                SettlementTier { min_core_level: 7, name: "city".to_string() },
            ],
        }
    }
}

// --- Economy Config ---
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EconomyConfig {
    pub version: u32,
    pub tax_rate: f32,
    pub vendor_prices: HashMap<String, VendorPrice>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VendorPrice {
    pub buy: u32,
    pub sell: u32,
}

impl Default for EconomyConfig {
    fn default() -> Self {
        Self {
            version: 1,
            tax_rate: 0.05,
            vendor_prices: HashMap::new(),
        }
    }
}

// --- Quests Config ---
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestsConfig {
    pub version: u32,
    pub templates: Vec<QuestTemplate>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestTemplate {
    pub id: String,
    pub name: String,
    pub objectives: Vec<QuestObjective>,
    pub rewards: QuestRewards,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum QuestObjective {
    #[serde(rename = "kill")]
    Kill {
        mob_type: String,
        count: u32,
    },
    // Add other types as needed
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuestRewards {
    pub xp: u32,
}

impl Default for QuestsConfig {
    fn default() -> Self {
        Self {
            version: 1,
            templates: vec![],
        }
    }
}

// --- Achievements Config ---
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AchievementsConfig {
    pub version: u32,
    pub achievements: Vec<Achievement>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub requirement: AchievementRequirement,
    pub rewards: AchievementRewards,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum AchievementRequirement {
    #[serde(rename = "steps")]
    Steps { count: u32 },
    // Add other types as needed
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AchievementRewards {
    pub xp: u32,
    #[serde(default)]
    pub stat_bonuses: HashMap<String, f32>,
}

impl Default for AchievementsConfig {
    fn default() -> Self {
        Self {
            version: 1,
            achievements: vec![],
        }
    }
}

// --- Main Container ---
#[derive(Debug, Clone, Default)]
pub struct Config {
    pub server: ServerConfig,
    pub world: WorldConfig,
    pub balance: BalanceConfig,
    pub survival: SurvivalConfig,
    pub crafting: CraftingConfig,
    pub spawning: SpawningConfig,
    pub biomes: BiomesConfig,
    pub settlements: SettlementsConfig,
    pub economy: EconomyConfig,
    pub quests: QuestsConfig,
    pub achievements: AchievementsConfig,
}