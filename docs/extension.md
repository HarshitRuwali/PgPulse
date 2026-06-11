# PostgreSQL Extension

The `pgpulse` extension runs inside PostgreSQL. It is responsible for collecting the monitoring snapshot.

## What It Installs

The extension installs:

- A PostgreSQL shared library: `pgpulse.so`
- A control file: `pgpulse.control`
- SQL install file: `pgpulse--0.2.0.sql`
- SQL views and functions for monitoring data
- A background worker registered at server startup

## Build Requirements

For PostgreSQL 18 on Debian or Ubuntu, install:

- PostgreSQL 18 server
- PostgreSQL 18 server development headers
- Rust and Cargo
- `cargo-pgrx` version `0.18.0`
- `libclang-dev`
- `clang`
- `pkg-config`
- `libssl-dev`

Example:

```bash
apt-get update
apt-get install -y postgresql-18 postgresql-server-dev-18 \
    libclang-dev clang pkg-config libssl-dev
```

Install the exact `cargo-pgrx` version:

```bash
cargo install cargo-pgrx --version "=0.18.0" --locked
```

Initialize `pgrx` against PostgreSQL 18:

```bash
cargo pgrx init --pg18 /usr/lib/postgresql/18/bin/pg_config
```

## Build the Package

```bash
cargo pgrx package \
    --features pg18 \
    --pg-config /usr/lib/postgresql/18/bin/pg_config \
    -p pgpulse
```

The output lands under:

```text
target/release/pgpulse-pg18/
```

Expected files:

```text
target/release/pgpulse-pg18/usr/lib/postgresql/18/lib/pgpulse.so
target/release/pgpulse-pg18/usr/share/postgresql/18/extension/pgpulse.control
target/release/pgpulse-pg18/usr/share/postgresql/18/extension/pgpulse--0.2.0.sql
```

## Install the Files

```bash
cp target/release/pgpulse-pg18/usr/lib/postgresql/18/lib/pgpulse.so \
    /usr/lib/postgresql/18/lib/

cp target/release/pgpulse-pg18/usr/share/postgresql/18/extension/pgpulse.control \
   target/release/pgpulse-pg18/usr/share/postgresql/18/extension/pgpulse--0.2.0.sql \
    /usr/share/postgresql/18/extension/
```

## Configure PostgreSQL

Add this to `postgresql.conf`:

```ini
shared_preload_libraries = 'pgpulse'

pgpulse.replica_host = 'replica-db-host'
pgpulse.replica_port = 5432
pgpulse.replica_user = 'pgpulse'
pgpulse.replica_password = 'pgpulse'
pgpulse.replica_dbname = 'postgres'
```

Restart PostgreSQL:

```bash
systemctl restart postgresql
```

Or:

```bash
pg_ctlcluster 18 main restart
```

## Create the Extension

Connect to the database you want the exporter to read from:

```bash
psql -d postgres
```

Then run:

```sql
CREATE EXTENSION pgpulse;
```

## Verify It Works

Wait around 10 seconds for the first polling cycle, then run:

```sql
SELECT pgpulse_collected_at();
SELECT pgpulse_health_status();
SELECT * FROM pgpulse.replication_status;
SELECT * FROM pgpulse.long_running_queries;
```

`pgpulse_collected_at()` returns a Unix timestamp. If it returns `0`, the background worker has not written its first snapshot yet.

## SQL Objects

### `pgpulse.replication_status`

Returns one row per replication client known to `pg_stat_replication`.

Columns:

| Column | Type | Meaning |
| --- | --- | --- |
| `application_name` | `text` | Replica application name |
| `state` | `text` | Replication state |
| `lsn_gap_bytes` | `bigint` | Difference between current WAL LSN and replay, flush, or write LSN |
| `replay_lag_seconds` | `double precision` | Replay lag from `pg_stat_replication` |
| `replica_replay_lag_seconds` | `double precision` | Replica-side lag from `pg_last_xact_replay_timestamp()` |

### `pgpulse.long_running_queries`

Returns active queries whose duration is above `pgpulse.long_running_query_warning_seconds`.

Columns:

| Column | Type | Meaning |
| --- | --- | --- |
| `query` | `text` | Query preview, truncated by the collector |
| `duration_seconds` | `double precision` | How long the query has been running |

### `pgpulse_health_status()`

Returns:

```text
Healthy
Warning
Critical
```

### `pgpulse_collected_at()`

Returns the Unix timestamp of the last collected snapshot.

## Permissions

The extension SQL grants public read access to:

- Schema usage on `pgpulse`
- `SELECT` on `pgpulse.replication_status`
- `SELECT` on `pgpulse.long_running_queries`
- `EXECUTE` on `pgpulse_health_status()`
- `EXECUTE` on `pgpulse_collected_at()`
