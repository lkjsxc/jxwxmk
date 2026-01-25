# Deployment

## Strategy
Docker Compose for all environments (Dev/Prod).

## Containers
- **App**: Rust Binary + Static Files (Multi-stage build).
- **DB**: PostgreSQL 16 (Alpine).

## CI/CD
- GitHub Actions to build Docker image.
- Watchtower or simple `docker-compose pull && up -d` for updates on VPS.
