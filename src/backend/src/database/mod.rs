use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        
        Ok(Database { pool })
    }

    pub async fn init(&self) -> Result<(), sqlx::Error> {
        sqlx::migrate!().run(&self.pool).await?;
        Ok(())
    }
}