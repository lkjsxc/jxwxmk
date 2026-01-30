# Docker Build Guide

## Quick Start

```bash
# Build the Docker image
docker build -f src/runtime/Dockerfile -t jxwxmk .

# Run the container
docker run --rm -p 8080:8080 \
  -v jxwxmk_pgdata:/var/lib/postgresql/data \
  -v ./config:/app/config:ro \
  jxwxmk

# Test endpoints
curl http://localhost:8080/health
curl http://localhost:8080/metrics
```

## Improved Build Options

### Option 1: Using Make (Recommended)

```bash
# Build and run
make dev

# Just build
make build

# Run tests
make test

# View logs
make logs

# Stop container
make stop

# Clean up
make clean
```

### Option 2: Using Docker Compose

```bash
# Start services
docker compose -f src/runtime/compose/docker-compose.yml up -d

# View logs
docker compose -f src/runtime/compose/docker-compose.yml logs -f

# Stop services
docker compose -f src/runtime/compose/docker-compose.yml down
```

### Option 3: Using Helper Scripts

```bash
# Build
./scripts/build.sh

# Or quick build
./scripts/build.sh quick

# Test
./scripts/test.sh
```

## Build Improvements Made

### 1. Optimized Dockerfile (`src/runtime/Dockerfile`)

**Caching Improvements:**
- Copy `package.json` before source files for better Node.js layer caching
- Copy `Cargo.toml` files before source for Rust dependency caching
- Use dummy files to pre-build dependencies (cached layer)
- Only rebuild when actual source changes

**Security Improvements:**
- Added `HEALTHCHECK` instruction
- Added Docker labels
- Use `--no-install-recommends` for smaller image
- Clean up apt cache

**Build Reliability:**
- Proper layer ordering for cache efficiency
- Explicit target stage selection
- Better error handling

### 2. Quick Build Dockerfile (`src/runtime/Dockerfile.quick`)

Uses `cargo-chef` for significantly faster rebuilds during development:

```bash
docker build -f src/runtime/Dockerfile.quick -t jxwxmk .
```

### 3. Makefile Targets

| Target | Description |
|--------|-------------|
| `make build` | Standard Docker build |
| `make build-quick` | Fast build with cargo-chef |
| `make run` | Run container in background |
| `make up` | Start with docker-compose |
| `make stop` | Stop and remove container |
| `make down` | Stop docker-compose |
| `make test` | Test endpoints |
| `make logs` | View container logs |
| `make shell` | Shell into running container |
| `make dev` | Build and run in one command |
| `make clean` | Remove image and volumes |

## Troubleshooting

### Build fails with "Cannot connect to Docker daemon"

```bash
# Check Docker status
sudo systemctl status docker

# Start Docker if needed
sudo systemctl start docker

# Or use Docker Desktop (macOS/Windows)
```

### Build fails with npm errors

```bash
# Clear npm cache
docker builder prune -f

# Rebuild without cache
docker build --no-cache -f src/runtime/Dockerfile -t jxwxmk .
```

### Build fails with cargo errors

```bash
# Check Rust syntax locally (if cargo available)
cd src/server && cargo check 2>&1 | head -20

# Or use quick build which handles caching better
./scripts/build.sh quick
```

### Container exits immediately

```bash
# Check logs
docker logs jxwxmk

# Run interactively to see errors
docker run --rm -it -p 8080:8080 \
  -v jxwxmk_pgdata:/var/lib/postgresql/data \
  -v ./config:/app/config:ro \
  jxwxmk
```

## Build Output

After successful build:

```
Successfully tagged jxwxmk:latest
```

Image details:
- **Name**: jxwxmk:latest
- **Size**: ~200-300MB (compressed)
- **Ports**: 8080 (game server)
- **Volumes**: 
  - `jxwxmk_pgdata` (PostgreSQL data)
  - `./config` (read-only config)

## Verification

After running, test these endpoints:

```bash
# Health check (should return "OK")
curl http://localhost:8080/health

# Metrics (should return Prometheus format)
curl http://localhost:8080/metrics | head

# Static assets (should return HTML)
curl -I http://localhost:8080/

# WebSocket (should upgrade connection)
curl -I -N -H "Connection: Upgrade" \
  -H "Upgrade: websocket" \
  -H "Sec-WebSocket-Key: test" \
  -H "Sec-WebSocket-Version: 13" \
  http://localhost:8080/ws
```

## Development Workflow

```bash
# 1. Make code changes
# ...

# 2. Quick rebuild (only changed layers)
make build-quick

# 3. Run and test
make dev
make test

# 4. View logs if needed
make logs
```

## Production Deployment

```bash
# Use image variant compose file
docker compose -f src/runtime/compose/docker-compose.image.yml up -d

# Or run directly with resource limits
docker run -d \
  --name jxwxmk \
  -p 8080:8080 \
  -v jxwxmk_pgdata:/var/lib/postgresql/data \
  -v $(pwd)/config:/app/config:ro \
  --memory=1g \
  --cpus=1.0 \
  --restart=unless-stopped \
  jxwxmk:latest
```
