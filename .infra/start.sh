#!/bin/bash
set -e

# Start Postgres
# We need to initialize it if not already done. 
# In Docker, usually done by entrypoint. We are making a custom one.
# Default postgres image does complex stuff. Here we are installing postgres on debian.

if [ ! -d "/var/lib/postgresql/data" ]; then
    mkdir -p /var/lib/postgresql/data
    chown -R postgres:postgres /var/lib/postgresql
    su postgres -c "/usr/lib/postgresql/15/bin/initdb -D /var/lib/postgresql/data"
    
    # Configure to listen on localhost
    echo "listen_addresses = '*'" >> /var/lib/postgresql/data/postgresql.conf
    echo "host all all 0.0.0.0/0 md5" >> /var/lib/postgresql/data/pg_hba.conf
fi

# Start Postgres in background
su postgres -c "/usr/lib/postgresql/15/bin/pg_ctl -D /var/lib/postgresql/data start"

# Wait for it
sleep 5

# Create DB and User if not exist
su postgres -c "psql -c \"CREATE USER user WITH PASSWORD 'password';\" || true"
su postgres -c "psql -c \"CREATE DATABASE kkmypk_db OWNER user;\" || true"
su postgres -c "psql -c \"ALTER USER user CREATEDB;\" || true"

# Run App
./kkmypk
