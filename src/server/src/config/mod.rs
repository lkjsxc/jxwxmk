mod achievements;
mod balance;
mod biomes;
mod crafting;
mod economy;
mod loader;
mod quests;
mod server;
mod session;
mod settlements;
mod spawning;
mod survival;
mod world;

pub use achievements::*;
pub use balance::*;
pub use biomes::*;
pub use crafting::*;
pub use economy::*;
pub use loader::load_config;
pub use quests::*;
pub use server::*;
pub use session::*;
pub use settlements::*;
pub use spawning::*;
pub use survival::*;
pub use world::*;

#[derive(Debug, Clone)]
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
    pub fn new(
        server: ServerConfig,
        world: WorldConfig,
        balance: BalanceConfig,
        survival: SurvivalConfig,
        crafting: CraftingConfig,
        spawning: SpawningConfig,
        biomes: BiomesConfig,
        settlements: SettlementsConfig,
        economy: EconomyConfig,
        quests: QuestsConfig,
        achievements: AchievementsConfig,
    ) -> Self {
        Self {
            server,
            world,
            balance,
            survival,
            crafting,
            spawning,
            biomes,
            settlements,
            economy,
            quests,
            achievements,
        }
    }
}
