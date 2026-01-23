# Starve.io Clone - Multiplayer Survival Game

A multiplayer survival game inspired by starve.io with rich content and simple graphics.

## Project Overview

This is a browser-based multiplayer survival game where players gather resources, craft items, build shelters, and survive in a hostile environment. The game features real-time multiplayer interactions, persistent world state, and simple but engaging graphics.

## Technology Stack

- **Backend**: Rust with tokio and Actix Web
- **Frontend**: TypeScript with HTML5 Canvas
- **Database**: PostgreSQL
- **Containerization**: Docker Compose
- **Real-time Communication**: WebSockets

## Quick Start

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

## Game Features

### Core Mechanics
- Resource gathering (wood, stone, food, etc.)
- Crafting system with recipes
- Building construction
- Hunger and health management
- Day/night cycle
- Weather system

### Multiplayer Features
- Real-time player movement
- Chat system
- Trading between players
- Collaborative building
- PvP combat (optional zones)

### World Design
- Procedurally generated terrain
- Different biomes (forest, desert, snow, etc.)
- Wildlife and creatures
- Resource distribution
- Environmental hazards

## Development

### Prerequisites
- Rust 1.70+
- Node.js 18+
- Docker & Docker Compose
- PostgreSQL 14+

### Local Development

```bash
# Backend
cd src/backend
cargo run

# Frontend
cd src/frontend
npm install
npm run dev

# Database
docker-compose up postgres
```

## Project Structure

```
├── docs/                 # Documentation
│   ├── architecture/     # System architecture
│   ├── game-design/      # Game mechanics
│   ├── api/             # API documentation
│   └── deployment/      # Deployment guides
├── src/
│   ├── backend/         # Rust backend
│   ├── frontend/        # TypeScript frontend
│   └── shared/          # Shared types/utilities
├── docker-compose.yml   # Development environment
├── Cargo.toml          # Rust dependencies
└── package.json        # Node.js dependencies
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Implement your changes
4. Add tests
5. Submit a pull request

## License

Apache License 2.0 - see LICENSE file for details.