use std::env;

pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/starve_game".to_string()),
            server_host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
        }
    }
}