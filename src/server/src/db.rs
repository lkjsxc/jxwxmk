use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::{info, error};
use thiserror::Error;

pub async fn create_pool(database_url: &str) -> Result<PgPool, DbError> {
    info!("Creating database connection pool for: {}", database_url);
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(database_url)
        .await
        .map_err(|e| DbError::ConnectionError(e.to_string()))?;
    
    // Test the connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|e| DbError::ConnectionTestError(e.to_string()))?;
    
    info!("Database connection pool created successfully");
    Ok(pool)
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),
    #[error("Database connection test failed: {0}")]
    ConnectionTestError(String),
    #[error("Database query error: {0}")]
    QueryError(String),
    #[error("Database migration error: {0}")]
    MigrationError(String),
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), DbError> {
    info!("Running database migrations...");
    
    // In a real implementation, this would run SQL migration files
    // For now, we'll just create the basic tables if they don't exist
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS players (
            id UUID PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            last_login TIMESTAMP WITH TIME ZONE
        );
        
        CREATE TABLE IF NOT EXISTS player_sessions (
            id UUID PRIMARY KEY,
            player_id UUID NOT NULL REFERENCES players(id),
            token VARCHAR(255) NOT NULL,
            expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        
        CREATE TABLE IF NOT EXISTS player_inventory (
            id UUID PRIMARY KEY,
            player_id UUID NOT NULL REFERENCES players(id),
            item_type VARCHAR(50) NOT NULL,
            item_id VARCHAR(50) NOT NULL,
            quantity INTEGER NOT NULL,
            slot INTEGER NOT NULL,
            UNIQUE(player_id, slot)
        );
        
        CREATE TABLE IF NOT EXISTS world_resources (
            id UUID PRIMARY KEY,
            resource_type VARCHAR(50) NOT NULL,
            position_x FLOAT NOT NULL,
            position_y FLOAT NOT NULL,
            quantity FLOAT NOT NULL,
            respawn_time TIMESTAMP WITH TIME ZONE,
            biome VARCHAR(50) NOT NULL
        );
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| DbError::MigrationError(e.to_string()))?;
    
    info!("Database migrations completed successfully");
    Ok(())
}

pub async fn get_player_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<Player>, DbError> {
    let player = sqlx::query_as::<_, Player>(
        "SELECT id, username, password_hash, created_at, last_login FROM players WHERE username = $1"
    )
    .bind(username)
    .fetch_optional(pool)
    .await
    .map_err(|e| DbError::QueryError(e.to_string()))?;
    
    Ok(player)
}

pub async fn create_player(
    pool: &PgPool,
    username: &str,
    password_hash: &str,
) -> Result<Player, DbError> {
    let player = sqlx::query_as::<_, Player>(
        "INSERT INTO players (id, username, password_hash) VALUES ($1, $2, $3) RETURNING id, username, password_hash, created_at, last_login"
    )
    .bind(uuid::Uuid::new_v4())
    .bind(username)
    .bind(password_hash)
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::QueryError(e.to_string()))?;
    
    Ok(player)
}

#[derive(Debug, sqlx::FromRow)]
pub struct Player {
    pub id: uuid::Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
}