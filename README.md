# Starve.io-like Game

A multiplayer survival game inspired by starve.io, built with Rust, Tokio, Actix Web, PostgreSQL, TypeScript, and Docker Compose.

## Features

- **Multiplayer survival gameplay** with real-time interactions
- **Rich game content** with simple pixel-art graphics
- **Survival mechanics**: hunger, thirst, temperature, health
- **Crafting system**: tools, weapons, buildings, and resources
- **Persistent world** with PostgreSQL backend
- **Web-based frontend** with TypeScript and Canvas rendering
- **Dockerized development** for easy setup

## Technology Stack

### Backend
- **Rust** (stable) with **Tokio** async runtime
- **Actix Web** for HTTP and WebSocket servers
- **SQLx** for PostgreSQL database access
- **Serde** for JSON serialization

### Frontend
- **TypeScript** with **Vite** build system
- **Canvas API** for game rendering
- **WebSocket** for real-time communication

### Infrastructure
- **PostgreSQL** database
- **Docker Compose** for containerization
- **Nginx** (optional for production)

## Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/starveio-clone.git
cd starveio-clone

# Start development environment
docker-compose up -d

# Run backend (in separate terminal)
cd src/backend
cargo run

# Run frontend (in separate terminal)
cd src/frontend
npm install
npm run dev
```

Access the game at: http://localhost:3000

## Project Structure

```
starveio-clone/
├── README.md                  # This file
├── LICENSE                    # License information
├── AGENTS.md                  # Development guidelines
├── .env                       # Environment variables
├── docker-compose.yml         # Docker configuration
│
├── docs/                      # Documentation
│   ├── README.md              # Documentation overview
│   ├── architecture/          # System architecture
│   ├── backend/               # Backend docs
│   ├── frontend/              # Frontend docs
│   ├── game-design/           # Game design
│   └── deployment/            # Deployment guides
│
├── src/
│   ├── backend/               # Rust backend
│   │   ├── Cargo.toml         # Rust project config
│   │   ├── src/               # Rust source
│   │   │   ├── main.rs        # Entry point
│   │   │   ├── game/          # Game logic
│   │   │   ├── database/      # DB operations
│   │   │   └── websocket/     # WebSocket handling
│   │   └── migrations/        # Database migrations
│   │
│   └── frontend/              # TypeScript frontend
│       ├── package.json       # Node.js config
│       ├── vite.config.ts     # Vite configuration
│       ├── public/            # Static files
│       └── src/               # TypeScript source
│           ├── main.ts        # Entry point
│           ├── game/          # Game client
│           └── render/        # Rendering
│
└── Dockerfile.*              # Docker build files
```

## Game Design

### Core Mechanics
- **Survival**: Manage hunger, thirst, temperature, and health
- **Crafting**: Combine resources to create tools, weapons, and buildings
- **Gathering**: Collect wood, stone, food, and other resources
- **Combat**: Fight animals and other players
- **Building**: Construct shelters and defensive structures

### World Features
- **Biomes**: Forest, desert, snow, and plains
- **Day/Night cycle**: Affects visibility and monster spawning
- **Dynamic weather**: Impacts gameplay
- **Procedural generation**: Randomly generated worlds

### Multiplayer
- **Cooperative play**: Team up with friends
- **Trading system**: Exchange resources
- **Clans**: Form groups for better survival
- **PvP zones**: Optional player combat areas

## Development

### Backend Development

```bash
cd src/backend

# Build and run
cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Frontend Development

```bash
cd src/frontend

# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Run tests
npm test

# Format code
npm run format
```

### Database

```bash
# Access PostgreSQL
docker-compose exec db psql -U postgres -d starveio

# Run migrations
# (Implemented in backend startup)
```

## Deployment

### Development
```bash
docker-compose up -d
```

### Production
```bash
# Build production images
docker-compose -f docker-compose.yml build

# Start services
docker-compose up -d
```

See [Deployment Guide](docs/deployment/README.md) for detailed production setup.

## Architecture

The system follows a client-server architecture:

1. **Client**: TypeScript frontend with Canvas rendering
2. **Server**: Rust backend with game logic
3. **Database**: PostgreSQL for persistent storage
4. **Communication**: WebSocket for real-time updates

See [Architecture Documentation](docs/architecture/README.md) for details.

## Documentation

- [Game Design](docs/game-design/README.md)
- [Backend API](docs/backend/README.md)
- [Frontend Structure](docs/frontend/README.md)
- [Deployment Guide](docs/deployment/README.md)

## Contributing

Follow the guidelines in [AGENTS.md](AGENTS.md):
- Keep source files under 200 lines
- Keep documentation files under 300 lines
- Follow Rust and TypeScript code conventions
- Commit frequently with descriptive messages
- Use recursive directory structure

## License

This project is licensed under the terms in the [LICENSE](LICENSE) file.

## Roadmap

- [ ] Basic game mechanics implementation
- [ ] Multiplayer synchronization
- [ ] Crafting system
- [ ] World generation
- [ ] UI/UX improvements
- [ ] Mobile responsiveness
- [ ] Performance optimization

## Support

For issues and questions, please open a GitHub issue.

---

*Inspired by starve.io - a fun multiplayer survival game with simple graphics and rich content.*