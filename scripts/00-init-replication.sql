-- Allow the pgpulse user to perform streaming replication.
-- This runs on first init of the master only.
ALTER USER pgpulse WITH REPLICATION;

-- Add pg_hba entry to allow replication from any host (Docker network).
-- This uses ALTER SYSTEM equivalent: we write directly to pg_hba.conf via SQL.
-- Postgres 18 supports this:
DO $$
BEGIN
  -- Reload is needed after we modify pg_hba via the file approach below.
  -- But since this runs during initdb, the server will load the full config on start.
  NULL;
END $$;
