# Operations

Operational scripts, configuration, and deployment utilities.

## Structure

```
ops/
├── README.md              # This file
├── docker/                # Docker configuration
│   ├── Dockerfile         # Main Dockerfile
│   ├── Dockerfile.dev     # Development Dockerfile
│   ├── Dockerfile.prod    # Production Dockerfile
│   └── entrypoint.sh      # Entrypoint script
├── compose/               # Docker Compose files
│   ├── docker-compose.yml # Base compose file
│   ├── docker-compose.dev.yml # Development overrides
│   ├── docker-compose.prod.yml # Production overrides
│   └── docker-compose.test.yml # Test environment
├── scripts/               # Operational scripts
│   ├── startup.sh         # Startup script
│   ├── shutdown.sh        # Shutdown script
│   ├── backup.sh          # Backup script
│   ├── restore.sh         # Restore script
│   ├── migrate.sh         # Migration script
│   └── monitor.sh         # Monitoring script
├── config/                # Configuration templates
│   ├── server.toml.example # Server config example
│   ├── game.toml.example   # Game config example
│   ├── logging.toml.example # Logging config example
│   └── security.toml.example # Security config example
├── monitoring/            # Monitoring configuration
│   ├── prometheus.yml     # Prometheus config
│   ├── grafana/           # Grafana dashboards
│   └── alerts/            # Alert rules
└── deployment/            # Deployment utilities
    ├── kubernetes/        # Kubernetes manifests
    ├── ansible/           # Ansible playbooks
    └── terraform/         # Terraform configurations
```

## Docker Configuration

### Base Dockerfile

```dockerfile
# Multi-stage build for production

# Stage 1: Build frontend assets
FROM node:18-alpine as builder
WORKDIR /app

# Install dependencies
COPY src/client/package*.json ./
RUN npm install

# Copy source and build
COPY src/client ./
RUN npm run build

# Stage 2: Build Rust server
FROM rust:1.70 as rust-builder
WORKDIR /app

# Copy project files
COPY . ./

# Build release
RUN cargo build --release

# Stage 3: Final image
FROM debian:bullseye-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy built assets
COPY --from=builder /app/dist /app/static
COPY --from=rust-builder /app/target/release/server /app/server

# Copy configuration
COPY .env ./
COPY ops/docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Set up user
RUN useradd -m gameuser && chown -R gameuser /app
USER gameuser

EXPOSE 8080 8081
ENTRYPOINT ["/entrypoint.sh"]
```

### Development Dockerfile

```dockerfile
# Development build with hot-reload

FROM rust:1.70
WORKDIR /app

# Install Node.js for frontend development
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
&& apt-get install -y nodejs

# Copy project files
COPY . ./

# Install frontend dependencies
WORKDIR /app/src/client
RUN npm install

# Build frontend in development mode
WORKDIR /app
RUN cd src/client && npm run dev &

# Build and run server
EXPOSE 8080 8081 3000
CMD ["cargo", "run"]
```

### Entrypoint Script

```bash
#!/bin/sh
set -e

# Wait for database
until pg_isready -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER"; do
  echo "Waiting for database..."
  sleep 1
done

# Run migrations
echo "Running database migrations..."
# Implement migration runner here

# Start server
echo "Starting server..."
exec "./server"
```

## Docker Compose Configuration

### Base Compose File

```yaml
version: '3.8'

services:
  rust-server:
    build:
      context: .
      dockerfile: ops/docker/Dockerfile
    ports:
      - "${SERVER_PORT}:${SERVER_PORT}"
      - "${SERVER_WS_PORT}:${SERVER_WS_PORT}"
    environment:
      - SERVER_HOST=${SERVER_HOST}
      - SERVER_PORT=${SERVER_PORT}
      - SERVER_WS_PORT=${SERVER_WS_PORT}
      - SERVER_ENV=${SERVER_ENV}
      - DB_HOST=${DB_HOST}
      - DB_PORT=${DB_PORT}
      - DB_NAME=${DB_NAME}
      - DB_USER=${DB_USER}
      - DB_PASSWORD=${DB_PASSWORD}
      - GAME_TICK_RATE=${GAME_TICK_RATE}
      - GAME_MAX_PLAYERS=${GAME_MAX_PLAYERS}
      - JWT_SECRET=${JWT_SECRET}
      - LOG_LEVEL=${LOG_LEVEL}
    depends_on:
      postgres:
        condition: service_healthy
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:${SERVER_PORT}/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=${DB_NAME}
      - POSTGRES_USER=${DB_USER}
      - POSTGRES_PASSWORD=${DB_PASSWORD}
    ports:
      - "${DB_PORT}:${DB_PORT}"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./src/db/migrations:/docker-entrypoint-initdb.d
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${DB_USER} -d ${DB_NAME}"]
      interval: 5s
      timeout: 5s
      retries: 5
    restart: unless-stopped

volumes:
  postgres_data:
```

### Development Overrides

```yaml
version: '3.8'

services:
  rust-server:
    build:
      dockerfile: ops/docker/Dockerfile.dev
    volumes:
      - .:/app
      - /app/target
    environment:
      - SERVER_ENV=development
      - RUST_BACKTRACE=1
    ports:
      - "3000:3000"  # Frontend dev server

  postgres:
    ports:
      - "5432:5432"  # Expose for local development
```

### Production Overrides

```yaml
version: '3.8'

services:
  rust-server:
    build:
      dockerfile: ops/docker/Dockerfile.prod
    environment:
      - SERVER_ENV=production
    deploy:
      resources:
        limits:
          cpus: '2.0'
          memory: 2G
      restart_policy:
        condition: on-failure
        delay: 5s
        max_attempts: 3
        window: 120s

  postgres:
    volumes:
      - postgres_data:/var/lib/postgresql/data
    deploy:
      resources:
        limits:
          cpus: '1.0'
          memory: 1G
```

## Operational Scripts

### Startup Script

```bash
#!/bin/bash
set -e

echo "Starting game server..."

# Source environment variables
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

# Check dependencies
check_dependencies() {
    echo "Checking dependencies..."
    command -v docker >/dev/null 2>&1 || { echo >&2 "Docker is required but not installed."; exit 1; }
    command -v docker-compose >/dev/null 2>&1 || { echo >&2 "Docker Compose is required but not installed."; exit 1; }
}

# Start services
start_services() {
    echo "Starting services..."
    docker-compose -f ops/compose/docker-compose.yml \
                   -f ops/compose/docker-compose.$SERVER_ENV.yml \
                   up -d --build
}

# Monitor startup
monitor_startup() {
    echo "Monitoring startup..."
    docker-compose logs -f --tail=50
}

check_dependencies
start_services
monitor_startup
```

### Backup Script

```bash
#!/bin/bash
set -e

BACKUP_DIR="/backups/game"
DATE=$(date +%Y%m%d_%H%M%S)
DAYS_TO_KEEP=7

echo "Starting backup..."

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Database backup
echo "Backing up database..."
docker-compose exec postgres pg_dump -U "$DB_USER" -d "$DB_NAME" -F c -f "/backup/db_$DATE.dump"
docker cp "$(docker-compose ps -q postgres):/backup/db_$DATE.dump" "$BACKUP_DIR/"
docker-compose exec postgres rm "/backup/db_$DATE.dump"

# Clean up old backups
echo "Cleaning up old backups..."
find "$BACKUP_DIR" -name "*.dump" -mtime +$DAYS_TO_KEEP -delete

echo "Backup completed: $BACKUP_DIR/db_$DATE.dump"
```

### Migration Script

```bash
#!/bin/bash
set -e

echo "Running database migrations..."

# Wait for database to be ready
until docker-compose exec postgres pg_isready -U "$DB_USER" -d "$DB_NAME"; do
  echo "Waiting for database..."
  sleep 1
done

# Run migrations
for migration in src/db/migrations/*.sql; do
    echo "Applying $migration..."
    docker-compose exec postgres psql -U "$DB_USER" -d "$DB_NAME" -f "/docker-entrypoint-initdb.d/$(basename $migration)"
done

echo "Migrations completed successfully."
```

## Configuration Management

### Configuration Files

```toml
# server.toml
[server]
host = "0.0.0.0"
port = 8080
ws_port = 8081
environment = "production"
max_connections = 1000
worker_threads = 4

[network]
max_message_size = 8192
compression_enabled = true
ping_interval = 30
pong_timeout = 10

[cors]
allowed_origins = ["https://game.example.com"]
allowed_methods = ["GET", "POST", "OPTIONS"]
allowed_headers = ["Content-Type", "Authorization"]
```

### Environment Variables

```env
# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
SERVER_WS_PORT=8081
SERVER_ENV=production

# Database Configuration
DB_HOST=postgres
DB_PORT=5432
DB_NAME=game_db
DB_USER=game_user
DB_PASSWORD=secure_password
DB_SSL=false
DB_POOL_SIZE=20

# Game Configuration
GAME_TICK_RATE=20
GAME_MAX_PLAYERS=100
GAME_WORLD_SIZE=4000
GAME_RESOURCE_RESPAWN_MIN=300
GAME_RESOURCE_RESPAWN_MAX=900

# Security
JWT_SECRET=your_very_secure_random_string_here
JWT_EXPIRES_IN=24h
RATE_LIMIT_WINDOW=60
RATE_LIMIT_MAX=100
SESSION_TIMEOUT=86400

# Logging
LOG_LEVEL=info
LOG_FORMAT=json
LOG_FILE=/var/log/game/server.log
LOG_MAX_SIZE=100MB
LOG_MAX_FILES=10

# Performance
SIMULATION_WORKERS=4
NETWORK_WORKERS=2
COMPRESSION_LEVEL=6
```

## Monitoring Setup

### Prometheus Configuration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'game-server'
    static_configs:
      - targets: ['rust-server:9090']

  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres:5432']
    metrics_path: '/metrics'
    params:
      collect[]:
        - 'database'
        - 'activity'
        - 'replication'
```

### Grafana Dashboard

```json
{
  "title": "Game Server Dashboard",
  "panels": [
    {
      "title": "Server Health",
      "type": "graph",
      "targets": [
        {
          "expr": "up{job=\"game-server\"}",
          "legendFormat": "Server Up"
        }
      ]
    },
    {
      "title": "Players Online",
      "type": "singlestat",
      "targets": [
        {
          "expr": "game_players_online",
          "format": "time_series"
        }
      ],
      "valueMaps": [
        {"value": "null", "text": "N/A"}
      ]
    },
    {
      "title": "Simulation Performance",
      "type": "graph",
      "targets": [
        {
          "expr": "rate(game_tick_duration_seconds_sum[1m]) / rate(game_tick_duration_seconds_count[1m])",
          "legendFormat": "Avg Tick Duration"
        },
        {
          "expr": "histogram_quantile(0.95, sum(rate(game_tick_duration_seconds_bucket[1m])) by (le))",
          "legendFormat": "95th Percentile"
        }
      ]
    },
    {
      "title": "Network Traffic",
      "type": "graph",
      "targets": [
        {
          "expr": "rate(game_network_bytes_received[1m])",
          "legendFormat": "Received"
        },
        {
          "expr": "rate(game_network_bytes_sent[1m])",
          "legendFormat": "Sent"
        }
      ]
    },
    {
      "title": "Database Performance",
      "type": "graph",
      "targets": [
        {
          "expr": "pg_stat_database_xact_commit{datname=\"game_db\"}",
          "legendFormat": "Commits"
        },
        {
          "expr": "pg_stat_database_xact_rollback{datname=\"game_db\"}",
          "legendFormat": "Rollbacks"
        }
      ]
    }
  ],
  "templating": {
    "list": [
      {
        "name": "server",
        "query": "label_values(game_players_online, instance)",
        "label": "Server Instance"
      }
    ]
  }
}
```

## Deployment Strategies

### Blue-Green Deployment

```mermaid
flowchart TD
    A[Load Balancer] -->|Active| B[Green Environment]
    A -->|Standby| C[Blue Environment]
    D[New Version] --> C
    E[Test] --> C
    F[Switch] --> A
    A -->|Active| C
    A -->|Standby| B
```

### Rolling Deployment

```bash
# Kubernetes rolling update example
kubectl set image deployment/game-server \
    game-server=your-registry/game-server:v1.2.3

# Check rollout status
kubectl rollout status deployment/game-server

# Rollback if needed
kubectl rollout undo deployment/game-server
```

### Canary Deployment

```yaml
# Kubernetes canary deployment
apiVersion: apps/v1
kind: Deployment
metadata:
  name: game-server-canary
spec:
  replicas: 1
  selector:
    matchLabels:
      app: game-server
      track: canary
  template:
    metadata:
      labels:
        app: game-server
        track: canary
    spec:
      containers:
      - name: game-server
        image: your-registry/game-server:v1.2.3-canary
        ports:
        - containerPort: 8080
```

## Scaling Operations

### Horizontal Scaling

```yaml
# Kubernetes horizontal pod autoscaler
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: game-server-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: game-server
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

### Vertical Scaling

```yaml
# Docker Compose resource limits
services:
  rust-server:
    deploy:
      resources:
        limits:
          cpus: '4.0'
          memory: 4G
        reservations:
          cpus: '2.0'
          memory: 2G
```

## Disaster Recovery

### Backup Strategy

```
Daily Backups:
- Full database backup
- Retention: 7 days
- Storage: Local + Cloud

Weekly Backups:
- Full system snapshot
- Retention: 4 weeks
- Storage: Cloud

Monthly Backups:
- Archive backup
- Retention: 12 months
- Storage: Cold storage
```

### Recovery Procedures

```bash
# Database recovery from backup
1. Stop all services
2. Restore from latest backup:
   pg_restore -h localhost -U game_user -d game_db -c -j 4 backup_file.dump
3. Replay transaction logs if available
4. Verify data integrity
5. Restart services

# Full system recovery
1. Provision new infrastructure
2. Restore from latest system snapshot
3. Restore database from backup
4. Update DNS records
5. Monitor for issues
```

## CI/CD Pipeline

### GitHub Actions Example

```yaml
name: CI/CD Pipeline

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15-alpine
        env:
          POSTGRES_USER: test_user
          POSTGRES_PASSWORD: test_password
          POSTGRES_DB: test_db
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Run Rust tests
        run: cargo test
        env:
          DATABASE_URL: postgres://test_user:test_password@localhost:5432/test_db
      
      - name: Set up Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18
      
      - name: Install dependencies
        working-directory: src/client
        run: npm install
      
      - name: Run client tests
        working-directory: src/client
        run: npm test

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Build Docker image
        run: docker build -t game-server .
      
      - name: Push to registry
        if: github.ref == 'refs/heads/main'
        run: |
          echo "${{ secrets.DOCKER_PASSWORD }}" | docker login -u "${{ secrets.DOCKER_USERNAME }}" --password-stdin
          docker tag game-server your-registry/game-server:${{ github.sha }}
          docker push your-registry/game-server:${{ github.sha }}
          docker tag game-server your-registry/game-server:latest
          docker push your-registry/game-server:latest

  deploy:
    needs: build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - name: Deploy to production
        run: |
          # Add your deployment commands here
          # Example: kubectl set image deployment/game-server game-server=your-registry/game-server:${{ github.sha }}
          echo "Deployment would happen here"
```

## Monitoring and Alerting

### Key Metrics to Monitor

```
Server Metrics:
- CPU usage
- Memory usage
- Response times
- Error rates
- Players online

Database Metrics:
- Query performance
- Connection count
- Lock contention
- Replication lag
- Disk usage

Network Metrics:
- Latency
- Bandwidth usage
- Packet loss
- Connection rates
```

### Alert Rules

```yaml
# Example Prometheus alert rules
groups:
- name: server-alerts
  rules:
  - alert: HighCPUUsage
    expr: 100 - (avg by(instance) (rate(node_cpu_seconds_total{mode="idle"}[1m])) * 100) > 80
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High CPU usage on {{ $labels.instance }}"
      description: "CPU usage is {{ $value }}%"
  
  - alert: HighMemoryUsage
    expr: (node_memory_MemTotal_bytes - node_memory_MemAvailable_bytes) / node_memory_MemTotal_bytes * 100 > 85
    for: 10m
    labels:
      severity: warning
    annotations:
      summary: "High memory usage on {{ $labels.instance }}"
      description: "Memory usage is {{ $value }}%"
  
  - alert: ServerDown
    expr: up{job="game-server"} == 0
    for: 1m
    labels:
      severity: critical
    annotations:
      summary: "Server {{ $labels.instance }} is down"
      description: "Server has been down for more than 1 minute"

- name: database-alerts
  rules:
  - alert: DatabaseHighConnections
    expr: pg_stat_activity_count{datname="game_db"} > 50
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High database connections on {{ $labels.instance }}"
      description: "Database has {{ $value }} active connections"
```

## Future Enhancements

### Planned Improvements
1. **Automated Scaling**: Better auto-scaling policies
2. **Improved Monitoring**: More comprehensive metrics
3. **Enhanced Security**: Better security practices
4. **Disaster Recovery**: More robust backup strategies
5. **CI/CD Optimization**: Faster pipeline execution

### Technical Debt
- [ ] Improve deployment documentation
- [ ] Add comprehensive monitoring setup
- [ ] Implement proper secret management
- [ ] Add load testing to CI pipeline
- [ ] Document all operational procedures

## Related Documentation

- **Architecture**: See `../../docs/architecture/README.md`
- **Protocol**: See `../../docs/protocol/README.md`
- **Gameplay**: See `../../docs/gameplay/README.md`
- **Database**: See `../db/README.md`