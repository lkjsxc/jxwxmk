# Deployment Guide

## Development Environment

### Docker Compose Setup
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Local Development
```bash
# Backend
cd src/backend
cargo run

# Frontend
cd src/frontend
npm run dev

# Database setup
createdb starve_game
psql starve_game < migrations/init.sql
```

## Production Deployment

### Environment Variables
```bash
# Database
DATABASE_URL=postgresql://user:pass@localhost/starve_game
DATABASE_POOL_SIZE=10

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
WS_PATH=/ws

# Security
JWT_SECRET=your-secret-key
CORS_ORIGIN=https://yourdomain.com
```

### Docker Production
```dockerfile
# Backend Dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/starve_game /usr/local/bin/
CMD ["starve_game"]
```

### Nginx Configuration
```nginx
server {
    listen 80;
    server_name yourdomain.com;
    
    location / {
        root /var/www/html;
        try_files $uri $uri/ /index.html;
    }
    
    location /api {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
    
    location /ws {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

### Monitoring
- **Metrics**: Prometheus with Grafana
- **Logs**: ELK stack (Elasticsearch, Logstash, Kibana)
- **Health Checks**: HTTP endpoints for service status
- **Alerts**: Slack/email notifications for critical issues

### Backup Strategy
- **Database**: Daily automated backups to S3
- **Game State**: Real-time replication to standby server
- **Assets**: CDN distribution with fallback sources