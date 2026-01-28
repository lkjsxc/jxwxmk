#!/bin/bash
set -e

# Add PostgreSQL to PATH
export PATH=$PATH:/usr/lib/postgresql/15/bin

# Ensure correct permissions
mkdir -p /var/lib/postgresql/data
chown -R postgres:postgres /var/lib/postgresql
chmod 700 /var/lib/postgresql/data

# Version check
if [ -f "/var/lib/postgresql/data/PG_VERSION" ]; then
    VERSION=$(cat /var/lib/postgresql/data/PG_VERSION)
    if [ "$VERSION" != "15" ]; then
        echo "Incompatible Database Version ($VERSION != 15). Clearing data for re-initialization..."
        # We can't move the mount point, so we delete everything inside.
        rm -rf /var/lib/postgresql/data/*
        chown -R postgres:postgres /var/lib/postgresql/data
        chmod 700 /var/lib/postgresql/data
    fi
fi

# Start PostgreSQL
# We need to initialize the DB if it's empty (data volume).
if [ -z "$(ls -A /var/lib/postgresql/data)" ]; then
    echo "Initializing Database..."
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