use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Server Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub tick_rate: u64,
    pub db_url: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            tick_rate: 30,
            db_url: "postgres://jxwxmk:jxwxmk@localhost:5432/jxwxmk".to_string(),
        }
    }
}

// World Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldConfig {
    pub seed: u64,
    pub chunk_size: i32,
    pub view_distance: i32,
}

// Balance Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BalanceConfig {
    pub player: PlayerBalance,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerBalance {
    pub max_hp: f32,
    pub speed: f32,
}

// Survival Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SurvivalConfig {
    pub hunger_rate: f32,
    pub temp_rate: f32,
}

// Crafting Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CraftingConfig {
    pub recipes: Vec<Recipe>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub id: String,
    pub inputs: HashMap<String, u32>,
    pub outputs: HashMap<String, u32>,
}

// Spawning Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpawningConfig {
    pub budgets: HashMap<String, u32>,
}

// Biomes Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BiomesConfig {
    pub biomes: HashMap<String, BiomeDef>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BiomeDef {
    pub name: String,
}

// Settlements Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettlementsConfig {
    pub tiers: Vec<SettlementTier>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettlementTier {
    pub level: u32,
    pub name: String,
}

// Economy Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EconomyConfig {
    pub tax_rate: f32,
}

// Quests Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestsConfig {
    pub quests: Vec<QuestDef>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestDef {
    pub id: String,
    pub name: String,
}

// Achievements Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AchievementsConfig {
    pub achievements: Vec<AchievementDef>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AchievementDef {
    pub id: String,
    pub name: String,
}

// Root Config
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub world: Option<WorldConfig>,
    pub balance: Option<BalanceConfig>,
    pub survival: Option<SurvivalConfig>,
    pub crafting: Option<CraftingConfig>,
    pub spawning: Option<SpawningConfig>,
    pub biomes: Option<BiomesConfig>,
    pub settlements: Option<SettlementsConfig>,
    pub economy: Option<EconomyConfig>,
    pub quests: Option<QuestsConfig>,
    pub achievements: Option<AchievementsConfig>,
}

impl Config {
    pub fn load_from_dir<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        
        let mut config = Config::default();

        macro_rules! load {
            ($field:ident, $file:expr, $type:ty) => {
                let p = path.join($file);
                if p.exists() {
                    let content = std::fs::read_to_string(p)?;
                    config.$field = Some(serde_json::from_str::<$type>(&content)?);
                }
            };
        }

        let server_path = path.join("server.json");
        if server_path.exists() {
            let content = std::fs::read_to_string(server_path)?;
            config.server = serde_json::from_str(&content)?;
        }

        load!(world, "world.json", WorldConfig);
        load!(balance, "balance.json", BalanceConfig);
        load!(survival, "survival.json", SurvivalConfig);
        load!(crafting, "crafting.json", CraftingConfig);
        load!(spawning, "spawning.json", SpawningConfig);
        load!(biomes, "biomes.json", BiomesConfig);
        load!(settlements, "settlements.json", SettlementsConfig);
        load!(economy, "economy.json", EconomyConfig);
        load!(quests, "quests.json", QuestsConfig);
        load!(achievements, "achievements.json", AchievementsConfig);

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_load_config() {
        let dir = tempfile::tempdir().unwrap();
        let server_path = dir.path().join("server.json");
        let mut file = std::fs::File::create(server_path).unwrap();
        writeln!(file, r#"{{"host": "127.0.0.1", "port": 9090, "tick_rate": 60, "db_url": "test"}}"#).unwrap();

        let config = Config::load_from_dir(dir.path()).unwrap();
        assert_eq!(config.server.port, 9090);
    }
}
