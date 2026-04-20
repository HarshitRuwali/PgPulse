#!/bin/bash
set -e

# Grant replication privilege
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    ALTER USER pgpulse WITH REPLICATION;
EOSQL

# PGDATA is set by the Docker image entrypoint (e.g. /var/lib/postgresql/18/docker)
echo "host replication pgpulse all scram-sha-256" >> "${PGDATA}/pg_hba.conf"

echo "Replication user configured and pg_hba updated."
