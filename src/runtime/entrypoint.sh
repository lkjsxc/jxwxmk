#!/bin/bash
set -e

# Start PostgreSQL
# We need to initialize the DB if it's empty (data volume).
if [ -z "$(ls -A /var/lib/postgresql/data)" ]; then
    echo "Initializing Database..."
    mkdir -p /var/lib/postgresql/data
    chown -R postgres:postgres /var/lib/postgresql
    su postgres -c "initdb -D /var/lib/postgresql/data"
    
    # Start momentarily to create user/db
    su postgres -c "pg_ctl -D /var/lib/postgresql/data -w start"
    su postgres -c "psql -c \"CREATE USER postgres WITH SUPERUSER PASSWORD 'postgres';\"" || true # might exist
    su postgres -c "createdb jxwxmk" || true
    su postgres -c "pg_ctl -D /var/lib/postgresql/data -m fast -w stop"
fi

echo "Starting PostgreSQL..."
su postgres -c "pg_ctl -D /var/lib/postgresql/data -w start"

echo "Starting Game Server..."
./server
