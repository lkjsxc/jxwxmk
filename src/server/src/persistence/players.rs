use chrono::Utc;
use serde_json::Value;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::config::Config;
use crate::game::{Inventory, PlayerId, PlayerState, PlayerStats};
use crate::util::now_utc;

pub async fn load_player_by_token(
    pool: &PgPool,
    token: Uuid,
    config: &Config,
) -> Option<PlayerState> {
    let row = sqlx::query(
        "SELECT id, token, username, level, xp, x, y, health, hunger, inventory, stats, spawned \
         FROM players WHERE token = $1",
    )
    .bind(token)
    .fetch_optional(pool)
    .await
    .ok()?;

    row.map(|row| decode_player_row(row, config))
}

pub async fn load_player_by_id(
    pool: &PgPool,
    player_id: PlayerId,
    config: &Config,
) -> Option<PlayerState> {
    let row = sqlx::query(
        "SELECT id, token, username, level, xp, x, y, health, hunger, inventory, stats, spawned \
         FROM players WHERE id = $1",
    )
    .bind(player_id)
    .fetch_optional(pool)
    .await
    .ok()?;

    row.map(|row| decode_player_row(row, config))
}

pub async fn save_player(pool: &PgPool, player: &PlayerState) -> Result<(), sqlx::Error> {
    let inventory = serde_json::to_value(&player.inventory).unwrap_or(Value::Null);
    let stats = serde_json::to_value(&player.stats).unwrap_or(Value::Null);
    sqlx::query(
        "INSERT INTO players (id, token, username, level, xp, x, y, health, hunger, inventory, stats, spawned, updated_at)\
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)\
         ON CONFLICT (id) DO UPDATE SET \
         token = EXCLUDED.token,\
         username = EXCLUDED.username,\
         level = EXCLUDED.level,\
         xp = EXCLUDED.xp,\
         x = EXCLUDED.x,\
         y = EXCLUDED.y,\
         health = EXCLUDED.health,\
         hunger = EXCLUDED.hunger,\
         inventory = EXCLUDED.inventory,\
         stats = EXCLUDED.stats,\
         spawned = EXCLUDED.spawned,\
         updated_at = EXCLUDED.updated_at"
    )
    .bind(player.id)
    .bind(player.token)
    .bind(&player.username)
    .bind(player.level as i32)
    .bind(player.xp)
    .bind(player.x as f64)
    .bind(player.y as f64)
    .bind(player.health as f64)
    .bind(player.hunger as f64)
    .bind(inventory)
    .bind(stats)
    .bind(player.spawned)
    .bind(now_utc())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn rotate_token(pool: &PgPool, player_id: PlayerId, token: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE players SET token = $1, updated_at = $2 WHERE id = $3")
        .bind(token)
        .bind(Utc::now())
        .bind(player_id)
        .execute(pool)
        .await?;
    Ok(())
}

fn decode_player_row(row: sqlx::postgres::PgRow, config: &Config) -> PlayerState {
    let id: Uuid = row.get("id");
    let token: Uuid = row.get("token");
    let username: String = row.get("username");
    let level: i32 = row.get("level");
    let xp: i64 = row.get("xp");
    let x: f64 = row.get("x");
    let y: f64 = row.get("y");
    let health: f64 = row.get("health");
    let hunger: f64 = row.get("hunger");
    let inventory_value: Value = row.get("inventory");
    let stats_value: Value = row.get("stats");
    let spawned: bool = row.get("spawned");

    let inventory: Inventory = serde_json::from_value(inventory_value)
        .unwrap_or_else(|_| Inventory::new(config.balance.player.inventory_slots));
    let stats: PlayerStats = serde_json::from_value(stats_value).unwrap_or_default();

    let mut player = PlayerState::new(id, token, inventory.slots.len(), config.balance.player.max_health);
    player.username = username;
    player.level = level.max(1) as u32;
    player.xp = xp;
    player.x = x as f32;
    player.y = y as f32;
    player.health = health as f32;
    player.hunger = hunger as f32;
    player.inventory = inventory;
    player.stats = stats;
    player.spawned = spawned;
    player
}
