# Deployment Procedures

## Prerequisites
- Docker 20.10+
- Docker Compose v2+
- 2GB+ RAM (4GB recommended)
- 2+ CPU cores

## Quickstart

```bash
# Clone and setup
git clone https://github.com/your-repo/kkmypk.git
cd kkmypk
cp .env.example .env

# Build and start
docker compose up --build
```

## Environment Configuration

### Required Variables
```env
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
DB_HOST=postgres
DB_PORT=5432
DB_NAME=game_db
DB_USER=game_user
DB_PASSWORD=secure_password
```

### Optional Variables
```env
GAME_TICK_RATE=20
GAME_MAX_PLAYERS=50
LOG_LEVEL=info
```

## Docker Setup

### Development
```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml up --build
```

### Production
```bash
docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

## Health Checks

### Endpoints
- `GET /health` - Server health
- `GET /metrics` - Performance metrics
- `GET /status` - Game status

### Monitoring
```bash
# Check container status
docker ps

# View logs
docker logs rust-server

# Check database
docker exec -it postgres psql -U game_user -d game_db
```