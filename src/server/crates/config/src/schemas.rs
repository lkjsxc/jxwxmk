use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub version: i32,
    #[serde(default = "default_bind")]
    pub bind_http: String,
    #[serde(default = "default_protocol_version")]
    pub protocol_version: u32,
    #[serde(default = "default_tick_rate")]
    pub tick_rate: u32,
    #[serde(default)]
    pub limits: ServerLimits,
    #[serde(default)]
    pub rate_limits: RateLimits,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            version: 1,
            bind_http: default_bind(),
            protocol_version: default_protocol_version(),
            tick_rate: default_tick_rate(),
            limits: ServerLimits::default(),
            rate_limits: RateLimits::default(),
        }
    }
}

fn default_bind() -> String {
    "0.0.0.0:8080".to_string()
}

fn default_protocol_version() -> u32 {
    3
}

fn default_tick_rate() -> u32 {
    30
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerLimits {
    #[serde(default = "default_ws_max_message")]
    pub ws_max_message_bytes: usize,
    #[serde(default = "default_ws_msgs_per_sec")]
    pub ws_messages_per_sec: u32,
    #[serde(default = "default_ws_burst")]
    pub ws_burst: u32,
    #[serde(default = "default_ws_idle_timeout_secs")]
    pub ws_idle_timeout_secs: u64,
    #[serde(default = "default_ws_heartbeat_interval_secs")]
    pub ws_heartbeat_interval_secs: u64,
    #[serde(default = "default_max_name_len")]
    pub max_name_len: usize,
}

impl Default for ServerLimits {
    fn default() -> Self {
        Self {
            ws_max_message_bytes: default_ws_max_message(),
            ws_messages_per_sec: default_ws_msgs_per_sec(),
            ws_burst: default_ws_burst(),
            ws_idle_timeout_secs: default_ws_idle_timeout_secs(),
            ws_heartbeat_interval_secs: default_ws_heartbeat_interval_secs(),
            max_name_len: default_max_name_len(),
        }
    }
}

fn default_ws_max_message() -> usize {
    16384
}

fn default_ws_msgs_per_sec() -> u32 {
    30
}

fn default_ws_burst() -> u32 {
    60
}

fn default_ws_idle_timeout_secs() -> u64 {
    10
}

fn default_ws_heartbeat_interval_secs() -> u64 {
    5
}

fn default_max_name_len() -> usize {
    24
}

#[derive(Debug, Clone, Deserialize)]
pub struct RateLimits {
    #[serde(default = "default_session_claim")]
    pub session_claim_per_ip_per_minute: u32,
}

impl Default for RateLimits {
    fn default() -> Self {
        Self {
            session_claim_per_ip_per_minute: default_session_claim(),
        }
    }
}

fn default_session_claim() -> u32 {
    10
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorldConfig {
    pub version: i32,
    #[serde(default = "default_seed")]
    pub seed: String,
    #[serde(default = "default_chunk_size")]
    pub chunk_size_wu: i32,
    #[serde(default = "default_view_radius")]
    pub view_radius_chunks: i32,
    #[serde(default = "default_sim_radius")]
    pub sim_radius_chunks: i32,
    #[serde(default = "default_max_active")]
    pub max_active_chunks: usize,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            version: 1,
            seed: default_seed(),
            chunk_size_wu: default_chunk_size(),
            view_radius_chunks: default_view_radius(),
            sim_radius_chunks: default_sim_radius(),
            max_active_chunks: default_max_active(),
        }
    }
}

fn default_seed() -> String {
    "123456789".to_string()
}

fn default_chunk_size() -> i32 {
    128
}

fn default_view_radius() -> i32 {
    3
}

fn default_sim_radius() -> i32 {
    2
}

fn default_max_active() -> usize {
    512
}

#[derive(Debug, Clone, Deserialize)]
pub struct BalanceConfig {
    pub version: i32,
    #[serde(default)]
    pub player: PlayerBalance,
    #[serde(default)]
    pub resources: ResourceBalance,
    #[serde(default)]
    pub tools: ToolBalance,
}

impl Default for BalanceConfig {
    fn default() -> Self {
        Self {
            version: 1,
            player: PlayerBalance::default(),
            resources: ResourceBalance::default(),
            tools: ToolBalance::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerBalance {
    #[serde(default = "default_max_health")]
    pub max_health: f64,
    #[serde(default = "default_base_speed")]
    pub base_speed: f64,
    #[serde(default = "default_interaction_range")]
    pub interaction_range_wu: f64,
    #[serde(default = "default_hotbar")]
    pub hotbar_slots: usize,
    #[serde(default = "default_inventory")]
    pub inventory_slots: usize,
}

impl Default for PlayerBalance {
    fn default() -> Self {
        Self {
            max_health: default_max_health(),
            base_speed: default_base_speed(),
            interaction_range_wu: default_interaction_range(),
            hotbar_slots: default_hotbar(),
            inventory_slots: default_inventory(),
        }
    }
}

fn default_max_health() -> f64 {
    100.0
}

fn default_base_speed() -> f64 {
    6.0
}

fn default_interaction_range() -> f64 {
    4.0
}

fn default_hotbar() -> usize {
    7
}

fn default_inventory() -> usize {
    30
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResourceBalance {
    #[serde(default = "default_tree_amount")]
    pub tree_amount: f64,
    #[serde(default = "default_rock_amount")]
    pub rock_amount: f64,
    #[serde(default = "default_food_amount")]
    pub food_amount: f64,
}

impl Default for ResourceBalance {
    fn default() -> Self {
        Self {
            tree_amount: default_tree_amount(),
            rock_amount: default_rock_amount(),
            food_amount: default_food_amount(),
        }
    }
}

fn default_tree_amount() -> f64 {
    30.0
}

fn default_rock_amount() -> f64 {
    30.0
}

fn default_food_amount() -> f64 {
    10.0
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolBalance {
    #[serde(default = "default_rock_mult")]
    pub rock_mult: f64,
}

impl Default for ToolBalance {
    fn default() -> Self {
        Self {
            rock_mult: default_rock_mult(),
        }
    }
}

fn default_rock_mult() -> f64 {
    1.5
}

#[derive(Debug, Clone, Deserialize)]
pub struct SurvivalConfig {
    pub version: i32,
    #[serde(default = "default_hunger_decay")]
    pub hunger_decay: f64,
    #[serde(default = "default_starve_damage")]
    pub starve_damage: f64,
    #[serde(default = "default_heal_threshold")]
    pub heal_threshold: f64,
    #[serde(default = "default_heal_rate")]
    pub heal_rate: f64,
    #[serde(default = "default_neutral_temp")]
    pub neutral_temp: f64,
    #[serde(default = "default_temp_converge")]
    pub temperature_converge_rate: f64,
    #[serde(default = "default_freeze_damage")]
    pub freeze_damage: f64,
    #[serde(default)]
    pub thirst_enabled: bool,
    #[serde(default = "default_thirst_decay")]
    pub thirst_decay: f64,
    #[serde(default = "default_dehydrate")]
    pub dehydrate_damage: f64,
}

impl Default for SurvivalConfig {
    fn default() -> Self {
        Self {
            version: 1,
            hunger_decay: default_hunger_decay(),
            starve_damage: default_starve_damage(),
            heal_threshold: default_heal_threshold(),
            heal_rate: default_heal_rate(),
            neutral_temp: default_neutral_temp(),
            temperature_converge_rate: default_temp_converge(),
            freeze_damage: default_freeze_damage(),
            thirst_enabled: false,
            thirst_decay: default_thirst_decay(),
            dehydrate_damage: default_dehydrate(),
        }
    }
}

fn default_hunger_decay() -> f64 {
    2.0
}

fn default_starve_damage() -> f64 {
    5.0
}

fn default_heal_threshold() -> f64 {
    60.0
}

fn default_heal_rate() -> f64 {
    2.0
}

fn default_neutral_temp() -> f64 {
    50.0
}

fn default_temp_converge() -> f64 {
    0.5
}

fn default_freeze_damage() -> f64 {
    5.0
}

fn default_thirst_decay() -> f64 {
    2.0
}

fn default_dehydrate() -> f64 {
    5.0
}

#[derive(Debug, Clone, Deserialize)]
pub struct CraftingConfig {
    pub version: i32,
    #[serde(default)]
    pub recipes: Vec<Recipe>,
}

impl Default for CraftingConfig {
    fn default() -> Self {
        Self {
            version: 1,
            recipes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub station: String,
    pub inputs: Vec<RecipeInput>,
    pub output: RecipeOutput,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecipeInput {
    pub item: String,
    pub count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecipeOutput {
    pub item: String,
    pub count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpawningConfig {
    pub version: i32,
    #[serde(default)]
    pub resource_respawn_seconds: HashMap<String, i32>,
    #[serde(default)]
    pub mob_respawn_seconds: HashMap<String, i32>,
    #[serde(default)]
    pub chunk_budgets: HashMap<String, ChunkBudget>,
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

#[derive(Debug, Clone, Deserialize)]
pub struct ChunkBudget {
    #[serde(default)]
    pub resources: HashMap<String, i32>,
    #[serde(default)]
    pub mobs: HashMap<String, i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BiomesConfig {
    pub version: i32,
    #[serde(default)]
    pub biomes: Vec<Biome>,
}

impl Default for BiomesConfig {
    fn default() -> Self {
        Self {
            version: 1,
            biomes: vec![Biome {
                id: "forest".to_string(),
                temperature_modifier: 0.0,
                hunger_modifier: 1.0,
            }],
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Biome {
    pub id: String,
    #[serde(default)]
    pub temperature_modifier: f64,
    #[serde(default = "one")]
    pub hunger_modifier: f64,
}

fn one() -> f64 {
    1.0
}

#[derive(Debug, Clone, Deserialize)]
pub struct SettlementsConfig {
    pub version: i32,
    #[serde(default)]
    pub barrier: BarrierConfig,
    #[serde(default)]
    pub tiers: Vec<SettlementTier>,
}

impl Default for SettlementsConfig {
    fn default() -> Self {
        Self {
            version: 1,
            barrier: BarrierConfig::default(),
            tiers: vec![
                SettlementTier { min_core_level: 1, name: "outpost".to_string() },
                SettlementTier { min_core_level: 2, name: "village".to_string() },
                SettlementTier { min_core_level: 4, name: "town".to_string() },
                SettlementTier { min_core_level: 7, name: "city".to_string() },
            ],
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BarrierConfig {
    #[serde(default = "default_base_range")]
    pub base_range_wu: f64,
    #[serde(default = "default_level_mult")]
    pub level_multiplier_wu: f64,
}

impl Default for BarrierConfig {
    fn default() -> Self {
        Self {
            base_range_wu: default_base_range(),
            level_multiplier_wu: default_level_mult(),
        }
    }
}

fn default_base_range() -> f64 {
    24.0
}

fn default_level_mult() -> f64 {
    6.0
}

#[derive(Debug, Clone, Deserialize)]
pub struct SettlementTier {
    pub min_core_level: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct EconomyConfig {
    pub version: i32,
    #[serde(default)]
    pub prices: HashMap<String, Price>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Price {
    pub buy: i32,
    pub sell: i32,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QuestsConfig {
    pub version: i32,
    #[serde(default)]
    pub quests: Vec<QuestTemplate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QuestTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub objectives: Vec<QuestObjectiveTemplate>,
    #[serde(default)]
    pub rewards: QuestRewards,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QuestObjectiveTemplate {
    pub r#type: String,
    pub target: String,
    pub count: i32,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QuestRewards {
    #[serde(default)]
    pub xp: i32,
    #[serde(default)]
    pub items: Vec<QuestRewardItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QuestRewardItem {
    pub item: String,
    pub count: i32,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AchievementsConfig {
    pub version: i32,
    #[serde(default)]
    pub achievements: Vec<AchievementTemplate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AchievementTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub requirement: AchievementRequirement,
    #[serde(default)]
    pub reward_xp: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AchievementRequirement {
    pub r#type: String,
    pub stat: String,
    pub value: i64,
}
