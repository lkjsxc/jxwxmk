use std::fs;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;

use super::{
    AchievementsConfig, BalanceConfig, BiomesConfig, Config, CraftingConfig, EconomyConfig,
    QuestsConfig, ServerConfig, SettlementsConfig, SpawningConfig, SurvivalConfig, WorldConfig,
};

pub fn load_config(dir: &Path) -> Config {
    let server = load_or_default::<ServerConfig>(&dir.join("server.json"));
    let world = load_or_default::<WorldConfig>(&dir.join("world.json"));
    let balance = load_or_default::<BalanceConfig>(&dir.join("balance.json"));
    let survival = load_or_default::<SurvivalConfig>(&dir.join("survival.json"));
    let crafting = load_or_default::<CraftingConfig>(&dir.join("crafting.json"));
    let spawning = load_or_default::<SpawningConfig>(&dir.join("spawning.json"));
    let biomes = load_or_default::<BiomesConfig>(&dir.join("biomes.json"));
    let settlements = load_or_default::<SettlementsConfig>(&dir.join("settlements.json"));
    let economy = load_or_default::<EconomyConfig>(&dir.join("economy.json"));
    let quests = load_or_default::<QuestsConfig>(&dir.join("quests.json"));
    let achievements = load_or_default::<AchievementsConfig>(&dir.join("achievements.json"));

    Config::new(
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
    )
}

fn load_or_default<T>(path: &PathBuf) -> T
where
    T: DeserializeOwned + Default,
{
    match fs::read_to_string(path) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(parsed) => parsed,
            Err(err) => {
                log::warn!("Failed to parse {:?}: {}. Using defaults.", path, err);
                T::default()
            }
        },
        Err(_) => T::default(),
    }
}
