use sqlx::{PgPool, FromRow};
use crate::world::Player;

#[derive(FromRow)]
pub struct DbPlayer {
    pub id: String,
    pub position_x: f32,
    pub position_y: f32,
    pub health: f32,
    pub hunger: f32,
}

pub async fn load_player(pool: &PgPool, account_id: i32) -> Result<Option<Player>, sqlx::Error> {
    let db_player: Option<DbPlayer> = sqlx::query_as!(
        DbPlayer,
        "SELECT id, position_x, position_y, health, hunger FROM players WHERE account_id = $1",
        account_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(db_player.map(|p| Player {
        id: p.id,
        position: (p.position_x, p.position_y),
        health: p.health,
        hunger: p.hunger,
        inventory: std::collections::HashMap::new(),  // Load separately
    }))
}

pub async fn save_player(pool: &PgPool, player: &Player) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO players (id, position_x, position_y, health, hunger) VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (id) DO UPDATE SET position_x = $2, position_y = $3, health = $4, hunger = $5",
        player.id,
        player.position.0,
        player.position.1,
        player.health,
        player.hunger
    )
    .execute(pool)
    .await?;
    Ok(())
}