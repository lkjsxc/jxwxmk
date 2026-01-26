use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    bind: String,
    pub database_url: String,
    pub tick_hz: u64,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let bind = env::var("APP_BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/kkmypk".to_string());
        let tick_hz = env::var("TICK_HZ")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(20);

        Self {
            bind,
            database_url,
            tick_hz,
        }
    }

    pub fn bind_address(&self) -> String {
        self.bind.clone()
    }
}
