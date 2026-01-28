use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use std::time::Duration;
use world::PlayerState;

pub mod player;
pub mod world_state;

#[derive(Clone)]
pub struct PersistenceManager {
    pool: Pool<Postgres>,
}

impl PersistenceManager {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::migrate::MigrateError> {
        sqlx::migrate!("./migrations").run(&self.pool).await
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    pub async fn claim_session(&self, player_id: Uuid) -> Result<Uuid, sqlx::Error> {
        let new_token = Uuid::new_v4();
        
        sqlx::query(
            r#"INSERT INTO players (id, token, username, level, xp, x, y, health, hunger, temperature, inventory, stats, spawned, updated_at)
               VALUES ($1, $2, $3, 1, 0, 64.0, 64.0, 100.0, 100.0, 50.0, '[]', '{}', false, NOW())
               ON CONFLICT (id) DO UPDATE SET
                 token = EXCLUDED.token,
                 updated_at = NOW()"#
        )
        .bind(player_id)
        .bind(new_token)
        .bind(format!("Player_{}", &player_id.to_string()[..8]))
        .execute(&self.pool).await?;

        Ok(new_token)
    }

    pub async fn get_player_by_token(&self, token: Uuid) -> Result<Option<PlayerState>, sqlx::Error> {
        player::load_player_by_token(&self.pool, token).await
    }
}

pub fn init() {}