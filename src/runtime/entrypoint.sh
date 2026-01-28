#!/bin/bash
set -e

echo "Starting PostgreSQL..."
service postgresql start

# Wait for Postgres to be ready
echo "Waiting for PostgreSQL to start..."
for i in {1..30}; do
  if su - postgres -c "pg_isready" >/dev/null 2>&1; then
    echo "PostgreSQL is ready."
    break
  fi
  echo "Waiting... $i"
  sleep 1
done

# Setup DB
if ! su - postgres -c "psql -tAc \"SELECT 1 FROM pg_database WHERE datname='jxwxmk'\"" | grep -q 1; then
    echo "Initializing Database..."
    # Set password for postgres user
    su - postgres -c "psql -c \"ALTER USER postgres WITH PASSWORD 'postgres';\""
    su - postgres -c "createdb jxwxmk"
fi

echo "Starting Server..."
./server