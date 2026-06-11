# Exporter

The exporter is the HTTP side of PgPulse.

It connects to PostgreSQL, reads the `pgpulse` extension output, and serves that data for people and monitoring tools.

## Build

```bash
cargo build --release -p pgpulse-exporter
```

The binary is created at:

```text
target/release/pgpulse-exporter
```

## Run

With the default config:

```bash
./target/release/pgpulse-exporter --config config.yaml
```

With a server config:

```bash
pgpulse-exporter --config /etc/pgpulse/config.yaml
```

The exporter logs where it is listening:

```text
pgpulse-exporter listening on 0.0.0.0:8080
```

## Required Database State

The target database must have the extension installed:

```sql
CREATE EXTENSION pgpulse;
```

The configured user must be able to run:

```sql
SELECT * FROM pgpulse.replication_status;
SELECT * FROM pgpulse.long_running_queries;
SELECT pgpulse_health_status();
```

The extension install file grants these reads to `PUBLIC`, so the default setup works without extra grants.

## Config

The exporter config is YAML:

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

`ssl_enabled` must currently be `false`. If it is `true`, the exporter exits during startup.

## Endpoints

### `GET /health`

Simple process health check.

Example:

```bash
curl http://localhost:8080/health
```

Response:

```json
{"status":"ok","message":"PgPulse is running"}
```

This endpoint only says the exporter process is running. It does not prove PostgreSQL is healthy.

### `GET /replication-status`

JSON view of `pgpulse.replication_status`.

Example:

```bash
curl http://localhost:8080/replication-status
```

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

### `GET /metrics`

Prometheus text output.

Example:

```bash
curl http://localhost:8080/metrics
```

This endpoint queries PostgreSQL when Prometheus scrapes it, updates the registered metrics, and returns Prometheus text format.

## Running Against Docker PostgreSQL

Start only the database services:

```bash
docker compose up -d master_db replica_db
```

Build and run the exporter locally:

```bash
cargo build --release -p pgpulse-exporter
./target/release/pgpulse-exporter --config config.yaml
```

Prometheus inside Docker scrapes:

```text
host.docker.internal:8080
```

So the exporter should be reachable from the Docker network through the host.

## Running on a Server

Copy the binary:

```bash
scp target/release/pgpulse-exporter user@your-server:/usr/local/bin/pgpulse-exporter
```

Create:

```text
/etc/pgpulse/config.yaml
```

Run:

```bash
pgpulse-exporter --config /etc/pgpulse/config.yaml
```

For production use, run it under your service manager, such as systemd.
