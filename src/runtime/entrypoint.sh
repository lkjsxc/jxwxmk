#!/usr/bin/env bash
set -euo pipefail

PGDATA="/var/lib/postgresql/data"

init_db() {
  mkdir -p "$PGDATA"
  chown -R postgres:postgres /var/lib/postgresql
  su postgres -c "initdb -D '$PGDATA'"
  su postgres -c "pg_ctl -D '$PGDATA' -o \"-c listen_addresses='127.0.0.1'\" -w start"
  su postgres -c "psql -c \"ALTER USER postgres PASSWORD 'postgres';\""
  su postgres -c "psql -c \"CREATE DATABASE kkmypk;\""
  su postgres -c "pg_ctl -D '$PGDATA' -m fast -w stop"
}

if [ ! -s "$PGDATA/PG_VERSION" ]; then
  init_db
fi

su postgres -c "pg_ctl -D '$PGDATA' -o \"-c listen_addresses='127.0.0.1'\" -w start"

exec /app/kkmypk-server
