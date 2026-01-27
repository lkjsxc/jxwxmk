use jxwxmk::server::database;

#[actix_rt::test]
async fn database_initializes_players_table() {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/jxwxmk".to_string());
    let db = database::init_pool(&db_url).await.unwrap();
    let row: (Option<String>,) = sqlx::query_as("SELECT to_regclass('public.players')")
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!(row.0, Some("players".to_string()));
}
