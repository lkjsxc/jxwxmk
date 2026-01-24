# Rust Server

Server implementation using Rust, Actix Web, and Tokio.

## Structure

```
server/
├── README.md              # This file
├── Cargo.toml             # Rust project configuration
├── src/
│   ├── main.rs            # Entry point
│   ├── config/            # Configuration management
│   ├── network/           # Network handlers
│   ├── simulation/        # Game simulation
│   ├── world/             # World state management
│   ├── entities/          # Game entities
│   ├── systems/           # Game systems
│   ├── db/                # Database interface
│   ├── protocol/          # Protocol implementation
│   └── utils/             # Utilities and helpers
└── tests/                 # Server tests
```

## Key Components

### Main Entry Point
- **File**: `src/main.rs`
- **Responsibilities**:
  - Initialize logging and configuration
  - Set up database connections
  - Start network services
  - Launch simulation loop

### Configuration
- **Module**: `config/`
- **Files**:
  - `mod.rs` - Main config module
  - `server.rs` - Server configuration
  - `game.rs` - Gameplay configuration
  - `database.rs` - Database configuration

### Network Layer
- **Module**: `network/`
- **Files**:
  - `mod.rs` - Network module
  - `websocket.rs` - WebSocket handler
  - `http.rs` - HTTP endpoints
  - `messages.rs` - Message processing
  - `auth.rs` - Authentication

### Simulation Core
- **Module**: `simulation/`
- **Files**:
  - `mod.rs` - Simulation module
  - `loop.rs` - Main game loop
  - `tick.rs` - Tick processing
  - `events.rs` - Event system
  - `state.rs` - Game state management

### World Management
- **Module**: `world/`
- **Files**:
  - `mod.rs` - World module
  - `generation.rs` - World generation
  - `resources.rs` - Resource management
  - `entities.rs` - Entity management
  - `spatial.rs` - Spatial indexing

### Game Systems
- **Module**: `systems/`
- **Files**:
  - `mod.rs` - Systems module
  - `movement.rs` - Movement system
  - `combat.rs` - Combat system
  - `crafting.rs` - Crafting system
  - `survival.rs` - Survival mechanics
  - `inventory.rs` - Inventory management

### Database Interface
- **Module**: `db/`
- **Files**:
  - `mod.rs` - Database module
  - `models.rs` - Database models
  - `queries.rs` - Common queries
  - `migrations.rs` - Migration management
  - `connection.rs` - Connection pooling

### Protocol Implementation
- **Module**: `protocol/`
- **Files**:
  - `mod.rs` - Protocol module
  - `messages.rs` - Message definitions
  - `serialization.rs` - Serialization/deserialization
  - `binary.rs` - Binary protocol
  - `json.rs` - JSON protocol fallback

## Development Setup

### Prerequisites
- Rust 1.70+
- Cargo
- PostgreSQL 15+
- Docker (for full stack)

### Getting Started

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add required components
rustup component add clippy rustfmt

# Initialize project
cargo init

# Add dependencies (example Cargo.toml)
[dependencies]
actix-web = "4.0"
actix-web-actors = "4.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sqlx = { version = "0.6", features = ["postgres", "runtime-tokio-native-tls"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
thiserror = "1.0"
config = "0.13"
uuid = { version = "1.0", features = ["v4", "serde"] }
jsonwebtoken = "8.0"
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run
cargo run
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Test with all features
cargo test --all-features
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for warnings
cargo clippy -- -D warnings

# Generate documentation
cargo doc --open
```

## Architecture Patterns

### Simulation Loop
```rust
pub async fn run_simulation_loop() {
    let mut game_state = GameState::new();
    let mut tick = 0;
    
    loop {
        let start_time = Instant::now();
        
        // Process input events
        process_input_events(&mut game_state).await;
        
        // Update simulation
        update_simulation(&mut game_state, tick);
        
        // Send snapshots to clients
        send_snapshots(&game_state).await;
        
        // Calculate sleep time for target tick rate
        let processing_time = start_time.elapsed();
        let target_tick_time = Duration::from_secs_f64(1.0 / TICK_RATE as f64);
        let sleep_time = target_tick_time.saturating_sub(processing_time);
        
        tokio::time::sleep(sleep_time).await;
        tick += 1;
    }
}
```

### WebSocket Handler
```rust
pub struct GameWebSocket {
    player_id: Option<PlayerId>,
    outbound: mpsc::UnboundedSender<Message>,
}

impl Actor for GameWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(game_msg) = deserialize_message(&text) {
                    handle_game_message(game_msg, self.player_id, ctx.address());
                }
            }
            Ok(ws::Message::Binary(bin)) => {
                if let Ok(game_msg) = deserialize_binary(&bin) {
                    handle_game_message(game_msg, self.player_id, ctx.address());
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
```

### Database Models
```rust
#[derive(Debug, sqlx::FromRow)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_banned: bool,
    pub ban_reason: Option<String>,
    pub ban_expires: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PlayerProgress {
    pub player_id: Uuid,
    pub level: i32,
    pub experience: i64,
    pub last_position: Option<Point>,
    pub health: f32,
    pub hunger: f32,
    pub thirst: f32,
    pub inventory: Json<Value>,
    pub equipment: Json<Value>,
    pub skills: Json<Value>,
    pub statistics: Json<Value>,
    pub updated_at: DateTime<Utc>,
}
```

## Error Handling

### Custom Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Authentication failed")]
    Authentication,
    
    #[error("Invalid message format")]
    InvalidMessage,
    
    #[error("Rate limit exceeded")]
    RateLimited,
    
    #[error("Entity not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
}
```

### Error Responses
```rust
impl ResponseError for GameError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GameError::Authentication => HttpResponse::Unauthorized().json(json!({
                "error": "authentication_failed",
                "message": "Invalid credentials"
            })),
            GameError::RateLimited => HttpResponse::TooManyRequests().json(json!({
                "error": "rate_limited",
                "message": "Too many requests"
            })),
            _ => HttpResponse::InternalServerError().json(json!({
                "error": "internal_error",
                "message": self.to_string()
            })),
        }
    }
}
```

## Performance Considerations

### Optimization Strategies

1. **Simulation Performance**:
   - Use spatial partitioning for entity queries
   - Limit physics iterations per tick
   - Batch database operations

2. **Network Performance**:
   - Use binary protocol for production
   - Implement message compression for large payloads
   - Optimize serialization/deserialization

3. **Memory Management**:
   - Use object pooling for frequently created entities
   - Minimize allocations in hot paths
   - Use arena allocators for temporary data

### Profiling

```bash
# CPU profiling
cargo flamegraph --bin game-server

# Memory profiling
valgrind --tool=massif target/release/game-server

# Async profiling
tokio-console
```

## Security Practices

### Authentication
- Use JWT with secure signing algorithm
- Implement proper token rotation
- Store only token hashes in database

### Input Validation
- Validate all client inputs
- Use serde for structured validation
- Implement rate limiting

### Database Security
- Use parameterized queries
- Limit database user permissions
- Encrypt sensitive data at rest

## Testing Approach

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_combat_damage_calculation() {
        let attacker = Player::new(/* ... */);
        let defender = Player::new(/* ... */);
        let weapon = Weapon::new(/* ... */);
        
        let damage = calculate_damage(&attacker, &defender, &weapon);
        assert!(damage > 0);
        assert!(damage <= weapon.max_damage);
    }
}
```

### Integration Tests
```rust
#[actix_web::test]
async fn test_login_flow() {
    let app = test::init_service(App::new().configure(init_routes)).await;
    
    // Test successful login
    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(json!({
            "username": "testuser",
            "password": "testpass"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
```

## Deployment Notes

### Configuration
- Use environment variables for sensitive data
- Provide sensible defaults for development
- Validate configuration on startup

### Health Checks
- Implement `/health` endpoint
- Check database connectivity
- Verify critical subsystems

### Monitoring
- Expose metrics endpoint
- Log important events
- Implement structured logging

## Future Enhancements

### Planned Improvements
1. **Performance Optimization**: Spatial indexing, batching
2. **Scalability**: Horizontal scaling support
3. **Reliability**: Better error recovery
4. **Observability**: Enhanced metrics and tracing

### Technical Debt
- [ ] Improve error handling consistency
- [ ] Add more comprehensive tests
- [ ] Optimize serialization performance
- [ ] Enhance configuration management

## Related Documentation

- **Architecture**: See `../../docs/architecture/README.md`
- **Protocol**: See `../../docs/protocol/README.md`
- **Gameplay**: See `../../docs/gameplay/README.md`
- **Operations**: See `../../docs/operations/README.md`