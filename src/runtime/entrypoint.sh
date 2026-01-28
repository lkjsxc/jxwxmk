#!/bin/bash
set -e

# Initialize PostgreSQL data directory if empty
if [ -z "$(ls -A /var/lib/postgresql/data)" ]; then
    echo "Initializing PostgreSQL data directory..."
    su - postgres -c "/usr/lib/postgresql/15/bin/initdb -D /var/lib/postgresql/data"
fi

# Configure PostgreSQL to listen on localhost only
cat > /var/lib/postgresql/data/postgresql.conf <<EOF
listen_addresses = '127.0.0.1'
port = 5432
EOF

# Start PostgreSQL
echo "Starting PostgreSQL..."
su - postgres -c "/usr/lib/postgresql/15/bin/pg_ctl -D /var/lib/postgresql/data start"

# Wait for PostgreSQL to be ready
sleep 2

# Create database if it doesn't exist
su - postgres -c "psql -c \"CREATE DATABASE jxwxmk;\" 2>/dev/null || true"

# Start the game server
echo "Starting game server..."
exec /app/server
