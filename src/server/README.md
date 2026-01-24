# Rust Server

## Structure

```
server/
├── README.md              # This file
├── config/                # Configuration
│   └── README.md
├── network/               # Network handlers
│   └── README.md
├── simulation/            # Game simulation
│   └── README.md
├── world/                 # World management
│   └── README.md
├── systems/               # Game systems
│   └── README.md
├── db/                    # Database interface
│   └── README.md
└── protocol/              # Protocol implementation
    └── README.md
```

## Key Components

### Configuration
- TOML-based configuration
- Environment variable overrides
- Runtime configuration

### Network Layer
- WebSocket handlers
- HTTP endpoints
- Message processing

### Simulation Core
- Fixed-rate tick loop
- State management
- Event system

### World Management
- World generation
- Resource management
- Spatial indexing

### Game Systems
- Movement
- Combat
- Crafting
- Survival mechanics

### Database Interface
- Connection pooling
- Query utilities
- Migration management

### Protocol Implementation
- Message serialization
- Binary protocol
- JSON fallback

## Development Setup

```bash
# Build
cargo build

# Run
cargo run

# Test
cargo test
```

## Key Documentation

- **Config**: `config/README.md`
- **Network**: `network/README.md`
- **Simulation**: `simulation/README.md`
- **World**: `world/README.md`
- **Systems**: `systems/README.md`
- **Database**: `db/README.md`
- **Protocol**: `protocol/README.md`