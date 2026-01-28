use sqlx::{Pool, Postgres};
use world::{Chunk, Settlement};
use serde_json::json;

pub async fn save_chunk(pool: &Pool<Postgres>, chunk: &Chunk) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO chunks (cx, cy, biome, state, updated_at)
           VALUES ($1, $2, $3, $4, NOW())
           ON CONFLICT (cx, cy) DO UPDATE SET
             biome = EXCLUDED.biome,
             state = EXCLUDED.state,
             updated_at = NOW()"#
    )
    .bind(chunk.coord.x)
    .bind(chunk.coord.y)
    .bind(&chunk.biome)
    .bind(json!({
        "resources": chunk.resources,
        "mobs": chunk.mobs,
        "structures": chunk.structures,
        "npcs": chunk.npcs,
    }))
    .execute(pool).await?;

    Ok(())
}

pub async fn save_settlement(pool: &Pool<Postgres>, settlement: &Settlement) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO settlements (id, name, core_level, core_integrity, bounds, state, updated_at)
           VALUES ($1, $2, $3, $4, $5, $6, NOW())
           ON CONFLICT (id) DO UPDATE SET
             name = EXCLUDED.name,
             core_level = EXCLUDED.core_level,
             core_integrity = EXCLUDED.core_integrity,
             bounds = EXCLUDED.bounds,
             state = EXCLUDED.state,
             updated_at = NOW()"#
    )
    .bind(settlement.id)
    .bind(&settlement.name)
    .bind(settlement.core_level as i32)
    .bind(settlement.core_integrity as f64)
    .bind(json!({ "radius": settlement.bounds_radius }))
    .bind(json!(settlement.state))
    .execute(pool).await?;

    Ok(())
}
