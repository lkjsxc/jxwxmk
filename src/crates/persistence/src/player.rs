use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use world::{PlayerState, Vec2, ChunkCoord};
use serde_json::json;
use std::collections::{HashMap, HashSet};

pub async fn load_player_by_token(pool: &Pool<Postgres>, token: Uuid) -> Result<Option<PlayerState>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, username, level, xp, x, y, health, hunger, temperature, inventory, stats, spawned 
         FROM players WHERE token = $1"
    )
    .bind(token)
    .fetch_optional(pool).await?;

    if let Some(row) = row {
        Ok(Some(PlayerState {
            id: row.get("id"),
            token,
            name: row.get("username"),
            level: row.get::<i32, _>("level") as u32,
            xp: row.get::<i64, _>("xp") as u64,
            pos: Vec2 { x: row.get::<f64, _>("x") as f32, y: row.get::<f64, _>("y") as f32 },
            chunk: ChunkCoord { x: 0, y: 0 },
            hp: row.get::<f64, _>("health") as f32,
            max_hp: 100.0,
            hunger: row.get::<f64, _>("hunger") as f32,
            thirst: 100.0, // Default for now
            temp: row.get::<f64, _>("temperature") as f32,
            inventory: serde_json::from_value(row.get("inventory")).unwrap_or_default(),
            active_slot: 0,
            stats: serde_json::from_value(row.get("stats")).unwrap_or_default(),
            unlocked_achievements: HashSet::new(),
            stat_bonuses: HashMap::new(),
            active_quests: Vec::new(),
            spawned: row.get("spawned"),
            active_view: HashSet::new(),
            input_dx: 0.0,
            input_dy: 0.0,
            input_attack: false,
            input_interact: false,
            input_aim: None,
        }))
    } else {
        Ok(None)
    }
}

pub async fn save_player(pool: &Pool<Postgres>, player: &PlayerState) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO players (id, token, username, level, xp, x, y, health, hunger, temperature, inventory, stats, spawned, updated_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW())
           ON CONFLICT (id) DO UPDATE SET
             level = EXCLUDED.level,
             xp = EXCLUDED.xp,
             x = EXCLUDED.x,
             y = EXCLUDED.y,
             health = EXCLUDED.health,
             hunger = EXCLUDED.hunger,
             temperature = EXCLUDED.temperature,
             inventory = EXCLUDED.inventory,
             stats = EXCLUDED.stats,
             spawned = EXCLUDED.spawned,
             updated_at = NOW()"#
    )
    .bind(player.id)
    .bind(player.token)
    .bind(&player.name)
    .bind(player.level as i32)
    .bind(player.xp as i64)
    .bind(player.pos.x as f64)
    .bind(player.pos.y as f64)
    .bind(player.hp as f64)
    .bind(player.hunger as f64)
    .bind(player.temp as f64)
    .bind(json!(player.inventory))
    .bind(json!(player.stats))
    .bind(player.spawned)
    .execute(pool).await?;

    Ok(())
}