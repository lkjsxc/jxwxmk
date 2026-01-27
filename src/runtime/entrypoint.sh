#!/bin/bash
set -e

PG_DATA="/var/lib/postgresql/data"

# Initialize PostgreSQL if not already done
if [ ! -d "$PG_DATA/base" ]; then
    echo "Initializing PostgreSQL..."
    mkdir -p "$PG_DATA"
    chown -R postgres:postgres "$PG_DATA"
    su - postgres -c "/usr/lib/postgresql/15/bin/initdb -D $PG_DATA"
    
    # Configure to listen only on localhost (default) and trust local connections
    echo "host all all 127.0.0.1/32 trust" >> "$PG_DATA/pg_hba.conf"
    echo "host all all ::1/128 trust" >> "$PG_DATA/pg_hba.conf"
fi

echo "Starting PostgreSQL..."
su - postgres -c "/usr/lib/postgresql/15/bin/pg_ctl -D $PG_DATA -l /tmp/pg.log start"

# Wait for Postgres to be ready
until su - postgres -c "psql -c '\l'"; do
  echo "Waiting for Postgres..."
  sleep 1
done

echo "Creating database if not exists..."
su - postgres -c "psql -tc \"SELECT 1 FROM pg_database WHERE datname = 'kkmypk'\" | grep -q 1 || psql -c \"CREATE DATABASE kkmypk\""

echo "Starting Game Server..."
./kkmypk
