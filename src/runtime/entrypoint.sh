#!/bin/bash
set -e

# PostgreSQL binaries path
export PATH="/usr/lib/postgresql/15/bin:$PATH"

# Fix permissions for PostgreSQL data directory (needed when using volumes)
mkdir -p $PGDATA
chown -R postgres:postgres /var/lib/postgresql
chmod 700 $PGDATA

# Initialize PostgreSQL if data directory is empty
if [ -z "$(ls -A $PGDATA 2>/dev/null)" ]; then
    echo "Initializing PostgreSQL database..."
    su - postgres -c "/usr/lib/postgresql/15/bin/initdb -D $PGDATA"
    
    # Configure PostgreSQL to listen on localhost only
    echo "listen_addresses = '127.0.0.1'" >> $PGDATA/postgresql.conf
    echo "port = 5432" >> $PGDATA/postgresql.conf
fi

# Start PostgreSQL
echo "Starting PostgreSQL..."
su - postgres -c "/usr/lib/postgresql/15/bin/pg_ctl -D $PGDATA -l $PGDATA/logfile start"

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL..."
until su - postgres -c "/usr/lib/postgresql/15/bin/pg_isready -h 127.0.0.1 -p 5432"; do
    sleep 1
done

# Create database if it doesn't exist
su - postgres -c "/usr/lib/postgresql/15/bin/psql -h 127.0.0.1 -p 5432 -c 'CREATE DATABASE jxwxmk;' 2>/dev/null || true"

echo "PostgreSQL is ready"

# Graceful shutdown function
shutdown() {
    echo "Shutting down..."
    kill $SERVER_PID 2>/dev/null || true
    wait $SERVER_PID 2>/dev/null || true
    su - postgres -c "/usr/lib/postgresql/15/bin/pg_ctl -D $PGDATA stop" 2>/dev/null || true
    exit 0
}

trap shutdown SIGTERM SIGINT

# Start the game server
echo "Starting game server..."
/app/jxwxmk-server &
SERVER_PID=$!

# Wait for server process
wait $SERVER_PID
