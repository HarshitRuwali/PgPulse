# Operations

This page covers the day-to-day checks you will usually need after PgPulse is installed.

## Check the Extension

Connect to the primary:

```bash
psql -h localhost -p 5432 -U pgpulse -d pgpulse
```

Check that the extension exists:

```sql
SELECT extname, extversion
FROM pg_extension
WHERE extname = 'pgpulse';
```

Check the latest snapshot:

```sql
SELECT pgpulse_collected_at(), pgpulse_health_status();
```

If `pgpulse_collected_at()` is `0`, wait one poll interval and try again.

## Check Replication

From PostgreSQL:

```sql
SELECT *
FROM pgpulse.replication_status;
```

Useful fields:

- `state` should usually be `streaming`
- `lsn_gap_bytes` should stay near zero on a healthy low-traffic setup
- `replay_lag_seconds` can be `NULL` for async replication
- `replica_replay_lag_seconds` can be `NULL` if replica connection settings are missing or the replica query fails

## Check Long-Running Queries

```sql
SELECT *
FROM pgpulse.long_running_queries;
```

The threshold is controlled by:

```ini
pgpulse.long_running_query_warning_seconds = 30
```

## Check the Exporter

```bash
curl http://localhost:8080/health
curl http://localhost:8080/replication-status
curl http://localhost:8080/metrics
```

If `/health` works but `/metrics` fails, the exporter is running but cannot query PostgreSQL correctly.

## Check Prometheus

Open:

```text
http://localhost:9090
```

Try this query:

```text
pgpulse_health_status
```

If it returns no data:

1. Confirm the exporter is running.
2. Confirm Prometheus can reach the exporter target.
3. Check the Prometheus targets page.

## Check Grafana

Open:

```text
http://localhost:3000
```

Login:

```text
admin / admin
```

Use Prometheus as the data source.

For Docker Compose, Prometheus is reachable from Grafana as:

```text
http://prometheus:9090
```

## Rotate or Change Credentials

There are two places credentials can appear:

1. PostgreSQL settings for the extension replica connection.
2. Exporter YAML config for the primary connection.

After changing extension replica settings, reload PostgreSQL:

```bash
systemctl reload postgresql
```

After changing exporter config, restart the exporter process.

## Cleaning Local Docker State

Stop containers:

```bash
docker compose down
```

Remove local database volumes:

```bash
docker compose down -v
```

Use the volume removal only when you are okay losing local test data.
