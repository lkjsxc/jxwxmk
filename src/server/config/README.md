# Server Configuration

## Configuration Structure

```
config/
├── server.toml          # Server settings
├── game.toml            # Gameplay settings
├── database.toml        # Database settings
└── security.toml        # Security settings
```

## Server Configuration

```toml
[server]
host = "0.0.0.0"
port = 8080
ws_port = 8081
environment = "production"
worker_threads = 4
max_connections = 1000

[network]
max_message_size = 8192
compression_enabled = true
ping_interval = 30
pong_timeout = 10
```

## Game Configuration

```toml
[game]
tick_rate = 20
max_players = 100
world_size = 4000
resource_respawn_min = 300
resource_respawn_max = 900

[survival]
hunger_depletion_rate = 0.2
thirst_depletion_rate = 0.33
health_regeneration_rate = 0.1
```

## Database Configuration

```toml
[database]
host = "postgres"
port = 5432
name = "game_db"
user = "game_user"
password = "secure_password"
pool_size = 20
query_timeout = 5
```

## Security Configuration

```toml
[security]
jwt_secret = "your_secure_secret_here"
jwt_expires_in = "24h"
rate_limit_window = 60
rate_limit_max = 100
session_timeout = 86400
```

## Configuration Loading

```rust
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub ws_port: u16,
    pub environment: String,
    pub worker_threads: usize,
    pub max_connections: usize,
}

impl ServerConfig {
    pub fn load() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::with_name("config/server"))
            .add_source(config::Environment::with_prefix("SERVER"))
            .build()?;
        
        config.try_deserialize()
    }
}
```