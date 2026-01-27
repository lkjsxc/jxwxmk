#!/usr/bin/env bash
set -euo pipefail

DATA_DIR=/var/lib/postgresql/data

if [ ! -s "$DATA_DIR/PG_VERSION" ]; then
  mkdir -p "$DATA_DIR"
  chown -R postgres:postgres /var/lib/postgresql
  pwfile=$(mktemp)
  echo "postgres" > "$pwfile"
  su - postgres -c "initdb -D $DATA_DIR --username=postgres --pwfile=$pwfile" >/dev/null
  rm -f "$pwfile"
fi

su - postgres -c "pg_ctl -D $DATA_DIR -o '-c listen_addresses=127.0.0.1' -w start" >/dev/null

su - postgres -c "psql -tc \"SELECT 1 FROM pg_database WHERE datname='jxwxmk'\"" | grep -q 1 \
  || su - postgres -c "createdb jxwxmk"

exec /app/server
