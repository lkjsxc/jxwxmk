use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use uuid::Uuid;

use crate::config::Config;
use crate::game::world::entities::{Inventory, PlayerState};

pub type Db = PgPool;

#[derive(Clone, Debug, FromRow)]
pub struct PlayerRecord {
    pub id: Uuid,
    pub token: Uuid,
    pub username: String,
    pub level: i32,
    pub xp: i64,
    pub x: f64,
    pub y: f64,
    pub health: f64,
    pub hunger: f64,
    pub inventory: Value,
    pub stats: Value,
    pub spawned: bool,
    pub updated_at: DateTime<Utc>,
}

impl PlayerRecord {
    pub fn into_player_state(self, config: &Config) -> PlayerState {
        let inventory = serde_json::from_value(self.inventory)
            .unwrap_or_else(|_| Inventory::new(config.balance.player.inventory_slots));
        let stats = serde_json::from_value(self.stats).unwrap_or_default();
        let mut player = PlayerState::new(self.id, self.token, config.balance.player.inventory_slots);
        player.username = self.username;
        player.level = self.level as u32;
        player.xp = self.xp as u64;
        player.x = self.x as f32;
        player.y = self.y as f32;
        player.health = self.health as f32;
        player.hunger = self.hunger as f32;
        player.inventory = inventory;
        player.stats = stats;
        player.spawned = self.spawned;
        player
    }
}

pub async fn init_pool(database_url: &str) -> anyhow::Result<Db> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    ensure_schema(&pool).await?;
    Ok(pool)
}

async fn ensure_schema(pool: &Db) -> anyhow::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS players (
            id UUID PRIMARY KEY,
            token UUID UNIQUE NOT NULL,
            username TEXT NOT NULL,
            level INT NOT NULL,
            xp BIGINT NOT NULL,
            x DOUBLE PRECISION NOT NULL,
            y DOUBLE PRECISION NOT NULL,
            health DOUBLE PRECISION NOT NULL,
            hunger DOUBLE PRECISION NOT NULL,
            inventory JSONB NOT NULL,
            stats JSONB NOT NULL,
            spawned BOOLEAN NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn load_player_by_token(db: &Db, token: Uuid) -> anyhow::Result<Option<PlayerRecord>> {
    let record = sqlx::query_as::<_, PlayerRecord>(
        "SELECT id, token, username, level, xp, x, y, health, hunger, inventory, stats, spawned, updated_at
         FROM players WHERE token = $1",
    )
    .bind(token)
    .fetch_optional(db)
    .await?;
    Ok(record)
}

pub async fn load_player_by_id(db: &Db, id: Uuid) -> anyhow::Result<Option<PlayerRecord>> {
    let record = sqlx::query_as::<_, PlayerRecord>(
        "SELECT id, token, username, level, xp, x, y, health, hunger, inventory, stats, spawned, updated_at
         FROM players WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;
    Ok(record)
}

pub async fn upsert_player(db: &Db, player: &PlayerState) -> anyhow::Result<()> {
    let inventory = serde_json::to_value(&player.inventory).unwrap_or_else(|_| Value::Null);
    let stats = serde_json::to_value(&player.stats).unwrap_or_else(|_| Value::Null);
    sqlx::query(
        "INSERT INTO players (id, token, username, level, xp, x, y, health, hunger, inventory, stats, spawned, updated_at)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
         ON CONFLICT (id)
         DO UPDATE SET token=$2, username=$3, level=$4, xp=$5, x=$6, y=$7, health=$8, hunger=$9, inventory=$10, stats=$11, spawned=$12, updated_at=$13",
    )
    .bind(player.id)
    .bind(player.token)
    .bind(&player.username)
    .bind(player.level as i32)
    .bind(player.xp as i64)
    .bind(player.x as f64)
    .bind(player.y as f64)
    .bind(player.health as f64)
    .bind(player.hunger as f64)
    .bind(inventory)
    .bind(stats)
    .bind(player.spawned)
    .bind(Utc::now())
    .execute(db)
    .await?;
    Ok(())
}

pub async fn claim_player(db: &Db, player_id: Uuid, config: &Config) -> anyhow::Result<(Uuid, Uuid)> {
    let existing = load_player_by_id(db, player_id).await?;
    let token = Uuid::new_v4();
    match existing {
        Some(record) => {
            let mut player = record.into_player_state(config);
            player.token = token;
            upsert_player(db, &player).await?;
        }
        None => {
            let mut player = PlayerState::new(player_id, token, config.balance.player.inventory_slots);
            player.spawned = false;
            upsert_player(db, &player).await?;
        }
    }
    Ok((player_id, token))
}
