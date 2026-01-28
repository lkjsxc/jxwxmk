use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub struct Persistence {
    pool: Pool<Postgres>,
}

impl Persistence {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }
}
