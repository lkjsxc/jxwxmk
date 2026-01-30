use sqlx::{PgPool, Row};
use uuid::Uuid;
use world::{PlayerState, World};
use protocol::{InventorySlot, PlayerStats, Vitals};
use chrono::Utc;

#[derive(Clone)]
pub struct PersistenceHandle {
    pool: PgPool,
}

impl PersistenceHandle {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        // Run migrations
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS players (
                id UUID PRIMARY KEY,
                token UUID UNIQUE NOT NULL,
                username TEXT NOT NULL,
                level INT NOT NULL DEFAULT 1,
                xp BIGINT NOT NULL DEFAULT 0,
                x DOUBLE PRECISION NOT NULL DEFAULT 0,
                y DOUBLE PRECISION NOT NULL DEFAULT 0,
                health DOUBLE PRECISION NOT NULL DEFAULT 100,
                hunger DOUBLE PRECISION NOT NULL DEFAULT 100,
                temperature DOUBLE PRECISION NOT NULL DEFAULT 50,
                inventory JSONB NOT NULL DEFAULT '[]'::jsonb,
                stats JSONB NOT NULL DEFAULT '{}'::jsonb,
                spawned BOOLEAN NOT NULL DEFAULT FALSE,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#
        ).execute(&self.pool).await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS settlements (
                id UUID PRIMARY KEY,
                name TEXT NOT NULL,
                core_level INT NOT NULL DEFAULT 1,
                core_integrity DOUBLE PRECISION NOT NULL DEFAULT 100,
                bounds JSONB NOT NULL DEFAULT '{}'::jsonb,
                state JSONB NOT NULL DEFAULT '{}'::jsonb,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#
        ).execute(&self.pool).await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS chunks (
                cx INT NOT NULL,
                cy INT NOT NULL,
                biome TEXT NOT NULL DEFAULT 'forest',
                state JSONB NOT NULL DEFAULT '{}'::jsonb,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                PRIMARY KEY (cx, cy)
            )
            "#
        ).execute(&self.pool).await?;

        log::info!("Migrations applied successfully");
        Ok(())
    }

    pub async fn load_player(&self, player_id: Uuid) -> Result<Option<PlayerState>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, username, level, xp, x, y, health, hunger, temperature,
                   inventory, stats, spawned
            FROM players
            WHERE id = $1
            "#
        )
        .bind(player_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let inventory_json: serde_json::Value = row.get("inventory");
            let stats_json: serde_json::Value = row.get("stats");
            
            let inventory: Vec<Option<InventorySlot>> = serde_json::from_value(inventory_json)
                .unwrap_or_else(|_| (0..30).map(|_| None).collect());
            
            let stats: PlayerStats = serde_json::from_value(stats_json)
                .unwrap_or_default();

            let mut player = PlayerState {
                id: row.get("id"),
                name: row.get("username"),
                spawned: row.get("spawned"),
                x: row.get("x"),
                y: row.get("y"),
                vitals: Vitals {
                    hp: row.get("health"),
                    max_hp: 100.0,
                    hunger: row.get("hunger"),
                    max_hunger: 100.0,
                    temperature: row.get("temperature"),
                    max_temperature: 100.0,
                },
                inventory,
                active_slot: 0,
                level: row.get("level"),
                xp: row.get("xp"),
                stats,
                quests: Vec::new(),
                achievements: Vec::new(),
                settlement_id: None,
                respawn_cooldown: 0.0,
            };

            Ok(Some(player))
        } else {
            Ok(None)
        }
    }

    pub async fn save_player(&self, player: &PlayerState) -> Result<(), sqlx::Error> {
        let inventory_json = serde_json::to_value(&player.inventory).unwrap_or_default();
        let stats_json = serde_json::to_value(&player.stats).unwrap_or_default();

        sqlx::query(
            r#"
            INSERT INTO players (id, username, level, xp, x, y, health, hunger, temperature,
                                inventory, stats, spawned, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW())
            ON CONFLICT (id) DO UPDATE SET
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
                updated_at = NOW()
            "#
        )
        .bind(player.id)
        .bind(&player.name)
        .bind(player.level)
        .bind(player.xp)
        .bind(player.x)
        .bind(player.y)
        .bind(player.vitals.hp)
        .bind(player.vitals.hunger)
        .bind(player.vitals.temperature)
        .bind(inventory_json)
        .bind(stats_json)
        .bind(player.spawned)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn rotate_token(&self, player_id: Uuid) -> Result<Uuid, sqlx::Error> {
        let new_token = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO players (id, token, username, updated_at)
            VALUES ($1, $2, $1::text, NOW())
            ON CONFLICT (id) DO UPDATE SET
                token = EXCLUDED.token,
                updated_at = NOW()
            "#
        )
        .bind(player_id)
        .bind(new_token)
        .execute(&self.pool)
        .await?;

        Ok(new_token)
    }

    pub async fn get_token(&self, player_id: Uuid) -> Result<Option<Uuid>, sqlx::Error> {
        let row = sqlx::query("SELECT token FROM players WHERE id = $1")
            .bind(player_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| r.get("token")))
    }

    pub async fn save_world(&self, _world: &World) -> Result<(), sqlx::Error> {
        // Chunk and settlement persistence
        // Implementation would save dirty chunks and settlements
        Ok(())
    }
}
