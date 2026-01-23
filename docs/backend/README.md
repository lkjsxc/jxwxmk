# Backend Documentation

## Overview

The backend is built with Rust using the following technologies:
- **Actix Web**: Web framework for HTTP and WebSocket
- **Tokio**: Async runtime
- **SQLx**: PostgreSQL database access
- **Serde**: JSON serialization

## Module Structure

```
/src/backend/src
├── main.rs              # Entry point
├── game/               # Game logic
│   ├── mod.rs          # Game module
│   ├── player.rs       # Player logic
│   ├── world.rs        # World management
│   ├── items.rs        # Item system
│   └── crafting.rs     # Crafting system
├── database/           # Database operations
│   └── mod.rs          # Database module
└── websocket/          # WebSocket handling
    └── mod.rs          # WebSocket module
```

## API Endpoints

### WebSocket
- `/ws`: Real-time game communication

### REST API
- `GET /health`: Health check endpoint
- Additional endpoints to be added

## Database Schema

The database uses PostgreSQL with the following main tables:

- `players`: Player accounts and progress
- `game_state`: Persistent game world state
- `inventory`: Player inventory items
- `buildings`: Player-constructed buildings

## Environment Variables

- `HOST`: Server host (default: 0.0.0.0)
- `PORT`: Server port (default: 8080)
- `DATABASE_URL`: PostgreSQL connection URL
- `RUST_LOG`: Logging level