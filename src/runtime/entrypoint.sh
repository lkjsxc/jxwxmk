#!/bin/bash
set -e

# Start PostgreSQL locally
echo "Starting PostgreSQL..."
su - postgres -c "/usr/lib/postgresql/15/bin/initdb -D $PGDATA" || true
su - postgres -c "/usr/lib/postgresql/15/bin/pg_ctl -D $PGDATA -l /var/lib/postgresql/logfile start"

# Wait for Postgres
until su - postgres -c "psql -c '\l'"; do
  echo "Waiting for Postgres..."
  sleep 1
done

# Create DB and User if they don't exist
su - postgres -c "psql -c \"CREATE USER jxwxmk WITH PASSWORD 'jxwxmk';\"" || true
su - postgres -c "psql -c \"CREATE DATABASE jxwxmk OWNER jxwxmk;\"" || true

# Run the game server
echo "Starting Game Server..."
exec ./server