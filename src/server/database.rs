use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use uuid::Uuid;
use crate::game::world_state::{Player, Inventory, PlayerStats, Item, Structure, World};
use crate::game::quests::QuestState;
use crate::config::AppConfig;
use std::collections::{HashMap, HashSet};

pub type DbPool = Pool<Postgres>;

pub async fn init_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Run migrations
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS players (
            id UUID PRIMARY KEY,
            token UUID UNIQUE NOT NULL,
            username TEXT NOT NULL,
            x DOUBLE PRECISION NOT NULL,
            y DOUBLE PRECISION NOT NULL,
            health DOUBLE PRECISION NOT NULL,
            hunger DOUBLE PRECISION NOT NULL,
            inventory JSONB NOT NULL,
            stats JSONB NOT NULL,
            spawned BOOLEAN NOT NULL DEFAULT FALSE,
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        "#
    )
    .execute(&pool)
    .await?;

    sqlx::query("ALTER TABLE players ADD COLUMN IF NOT EXISTS quests JSONB NOT NULL DEFAULT '{}';")
    .execute(&pool)
    .await?;

    sqlx::query("ALTER TABLE players ADD COLUMN IF NOT EXISTS achievements JSONB NOT NULL DEFAULT '[]';")
    .execute(&pool)
    .await?;

    sqlx::query("ALTER TABLE players ADD COLUMN IF NOT EXISTS stat_bonuses JSONB NOT NULL DEFAULT '{}';")
    .execute(&pool)
    .await?;

    // World State Table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS world_state (
            id UUID PRIMARY KEY,
            structures JSONB NOT NULL DEFAULT '[]',
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        "#
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

pub async fn load_player(pool: &DbPool, token: Uuid) -> Result<Option<Player>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, token, username, x, y, health, hunger, inventory, stats, quests, achievements, stat_bonuses, spawned FROM players WHERE token = $1"
    )
    .bind(token)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        let inventory_json: serde_json::Value = row.get("inventory");
        let stats_json: serde_json::Value = row.get("stats");
        let quests_json: serde_json::Value = row.get("quests");
        let achievements_json: serde_json::Value = row.get("achievements");
        let stat_bonuses_json: serde_json::Value = row.get("stat_bonuses");

        let inventory: Inventory = serde_json::from_value(inventory_json).unwrap_or(Inventory::new(30));
        let stats: PlayerStats = serde_json::from_value(stats_json).unwrap_or_default();
        let quests: HashMap<String, QuestState> = serde_json::from_value(quests_json).unwrap_or_default();
        let achievements: HashSet<String> = serde_json::from_value(achievements_json).unwrap_or_default();
        let stat_bonuses: HashMap<String, f64> = serde_json::from_value(stat_bonuses_json).unwrap_or_default();

        Ok(Some(Player {
            id: row.get("id"),
            token: row.get("token"),
            username: row.get("username"),
            x: row.get("x"),
            y: row.get("y"),
            health: row.get("health"),
            hunger: row.get("hunger"),
            cold: 0.0,
            inventory,
            active_slot: 0,
            stats,
            achievements,
            quests,
            stat_bonuses,
            spawned: row.get("spawned"),
            last_attack_at: 0.0,
            last_interact_at: 0.0,
        }))
    } else {
        Ok(None)
    }
}

pub async fn save_player(pool: &DbPool, player: &Player) -> Result<(), sqlx::Error> {
    let inventory_json = serde_json::to_value(&player.inventory).unwrap();
    let stats_json = serde_json::to_value(&player.stats).unwrap();
    let quests_json = serde_json::to_value(&player.quests).unwrap();
    let achievements_json = serde_json::to_value(&player.achievements).unwrap();
    let stat_bonuses_json = serde_json::to_value(&player.stat_bonuses).unwrap();

    sqlx::query(
        r#"
        INSERT INTO players (id, token, username, x, y, health, hunger, inventory, stats, quests, achievements, stat_bonuses, spawned, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW())
        ON CONFLICT (id) DO UPDATE SET
            username = EXCLUDED.username,
            x = EXCLUDED.x,
            y = EXCLUDED.y,
            health = EXCLUDED.health,
            hunger = EXCLUDED.hunger,
            inventory = EXCLUDED.inventory,
            stats = EXCLUDED.stats,
            quests = EXCLUDED.quests,
            achievements = EXCLUDED.achievements,
            stat_bonuses = EXCLUDED.stat_bonuses,
            spawned = EXCLUDED.spawned,
            updated_at = NOW()
        "#
    )
    .bind(player.id)
    .bind(player.token)
    .bind(&player.username)
    .bind(player.x)
    .bind(player.y)
    .bind(player.health)
    .bind(player.hunger)
    .bind(inventory_json)
    .bind(stats_json)
    .bind(quests_json)
    .bind(achievements_json)
    .bind(stat_bonuses_json)
    .bind(player.spawned)
    .execute(pool)
    .await?;

    Ok(())
}

const WORLD_ID: Uuid = Uuid::nil();

pub async fn load_structures(pool: &DbPool) -> Result<HashMap<Uuid, Structure>, sqlx::Error> {
    let row = sqlx::query("SELECT structures FROM world_state WHERE id = $1")
        .bind(WORLD_ID)
        .fetch_optional(pool)
        .await?;

    if let Some(row) = row {
        let json: serde_json::Value = row.get("structures");
        let list: Vec<Structure> = serde_json::from_value(json).unwrap_or_default();
        let mut map = HashMap::new();
        for s in list {
            map.insert(s.id, s);
        }
        Ok(map)
    } else {
        Ok(HashMap::new())
    }
}

pub async fn save_structures(pool: &DbPool, structures: &HashMap<Uuid, Structure>) -> Result<(), sqlx::Error> {
    let list: Vec<&Structure> = structures.values().collect();
    let json = serde_json::to_value(&list).unwrap();

    sqlx::query(
        r#"
        INSERT INTO world_state (id, structures, updated_at)
        VALUES ($1, $2, NOW())
        ON CONFLICT (id) DO UPDATE SET
            structures = EXCLUDED.structures,
            updated_at = NOW()
        "#
    )
    .bind(WORLD_ID)
    .bind(json)
    .execute(pool)
    .await?;

    Ok(())
}