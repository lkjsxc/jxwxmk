#[cfg(test)]
mod tests {
    use persistence::PersistenceManager;
    use uuid::Uuid;

    #[actix_rt::test]
    async fn test_migrations_and_session_claim() {
        let db_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/jxwxmk".to_string());
        
        let pm = match PersistenceManager::new(&db_url).await {
            Ok(p) => p,
            Err(_) => return,
        };

        pm.run_migrations().await.unwrap();

        let player_id = Uuid::new_v4();
        let token = pm.claim_session(player_id).await.unwrap();
        assert!(!token.is_nil());

        let pool = pm.get_pool();
        let _row = sqlx::query("SELECT id FROM players WHERE id = $1")
            .bind(player_id)
            .fetch_one(pool).await.unwrap();
    }
}