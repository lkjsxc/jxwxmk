use std::path::Path;

pub mod defaults;
pub mod files;
mod loader;

use files::{
    AchievementsConfig, BalanceConfig, BiomesConfig, CraftingConfig, EconomyConfig,
    QuestsConfig, ServerConfig, SettlementsConfig, SpawningConfig, SurvivalConfig, WorldConfig,
};

#[derive(Clone, Debug)]
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

impl Config {
    pub fn load_from_dir(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        loader::load_from_dir(path.as_ref())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: Default::default(),
            world: Default::default(),
            balance: Default::default(),
            survival: Default::default(),
            crafting: Default::default(),
            spawning: Default::default(),
            biomes: Default::default(),
            settlements: Default::default(),
            economy: Default::default(),
            quests: Default::default(),
            achievements: Default::default(),
        }
    }
}
