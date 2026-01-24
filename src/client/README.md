# TypeScript Client

## Structure

```
client/
├── README.md              # This file
├── game/                  # Game logic
│   └── README.md
├── network/               # Network communication
│   └── README.md
├── ui/                    # User interface
│   └── README.md
├── rendering/             # Rendering system
│   └── README.md
├── input/                 # Input handling
│   └── README.md
├── state/                 # State management
│   └── README.md
└── assets/                # Asset management
    └── README.md
```

## Key Components

### Game Logic
- Game loop implementation
- World management
- Entity system
- Event handling

### Network Communication
- WebSocket client
- Message handling
- Reconnection logic

### User Interface
- HUD implementation
- Inventory system
- Crafting interface
- Chat system

### Rendering System
- Canvas management
- Sprite rendering
- Camera system
- Particle effects

### Input Handling
- Keyboard input
- Mouse input
- Gamepad support
- Key bindings

### State Management
- Game state
- Player state
- World state
- Synchronization

### Asset Management
- Asset loading
- Asset caching
- Sprite management
- Sound management

## Development Setup

```bash
# Install dependencies
npm install

# Development build
npm run dev

# Production build
npm run build
```

## Key Documentation

- **Game**: `game/README.md`
- **Network**: `network/README.md`
- **UI**: `ui/README.md`
- **Rendering**: `rendering/README.md`
- **Input**: `input/README.md`
- **State**: `state/README.md`
- **Assets**: `assets/README.md`