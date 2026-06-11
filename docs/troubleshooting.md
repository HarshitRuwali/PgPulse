# Troubleshooting

This page lists common issues and how to confirm them.

## `connection refused` on PostgreSQL

Likely cause:

PostgreSQL is not running, or the port is wrong.

Check Docker:

```bash
docker compose ps
```

Expected local ports:

```text
primary: localhost:5432
replica: localhost:5433
```

## Exporter Says Authentication Failed

Likely cause:

The credentials in `config.yaml` do not match PostgreSQL.

Check the local Docker credentials:

```yaml
primary:
  user: pgpulse
  password: pgpulse
  name: pgpulse
```

Then test with `psql`:

```bash
psql -h localhost -p 5432 -U pgpulse -d pgpulse
```

## `relation "pgpulse.replication_status" does not exist`

Likely cause:

The extension was not created in the database used by the exporter.

Fix:

```sql
CREATE EXTENSION pgpulse;
```

Then check:

```sql
SELECT * FROM pgpulse.replication_status;
```

## `pgpulse_collected_at()` Returns `0`

Likely causes:

- The background worker has not run yet.
- `shared_preload_libraries` does not include `pgpulse`.
- PostgreSQL needs a restart.

Fix:

1. Wait 10 to 15 seconds.
2. Check `shared_preload_libraries`.
3. Restart PostgreSQL.
4. Check PostgreSQL logs for `pgpulse: background worker started`.

## Health Is Always `Warning`

Likely causes:

- No replicas are connected.
- LSN gap is above the warning threshold.
- Replica lag is above the warning threshold.

Check:

```sql
SELECT * FROM pgpulse.replication_status;
SELECT pgpulse_health_status();
```

If no rows return from `pgpulse.replication_status`, PostgreSQL does not currently see a connected replication client.

## `replica_replay_lag_seconds` Is `NULL`

Likely causes:

- `pgpulse.replica_host` is not set.
- Replica user, password, dbname, or host is wrong.
- The replica is not reachable from the primary.
- The replica query failed.

Check the extension settings:

```sql
SHOW pgpulse.replica_host;
SHOW pgpulse.replica_port;
SHOW pgpulse.replica_user;
SHOW pgpulse.replica_dbname;
```

Check network access from the primary host to the replica.

## `/health` Works but `/metrics` Fails

Likely cause:

The exporter process is alive, but the PostgreSQL query failed.

Check:

```bash
curl http://localhost:8080/replication-status
```

Then connect with the same config values and run:

```sql
SELECT * FROM pgpulse.replication_status;
SELECT pgpulse_health_status();
```

## Prometheus Has No PgPulse Data

Likely causes:

- Exporter is not running.
- Prometheus target points to the wrong host or port.
- Docker cannot reach the host exporter.

Check Prometheus targets:

```text
http://localhost:9090/targets
```

For the included Docker setup, the target is:

```text
host.docker.internal:8080
```

If your platform does not support `host.docker.internal`, replace it with a reachable host IP or run the exporter in Docker.

## Exporter Exits When SSL Is Enabled

Current behavior:

The exporter exits if `primary.ssl_enabled` is `true`.

Fix:

```yaml
primary:
  ssl_enabled: false
```

SSL support is not implemented in the exporter yet.

## Docker Compose Grafana File Fails

Use the main file first:

```bash
docker compose up -d
```

The main compose file includes Grafana and Prometheus. The separate `docker-compose-grafana.yml` should be reviewed before relying on it because its YAML indentation does not currently match the main compose file style.
