# Architecture

PgPulse is split into an in-database collector and an external exporter.

That split is important. The PostgreSQL extension can see PostgreSQL internals cheaply. The exporter can speak normal HTTP and Prometheus without running inside the database process.

## Main Components

## PostgreSQL Extension

Location:

```text
pgpulse/
```

The extension is a `cdylib` built with `pgrx`.

When PostgreSQL starts with `shared_preload_libraries = 'pgpulse'`, PostgreSQL loads the extension library and calls `_PG_init()`.

Inside `_PG_init()`, PgPulse:

1. Registers its PostgreSQL settings.
2. Initializes shared memory.
3. Registers a background worker, but only from the postmaster process.

The background worker is named:

```text
postgres monitoring worker
```

## Background Worker

Location:

```text
pgpulse/src/bgw.rs
```

The worker wakes up every `pgpulse.poll_interval_seconds`. The default is `10` seconds.

Each cycle does this:

1. Reads replication clients from `pg_stat_replication`.
2. Reads long-running queries from `pg_stat_activity`.
3. Optionally connects to the replica with raw `libpq`.
4. Calculates the health status.
5. Writes the latest snapshot to PostgreSQL shared memory.

The worker uses SPI for queries against the primary. It uses raw `libpq` for the replica connection because normal Rust PostgreSQL clients are not a good fit inside a PostgreSQL background worker.

## Shared Memory

Location:

```text
pgpulse/src/shared_mem.rs
```

PgPulse stores only the latest snapshot.

That keeps the extension simple:

- One writer: the background worker
- Many readers: SQL functions and views
- No history table
- No write load on user databases

The shared memory value is protected by a PostgreSQL lightweight lock.

## SQL Surface

Location:

```text
pgpulse/sql/pgpulse--0.2.0.sql
```

The Rust functions are exposed through simple SQL views and functions.

User-friendly SQL objects:

```sql
SELECT * FROM pgpulse.replication_status;
SELECT * FROM pgpulse.long_running_queries;
SELECT pgpulse_health_status();
SELECT pgpulse_collected_at();
```

The SQL install file grants public read access to the monitoring views and functions.

## Exporter

Location:

```text
pgpulse-exporter/
```

The exporter is a Rust binary using:

- `axum` for HTTP
- `tokio-postgres` for PostgreSQL access
- `prometheus` for metric registration and text encoding
- `serde_yaml` for config loading
- `clap` for CLI parsing

The exporter connects to the primary database and reads the extension output.

It serves:

- `GET /health`
- `GET /replication-status`
- `GET /metrics`

## Data Flow

```text
Replica
  provides replay timestamp
        ^
        |
Primary PostgreSQL
  pgpulse background worker
  pg_stat_replication
  pg_stat_activity
  shared memory snapshot
        ^
        |
pgpulse SQL views and functions
        ^
        |
pgpulse-exporter
        ^
        |
Prometheus
        ^
        |
Grafana
```

## Health Logic

Location:

```text
pgpulse/src/health/evaluator.rs
```

Health is based on:

- Whether replication clients exist
- LSN gap thresholds
- Replica replay lag thresholds

Default behavior:

- No replication clients means `Warning`
- LSN gap at or above `pgpulse.lsn_gap_warning_bytes` means `Warning`
- LSN gap at or above `pgpulse.lsn_gap_critical_bytes` means `Critical`
- Replica replay lag at or above `pgpulse.replay_lag_warning_seconds` means `Warning`
- Replica replay lag at or above `pgpulse.replay_lag_critical_seconds` means `Critical`

The exporter maps health to numbers for Prometheus:

```text
0 = Healthy
1 = Warning
2 = Critical
```
