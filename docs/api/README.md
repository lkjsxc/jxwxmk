# API Documentation

## WebSocket API

### Connection
```javascript
const ws = new WebSocket('ws://localhost:8080/ws');
```

### Message Types

#### Client → Server
- `player_move` - Move player to new position
- `player_action` - Perform action (gather, craft, build)
- `chat_message` - Send chat message
- `inventory_update` - Update inventory state

#### Server → Client
- `game_state` - Current game state update
- `player_position` - Other player positions
- `chat_broadcast` - Chat messages from others
- `world_update` - Environmental changes

## REST API

### Authentication
```
POST /api/auth/register
POST /api/auth/login
POST /api/auth/logout
```

### Player Data
```
GET /api/player/profile
PUT /api/player/profile
GET /api/player/inventory
PUT /api/player/inventory
```

### Game Data
```
GET /api/game/world
GET /api/game/items
GET /api/game/recipes
GET /api/game/leaderboard
```

## Data Models

### Player
```rust
struct Player {
    id: Uuid,
    username: String,
    position: Vector2,
    health: i32,
    hunger: i32,
    inventory: Vec<Item>,
    equipment: Equipment,
}
```

### World
```rust
struct World {
    id: Uuid,
    seed: u64,
    size: Vector2,
    biomes: Vec<Biome>,
    entities: Vec<Entity>,
}
```

### Item
```rust
struct Item {
    id: Uuid,
    name: String,
    item_type: ItemType,
    quantity: u32,
    durability: Option<u32>,
}
```