# Docker Compose Configuration

## Base Configuration

```yaml
version: '3.8'

services:
  rust-server:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "${SERVER_PORT}:${SERVER_PORT}"
      - "${SERVER_WS_PORT}:${SERVER_WS_PORT}"
    environment:
      - SERVER_HOST=${SERVER_HOST}
      - DB_HOST=${DB_HOST}
      - DB_PORT=${DB_PORT}
      - DB_NAME=${DB_NAME}
      - DB_USER=${DB_USER}
      - DB_PASSWORD=${DB_PASSWORD}
    depends_on:
      postgres:
        condition: service_healthy
    restart: unless-stopped

  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=${DB_NAME}
      - POSTGRES_USER=${DB_USER}
      - POSTGRES_PASSWORD=${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${DB_USER} -d ${DB_NAME}"]
      interval: 5s
      timeout: 5s
      retries: 5
    restart: unless-stopped

volumes:
  postgres_data:
```

## Development Overrides

```yaml
version: '3.8'

services:
  rust-server:
    build:
      dockerfile: Dockerfile.dev
    volumes:
      - .:/app
      - /app/target
    environment:
      - SERVER_ENV=development
    ports:
      - "3000:3000"

  postgres:
    ports:
      - "5432:5432"
```

## Production Overrides

```yaml
version: '3.8'

services:
  rust-server:
    build:
      dockerfile: Dockerfile.prod
    environment:
      - SERVER_ENV=production
    deploy:
      resources:
        limits:
          cpus: '2.0'
          memory: 2G

  postgres:
    volumes:
      - postgres_data:/var/lib/postgresql/data
    deploy:
      resources:
        limits:
          cpus: '1.0'
          memory: 1G
```