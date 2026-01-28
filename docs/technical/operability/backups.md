# Backups + recovery

The runtime uses a persistent Postgres data directory (Docker volume or bind mount).

## Backup goals

- Prevent total loss of persistent world/player state.
- Enable recovery after container/host failure.

## Minimal backup procedure

One of:

- Offline backup of the Postgres data volume (stop container, snapshot volume), or
- `pg_dump`-based logical backups (scheduled).

Backups must be stored outside the container host volume.

## Recovery

- Restore the volume or import the dump.
- Start the container; migrations should apply cleanly.

## Safety notes

- Do not expose Postgres externally.
- Keep `DATABASE_URL` and tokens out of logs.
