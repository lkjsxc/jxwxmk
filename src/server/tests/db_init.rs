use sqlx::Row;

use jxwxmk::persistence::{init_db, init_pool};

#[actix_web::test]
async fn init_db_creates_players_table() {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return,
    };

    let pool = init_pool(&database_url).await.expect("db");
    init_db(&pool).await.expect("init db");

    let row = sqlx::query("SELECT to_regclass('public.players')::text as name")
        .fetch_one(&pool)
        .await
        .expect("query");
    let name: Option<String> = row.get("name");
    assert_eq!(name.as_deref(), Some("players"));
}
