#!/usr/bin/env bash
set -euo pipefail

PGDATA="/var/lib/postgresql/data"
PG_BIN="/usr/lib/postgresql/15/bin"

if [ ! -s "$PGDATA/PG_VERSION" ]; then
  mkdir -p "$PGDATA"
  chown -R postgres:postgres "$PGDATA"
  su - postgres -c "$PG_BIN/initdb -D '$PGDATA'"
fi

su - postgres -c "$PG_BIN/pg_ctl -D '$PGDATA' -o '-c listen_addresses=127.0.0.1' -w start"

su - postgres -c "psql -tc \"SELECT 1 FROM pg_database WHERE datname = 'jxwxmk'\" | grep -q 1 || createdb jxwxmk"

exec /app/jxwxmk
