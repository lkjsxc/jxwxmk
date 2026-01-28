use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::migrate::Migrator;
use std::path::Path;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    MIGRATOR.run(pool).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_db_init_and_migrate() {
        // This test requires a running Postgres instance.
        // In the Docker test environment, we expect DATABASE_URL to point to a valid DB.
        
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        // We might need to ensure the DB exists or is clean.
        // For now, just connecting and migrating is a good sanity check.
        
        let pool = init_pool(&db_url).await.expect("Failed to create pool");
        run_migrations(&pool).await.expect("Failed to run migrations");
        
        // Check if tables exist
        let row: (bool,) = sqlx::query_as("SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'players')")
            .fetch_one(&pool)
            .await
            .expect("Query failed");
            
        assert!(row.0, "players table should exist");
    }
}
