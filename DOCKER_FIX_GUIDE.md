# Docker Setup and Fix Guide

## Current Issues

1. **Docker daemon not running**: The Docker service needs to be started
2. **Rust version compatibility**: Updated from 1.70 to 1.80
3. **Missing package-lock.json**: Removed requirement from frontend Dockerfile
4. **Obsolete version attribute**: Removed from docker-compose.yml

## Step-by-Step Fix

### 1. Start Docker Daemon

```bash
# Try these commands to start Docker:

# Method 1: Using systemctl (recommended)
sudo systemctl start docker

# Method 2: If systemctl doesn't work
sudo dockerd &

# Method 3: If you get permission errors
sudo usermod -aG docker $USER
newgrp docker
```

### 2. Verify Docker is Running

```bash
docker info
# Should show server information without errors
```

### 3. Build and Start Services

```bash
# Navigate to project directory
cd /home/lkjsxc/repos/kkmypk

# Build and start containers
export DOCKER_HOST="unix://$XDG_RUNTIME_DIR/docker.sock"
docker compose up -d --build
```

### 4. Troubleshooting

#### If you get "Cannot connect to Docker daemon"

```bash
# Check if Docker socket exists
ls -la /var/run/docker.sock

# If it doesn't exist, start Docker manually
sudo systemctl unmask docker
sudo systemctl enable docker
sudo systemctl start docker
```

#### If you get Rust version errors

The Dockerfile has been updated to use Rust 1.80. If you still get errors:

```bash
# Update the Rust version in Dockerfile.backend
sed -i 's/FROM rust:.*/FROM rust:1.80/' Dockerfile.backend
```

#### If you get package-lock.json errors

The requirement has been removed from Dockerfile.frontend. If you still get errors:

```bash
# Generate package-lock.json
cd src/frontend
npm install
cd ../..
```

### 5. Manual Service Startup (Alternative)

If Docker still doesn't work, you can run services manually:

#### Backend (Rust)

```bash
cd src/backend
cargo run
```

#### Frontend (TypeScript)

```bash
cd src/frontend
npm install
npm run dev
```

#### Database (PostgreSQL)

Install PostgreSQL locally and create the database:

```bash
sudo apt-get install postgresql
sudo -u postgres psql -c "CREATE DATABASE starveio;"
```

### 6. Verify Services

Once running, verify services are accessible:

- **Backend**: http://localhost:8080/health
- **Frontend**: http://localhost:3000
- **Database**: localhost:5432

## Files Modified

1. **docker-compose.yml**: Removed obsolete `version` attribute
2. **Dockerfile.backend**: Updated Rust version from 1.70 to 1.80
3. **Dockerfile.frontend**: Removed package-lock.json requirement

## Common Errors and Solutions

### Error: "Cannot connect to Docker daemon"
**Solution**: Start Docker daemon as shown above

### Error: "rustc version too old"
**Solution**: Updated Dockerfile to use Rust 1.80

### Error: "package-lock.json not found"
**Solution**: Removed requirement from Dockerfile

### Error: "version attribute is obsolete"
**Solution**: Removed version from docker-compose.yml

## Next Steps

Once Docker is working:

1. Run `docker compose up -d --build`
2. Access the application at http://localhost:3000
3. Check backend health at http://localhost:8080/health

If you continue to have issues, please check:
- Docker is installed and running
- You have permissions to access Docker
- No other services are using ports 8080, 3000, or 5432