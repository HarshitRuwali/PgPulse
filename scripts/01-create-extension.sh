#!/bin/bash
# Install the pgpulse extension into the pgpulse database.
# Runs after 00-init-replication.sh during first container initialisation.
set -e

psql -v ON_ERROR_STOP=1 \
    --username "$POSTGRES_USER" \
    --dbname   "$POSTGRES_DB" \
    <<-'EOSQL'
        CREATE EXTENSION IF NOT EXISTS pgpulse;

        -- Schema, views and grants (pgrx auto-generates only C functions in the
        -- extension SQL; these friendly wrappers are added post-install).
        CREATE SCHEMA IF NOT EXISTS pgpulse;

        CREATE OR REPLACE VIEW pgpulse.replication_status AS
            SELECT * FROM pgpulse_replication_status();

        CREATE OR REPLACE VIEW pgpulse.long_running_queries AS
            SELECT * FROM pgpulse_long_running_queries();

        GRANT USAGE ON SCHEMA pgpulse TO PUBLIC;
        GRANT SELECT ON pgpulse.replication_status TO PUBLIC;
        GRANT SELECT ON pgpulse.long_running_queries TO PUBLIC;
        GRANT EXECUTE ON FUNCTION pgpulse_health_status() TO PUBLIC;
        GRANT EXECUTE ON FUNCTION pgpulse_collected_at() TO PUBLIC;
EOSQL

echo "pgpulse extension and views created in database $POSTGRES_DB."
