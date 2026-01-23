# Deployment Guide

## Prerequisites

- Docker and Docker Compose installed
- Domain name (for production)
- SSL certificates (for production)

## Development Deployment

```bash
# Start all services
docker-compose up -d

# Access services
- Backend: http://localhost:8080
- Frontend: http://localhost:3000
- Database: localhost:5432
```

## Production Deployment

### Build and Run

```bash
# Build production images
docker-compose -f docker-compose.yml -f docker-compose.prod.yml build

# Start services
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

### Environment Configuration

Create a `.env.production` file:

```
# Backend
HOST=0.0.0.0
PORT=8080
DATABASE_URL=postgres://user:password@db:5432/starveio
RUST_LOG=info

# Frontend
NODE_ENV=production
VITE_API_URL=https://yourdomain.com/api
```

### Scaling

For production scaling:

1. **Backend**: Run multiple instances behind a load balancer
2. **Frontend**: Use CDN for static assets
3. **Database**: Set up PostgreSQL replication
4. **Monitoring**: Add logging and metrics collection

### Updates

```bash
# Pull latest changes
git pull origin main

# Rebuild and restart
docker-compose down
docker-compose up -d --build
```

## Troubleshooting

### Common Issues

- **Database connection failed**: Check `DATABASE_URL` and PostgreSQL service
- **Port conflicts**: Ensure ports 8080, 3000, 5432 are available
- **Build failures**: Clean and rebuild (`docker-compose build --no-cache`)

### Logs

```bash
# View service logs
docker-compose logs -f backend
docker-compose logs -f frontend
docker-compose logs -f db
```