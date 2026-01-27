use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new().max_connections(5).connect(database_url).await
}

pub async fn init_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS players (\
        id UUID PRIMARY KEY,\
        token UUID UNIQUE NOT NULL,\
        username TEXT NOT NULL,\
        level INT NOT NULL,\
        xp BIGINT NOT NULL,\
        x DOUBLE PRECISION NOT NULL,\
        y DOUBLE PRECISION NOT NULL,\
        health DOUBLE PRECISION NOT NULL,\
        hunger DOUBLE PRECISION NOT NULL,\
        inventory JSONB NOT NULL,\
        stats JSONB NOT NULL,\
        spawned BOOLEAN NOT NULL,\
        updated_at TIMESTAMPTZ NOT NULL\
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}
