-- Dedicated schema for pgpulse
CREATE SCHEMA IF NOT EXISTS pgpulse;

-- Friendly views wrap the internal pg_extern functions
CREATE OR REPLACE VIEW pgpulse.replication_status AS
    SELECT * FROM pgpulse_replication_status();

CREATE OR REPLACE VIEW pgpulse.long_running_queries AS
    SELECT * FROM pgpulse_long_running_queries();

-- Allow any role with CONNECT to read monitoring data
GRANT USAGE ON SCHEMA pgpulse TO PUBLIC;
GRANT SELECT ON pgpulse.replication_status TO PUBLIC;
GRANT SELECT ON pgpulse.long_running_queries TO PUBLIC;
GRANT EXECUTE ON FUNCTION pgpulse_health_status() TO PUBLIC;
GRANT EXECUTE ON FUNCTION pgpulse_collected_at() TO PUBLIC;
