#!/usr/bin/env bash
set -euo pipefail

DATA_DIR=/var/lib/postgresql/data
PG_BIN=/usr/lib/postgresql/15/bin

if [ ! -s "$DATA_DIR/PG_VERSION" ]; then
  mkdir -p "$DATA_DIR"
  chown -R postgres:postgres /var/lib/postgresql
  pwfile=$(mktemp)
  echo "postgres" > "$pwfile"
  chown postgres:postgres "$pwfile"
  chmod 600 "$pwfile"
  su - postgres -c "$PG_BIN/initdb -D $DATA_DIR --username=postgres --pwfile=$pwfile" >/dev/null
  rm -f "$pwfile"
fi

su - postgres -c "$PG_BIN/pg_ctl -D $DATA_DIR -o '-c listen_addresses=127.0.0.1' -w start" >/dev/null

su - postgres -c "$PG_BIN/psql -tc \"SELECT 1 FROM pg_database WHERE datname='jxwxmk'\"" | grep -q 1 \
  || su - postgres -c "$PG_BIN/createdb jxwxmk"

exec /app/server
