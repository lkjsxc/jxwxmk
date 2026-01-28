use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;
use world::{PlayerId, PlayerState, World};
use serde_json;
use std::collections::HashMap;

pub struct Persistence {
    pool: Pool<Postgres>,
}

impl Persistence {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Persistence { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS players (
                id UUID PRIMARY KEY,
                token UUID UNIQUE NOT NULL,
                username TEXT NOT NULL,
                level INT NOT NULL,
                xp BIGINT NOT NULL,
                x DOUBLE PRECISION NOT NULL,
                y DOUBLE PRECISION NOT NULL,
                health DOUBLE PRECISION NOT NULL,
                hunger DOUBLE PRECISION NOT NULL,
                temperature DOUBLE PRECISION NOT NULL,
                inventory JSONB NOT NULL,
                stats JSONB NOT NULL,
                spawned BOOLEAN NOT NULL,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS settlements (
                id UUID PRIMARY KEY,
                name TEXT NOT NULL,
                core_level INT NOT NULL,
                core_integrity DOUBLE PRECISION NOT NULL,
                bounds JSONB NOT NULL,
                state JSONB NOT NULL,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS chunks (
                cx INT NOT NULL,
                cy INT NOT NULL,
                biome TEXT NOT NULL,
                state JSONB NOT NULL,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                PRIMARY KEY (cx, cy)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        log::info!("Database migrations completed");
        Ok(())
    }

    pub async fn load_player(
        &self,
        player_id: PlayerId,
    ) -> Result<Option<PlayerState>, sqlx::Error> {
        let row = sqlx::query_as::<_, PlayerRow>(
            "SELECT * FROM players WHERE id = $1"
        )
        .bind(player_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    pub async fn save_player(
        &self,
        player: &PlayerState,
        token: Uuid,
    ) -> Result<(), sqlx::Error> {
        let inventory_json: serde_json::Value = serde_json::json!(
            player.inventory.iter().map(|slot| {
                match slot {
                    Some(s) => serde_json::json!({"item": s.item, "count": s.count}),
                    None => serde_json::Value::Null,
                }
            }).collect::<Vec<_>>()
        );
        
        let stats_json = serde_json::json!({
            "steps": player.stats.steps,
            "kills": player.stats.kills,
            "crafts": player.stats.crafts,
            "gathers": player.stats.gathers,
            "deaths": player.stats.deaths,
        });

        sqlx::query(
            r#"
            INSERT INTO players (id, token, username, level, xp, x, y, health, hunger, temperature, inventory, stats, spawned, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW())
            ON CONFLICT (id) DO UPDATE SET
                token = EXCLUDED.token,
                username = EXCLUDED.username,
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
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(player.id)
        .bind(token)
        .bind(&player.name)
        .bind(player.level)
        .bind(player.xp)
        .bind(player.x as f64)
        .bind(player.y as f64)
        .bind(player.vitals.hp as f64)
        .bind(player.vitals.hunger as f64)
        .bind(player.vitals.temperature as f64)
        .bind(inventory_json)
        .bind(stats_json)
        .bind(player.spawned)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn rotate_token(
        &self,
        player_id: PlayerId,
        new_token: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE players SET token = $1, updated_at = NOW() WHERE id = $2")
            .bind(new_token)
            .bind(player_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_token(&self,
        player_id: PlayerId,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        let row: Option<(TokenRow)> = sqlx::query_as("SELECT token FROM players WHERE id = $1")
            .bind(player_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.token))
    }

    pub async fn save_settlement(
        &self,
        settlement: &world::Settlement,
    ) -> Result<(), sqlx::Error> {
        let bounds_json = serde_json::json!({
            "x": settlement.core_x,
            "y": settlement.core_y,
            "radius": settlement.safe_zone_radius,
        });
        
        let state_json = serde_json::json!({
            "core_integrity": settlement.core_integrity,
            "core_level": settlement.core_level,
        });

        sqlx::query(
            r#"
            INSERT INTO settlements (id, name, core_level, core_integrity, bounds, state, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW())
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                core_level = EXCLUDED.core_level,
                core_integrity = EXCLUDED.core_integrity,
                bounds = EXCLUDED.bounds,
                state = EXCLUDED.state,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(settlement.id)
        .bind(&settlement.name)
        .bind(settlement.core_level)
        .bind(settlement.core_integrity as f64)
        .bind(bounds_json)
        .bind(state_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn load_settlements(
        &self,
    ) -> Result<Vec<world::Settlement>, sqlx::Error> {
        let rows: Vec<SettlementRow> = sqlx::query_as("SELECT * FROM settlements")
            .fetch_all(&self.pool)
            .await?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn save_chunk(
        &self,
        chunk: &world::Chunk,
    ) -> Result<(), sqlx::Error> {
        let state_json = serde_json::json!({
            "resources": chunk.resources,
            "mobs": chunk.mobs,
            "structures": chunk.structures,
            "npcs": chunk.npcs,
            "settlement_id": chunk.settlement_id,
        });

        sqlx::query(
            r#"
            INSERT INTO chunks (cx, cy, biome, state, updated_at)
            VALUES ($1, $2, $3, $4, NOW())
            ON CONFLICT (cx, cy) DO UPDATE SET
                biome = EXCLUDED.biome,
                state = EXCLUDED.state,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(chunk.coord.0)
        .bind(chunk.coord.1)
        .bind(&chunk.biome_id)
        .bind(state_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn load_chunks(
        &self,
    ) -> Result<Vec<world::Chunk>, sqlx::Error> {
        let rows: Vec<ChunkRow> = sqlx::query_as("SELECT * FROM chunks")
            .fetch_all(&self.pool)
            .await?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct PlayerRow {
    id: Uuid,
    username: String,
    level: i32,
    xp: i64,
    x: f64,
    y: f64,
    health: f64,
    hunger: f64,
    temperature: f64,
    inventory: serde_json::Value,
    stats: serde_json::Value,
    spawned: bool,
}

impl From<PlayerRow> for PlayerState {
    fn from(row: PlayerRow) -> Self {
        let mut player = PlayerState::new(row.id, row.username);
        player.level = row.level;
        player.xp = row.xp;
        player.x = row.x as f32;
        player.y = row.y as f32;
        player.vitals.hp = row.health as f32;
        player.vitals.hunger = row.hunger as f32;
        player.vitals.temperature = row.temperature as f32;
        player.spawned = row.spawned;
        player
    }
}

#[derive(sqlx::FromRow)]
struct TokenRow {
    token: Uuid,
}

#[derive(sqlx::FromRow)]
struct SettlementRow {
    id: Uuid,
    name: String,
    core_level: i32,
    core_integrity: f64,
    bounds: serde_json::Value,
    state: serde_json::Value,
}

impl From<SettlementRow> for world::Settlement {
    fn from(row: SettlementRow) -> Self {
        let bounds = row.bounds.as_object().unwrap();
        let x = bounds.get("x").unwrap().as_f64().unwrap() as f32;
        let y = bounds.get("y").unwrap().as_f64().unwrap() as f32;
        let radius = bounds.get("radius").unwrap().as_f64().unwrap() as f32;
        
        world::Settlement {
            id: row.id,
            name: row.name,
            core_level: row.core_level,
            core_integrity: row.core_integrity as f32,
            core_x: x,
            core_y: y,
            safe_zone_radius: radius,
        }
    }
}

#[derive(sqlx::FromRow)]
struct ChunkRow {
    cx: i32,
    cy: i32,
    biome: String,
    state: serde_json::Value,
}

impl From<ChunkRow> for world::Chunk {
    fn from(row: ChunkRow) -> Self {
        let mut chunk = world::Chunk::new((row.cx, row.cy));
        chunk.biome_id = row.biome;
        
        if let Some(resources) = row.state.get("resources") {
            if let Ok(res_map) = serde_json::from_value::<HashMap<String, world::Resource>>(resources.clone()) {
                chunk.resources = res_map;
            }
        }
        
        if let Some(mobs) = row.state.get("mobs") {
            if let Ok(mob_map) = serde_json::from_value::<HashMap<String, world::Mob>>(mobs.clone()) {
                chunk.mobs = mob_map;
            }
        }
        
        if let Some(structures) = row.state.get("structures") {
            if let Ok(struct_map) = serde_json::from_value::<HashMap<String, world::Structure>>(structures.clone()) {
                chunk.structures = struct_map;
            }
        }
        
        if let Some(npcs) = row.state.get("npcs") {
            if let Ok(npc_map) = serde_json::from_value::<HashMap<String, world::Npc>>(npcs.clone()) {
                chunk.npcs = npc_map;
            }
        }
        
        if let Some(settlement_id) = row.state.get("settlement_id") {
            if let Ok(id) = serde_json::from_value::<Uuid>(settlement_id.clone()) {
                chunk.settlement_id = Some(id);
            }
        }
        
        chunk
    }
}
