# Metrics and APIs

This page lists what PgPulse exposes.

## Prometheus Metrics

The exporter registers these metrics.

| Metric | Type | Labels | Meaning |
| --- | --- | --- | --- |
| `pgpulse_replication_lag_seconds` | Gauge | `replica_name` | Replay lag for each replica in seconds |
| `pgpulse_lsn_gap_bytes` | Gauge | `replica_name` | WAL LSN gap between primary and replica |
| `pgpulse_health_status` | Int gauge | `node` | Health status as a number |
| `pgpulse_long_running_queries` | Int gauge | none | Count of long-running queries above the configured threshold |

Health status values:

```text
0 = Healthy
1 = Warning
2 = Critical
```

## HTTP API

### `GET /health`

Use this to check whether the exporter process is alive.

Response:

```json
{
  "status": "ok",
  "message": "PgPulse is running"
}
```

### `GET /replication-status`

Use this when you want JSON rather than Prometheus text.

Response shape:

```json
{
  "replication_status": [
    {
      "application_name": "walreceiver",
      "replay_lag_seconds": 0.5,
      "lsn_gap_bytes": 0,
      "state": "streaming",
      "replica_replay_lag_seconds": 0.8
    }
  ]
}
```

If the exporter cannot query PostgreSQL, this endpoint returns HTTP `500`.

### `GET /metrics`

Prometheus scrape endpoint.

The exporter reads:

```sql
SELECT application_name, replay_lag_seconds, lsn_gap_bytes
FROM pgpulse.replication_status;
```

It also reads:

```sql
SELECT pgpulse_health_status();
SELECT query FROM pgpulse.long_running_queries;
```

Then it returns Prometheus text format.

## SQL API

The extension exposes the database-level API.

### Replication Status

```sql
SELECT * FROM pgpulse.replication_status;
```

Example columns:

```text
application_name
state
lsn_gap_bytes
replay_lag_seconds
replica_replay_lag_seconds
```

### Long-Running Queries

```sql
SELECT * FROM pgpulse.long_running_queries;
```

Example columns:

```text
query
duration_seconds
```

### Health

```sql
SELECT pgpulse_health_status();
```

Returns one of:

```text
Healthy
Warning
Critical
```

### Last Collection Time

```sql
SELECT pgpulse_collected_at();
```

Returns the Unix timestamp for the latest snapshot.

## Prometheus Config

The repository includes:

```text
prometheus.yml
```

Relevant scrape job:

```yaml
- job_name: "pgpulse"
  metrics_path: /metrics
  static_configs:
    - targets: ["host.docker.internal:8080"]
      labels:
        app: "pgpulse"
```

If the exporter runs somewhere else, update the target.

Examples:

```yaml
targets: ["pgpulse-exporter:8080"]
```

or:

```yaml
targets: ["10.0.0.12:8080"]
```
