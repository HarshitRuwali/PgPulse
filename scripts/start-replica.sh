#!/bin/bash
set -e

# Postgres 18 Docker image uses /var/lib/postgresql/<major>/docker as PGDATA.
PG_MAJOR=$(postgres --version | grep -oP '\d+' | head -1)
PGDATA="/var/lib/postgresql/${PG_MAJOR}/docker"

echo "Replica: PGDATA=${PGDATA}"
echo "Replica: waiting for master to be ready..."

# If PGDATA already has data from a previous run, just start Postgres.
if [ -s "${PGDATA}/PG_VERSION" ]; then
    echo "Replica: data directory already exists, starting Postgres..."
    exec gosu postgres postgres -D "${PGDATA}"
fi

echo "Replica: taking base backup from master..."

# pg_basebackup clones the master's data directory.
# -R writes standby.signal and primary_conninfo into the data dir.
# -X stream ensures WAL is streamed during backup (no missing segments).
pg_basebackup \
    -h master_db \
    -p 5432 \
    -U pgpulse \
    -D "${PGDATA}" \
    -Fp -Xs -P -R

# Ensure the postgres user owns the entire postgresql directory tree
chown -R postgres:postgres /var/lib/postgresql

echo "Replica: base backup complete. Starting Postgres in standby mode..."
exec gosu postgres postgres -D "${PGDATA}"
