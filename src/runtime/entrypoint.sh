#!/bin/bash
set -e

# Ensure PostgreSQL data directory exists and has correct permissions
mkdir -p /var/lib/postgresql/data
chown -R postgres:postgres /var/lib/postgresql/data

# Initialize PostgreSQL if needed
if [ ! -d "/var/lib/postgresql/data/base" ]; then
    echo "Initializing PostgreSQL database..."
    su - postgres -c "/usr/lib/postgresql/15/bin/initdb -D /var/lib/postgresql/data"
    
    # Configure PostgreSQL to allow local connections
    echo "host all all 127.0.0.1/32 trust" >> /var/lib/postgresql/data/pg_hba.conf
    echo "listen_addresses='127.0.0.1'" >> /var/lib/postgresql/data/postgresql.conf
fi

# Start PostgreSQL
echo "Starting PostgreSQL..."
su - postgres -c "/usr/lib/postgresql/15/bin/pg_ctl -D /var/lib/postgresql/data -l /var/lib/postgresql/data/pg_log start"

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to start..."
until su - postgres -c "/usr/lib/postgresql/15/bin/pg_isready -h 127.0.0.1"; do
  sleep 1
done

# Create database if it doesn't exist
echo "Ensuring database exists..."
su - postgres -c "/usr/lib/postgresql/15/bin/psql -h 127.0.0.1 -c 'CREATE DATABASE kkmypk;'" || true

# Start the game server
echo "Starting Game Server..."
exec /app/kkmypk
