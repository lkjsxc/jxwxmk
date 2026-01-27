use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::config::Config;

pub fn load_from_dir(dir: &Path) -> Result<Config> {
    let server = load_file(dir, "server.json")?;
    let world = load_file(dir, "world.json")?;
    let balance = load_file(dir, "balance.json")?;
    let survival = load_file(dir, "survival.json")?;
    let crafting = load_file(dir, "crafting.json")?;
    let spawning = load_file(dir, "spawning.json")?;
    let biomes = load_file(dir, "biomes.json")?;
    let settlements = load_file(dir, "settlements.json")?;
    let economy = load_file(dir, "economy.json")?;
    let quests = load_file(dir, "quests.json")?;
    let achievements = load_file(dir, "achievements.json")?;

    Ok(Config {
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
    })
}

fn load_file<T>(dir: &Path, name: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned + Default,
{
    let path = dir.join(name);
    if !path.exists() {
        return Ok(T::default());
    }
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("failed reading {}", path.display()))?;
    let parsed = serde_json::from_str(&contents)
        .with_context(|| format!("failed parsing {}", path.display()))?;
    Ok(parsed)
}
