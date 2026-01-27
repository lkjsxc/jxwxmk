use crate::config::{
    AchievementsConfig, BalanceConfig, BiomesConfig, Config, CraftingConfig, EconomyConfig,
    QuestsConfig, ServerConfig, SettlementsConfig, SpawningConfig, SurvivalConfig, WorldConfig,
};
use crate::game::chunk::{Chunk, ChunkCoord};
use crate::game::entities::{Mob, PlayerState};
use crate::game::systems::actions::try_attack;
use crate::game::world::World;
use uuid::Uuid;

fn base_config() -> Config {
    Config::new(
        ServerConfig::default(),
        WorldConfig::default(),
        BalanceConfig::default(),
        SurvivalConfig::default(),
        CraftingConfig::default(),
        SpawningConfig::default(),
        BiomesConfig::default(),
        SettlementsConfig::default(),
        EconomyConfig::default(),
        QuestsConfig::default(),
        AchievementsConfig::default(),
    )
}

#[test]
fn attack_reduces_mob_health() {
    let config = base_config();
    let mut world = World::new(0);
    let mut player = PlayerState::new(Uuid::new_v4(), Uuid::new_v4(), 5, 100.0);
    player.spawned = true;
    player.x = 0.0;
    player.y = 0.0;
    player.chunk = (0, 0);
    world.upsert_player(player);

    let coord = ChunkCoord::new(0, 0);
    let mut chunk = Chunk::new(coord, "forest".to_string());
    chunk.mobs.insert(
        "mob".to_string(),
        Mob {
            id: "mob".to_string(),
            m_type: "wolf".to_string(),
            level: 1,
            health: 20.0,
            max_health: 20.0,
            x: 0.5,
            y: 0.5,
        },
    );
    world.chunks.insert(coord, chunk);

    let player_id = *world.players.keys().next().unwrap();
    try_attack(&mut world, &config, player_id);
    let chunk = world.chunks.get(&coord).unwrap();
    let mob = chunk.mobs.get("mob").unwrap();
    assert!(mob.health < 20.0);
}
