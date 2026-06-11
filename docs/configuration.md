# Configuration

PgPulse has two configuration surfaces:

1. PostgreSQL settings for the extension.
2. YAML config for the exporter.

The extension settings control what the background worker collects. The exporter config controls how the HTTP service connects to PostgreSQL and where it listens.

## Extension Settings

Set these in `postgresql.conf`, through Docker command arguments, or through your PostgreSQL configuration management.

Most settings use `SIGHUP` context, so they can be changed by reloading PostgreSQL. The extension still needs `shared_preload_libraries`, which requires a restart.

### Required Startup Setting

```ini
shared_preload_libraries = 'pgpulse'
```

This must be set before PostgreSQL starts. Without it, the background worker will not run.

### Replica Connection Settings

```ini
pgpulse.replica_host = 'replica-db-host'
pgpulse.replica_port = 5432
pgpulse.replica_user = 'pgpulse'
pgpulse.replica_password = 'pgpulse'
pgpulse.replica_dbname = 'postgres'
pgpulse.replica_ssl_mode = off
```

These settings let the background worker connect to a replica and run:

```sql
SELECT EXTRACT(EPOCH FROM (now() - pg_last_xact_replay_timestamp()))::float8
```

If these values are missing, PgPulse can still report primary-side replication information from `pg_stat_replication`, but replica-side replay lag can be `NULL`.

### Polling

```ini
pgpulse.poll_interval_seconds = 10
```

This controls how often the background worker refreshes the shared memory snapshot.

### Health Thresholds

```ini
pgpulse.replay_lag_warning_seconds = 10
pgpulse.replay_lag_critical_seconds = 60
pgpulse.lsn_gap_warning_bytes = 10485760
pgpulse.lsn_gap_critical_bytes = 104857600
pgpulse.long_running_query_warning_seconds = 30
```

Defaults:

| Setting | Default | Meaning |
| --- | ---: | --- |
| `pgpulse.replay_lag_warning_seconds` | `10` | Replay lag that marks health as warning |
| `pgpulse.replay_lag_critical_seconds` | `60` | Replay lag that marks health as critical |
| `pgpulse.lsn_gap_warning_bytes` | `10485760` | 10 MB WAL gap warning threshold |
| `pgpulse.lsn_gap_critical_bytes` | `104857600` | 100 MB WAL gap critical threshold |
| `pgpulse.long_running_query_warning_seconds` | `30` | Query duration threshold for long-running query reporting |

## Reloading Extension Settings

After changing `pgpulse.*` settings:

```bash
pg_ctl reload
```

Or with systemd:

```bash
systemctl reload postgresql
```

If you changed `shared_preload_libraries`, restart PostgreSQL:

```bash
systemctl restart postgresql
```

## Exporter Config

The exporter reads YAML. By default it loads:

```text
config.yaml
```

Example:

```yaml
primary:
  host: localhost
  port: 5432
  name: pgpulse
  user: pgpulse
  password: pgpulse
  ssl_enabled: false

server:
  host: "0.0.0.0"
  port: 8080
```

Run with a custom config:

```bash
pgpulse-exporter --config /etc/pgpulse/config.yaml
```

### Exporter Fields

| Field | Required | Meaning |
| --- | --- | --- |
| `primary.host` | Yes | Hostname or IP address of the PostgreSQL primary |
| `primary.port` | Yes | PostgreSQL port |
| `primary.name` | Yes | Database where `CREATE EXTENSION pgpulse;` was run |
| `primary.user` | Yes | Database user used by the exporter |
| `primary.password` | Yes | Database password |
| `primary.ssl_enabled` | Yes | Must currently be `false` |
| `server.host` | Yes | Address the exporter binds to |
| `server.port` | Yes | Port the exporter listens on |

## Security Notes

Current limitations to keep in mind:

- The extension stores replica connection details in PostgreSQL settings.
- The exporter config stores the primary password in YAML.
- The exporter does not support SSL yet.

For production-style usage, protect these files and settings with normal secret-handling practices.
