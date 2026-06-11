# Quick Start

This guide gets PgPulse running locally with Docker for PostgreSQL, Prometheus, and Grafana, then runs the exporter from your machine.

## Requirements

Install these first:

- Docker with Docker Compose
- Rust and Cargo
- `curl`, for checking endpoints

## Start PostgreSQL, Prometheus, and Grafana

From the repository root:

```bash
docker compose up -d
```

This starts:

- `master_db` on `localhost:5432`
- `replica_db` on `localhost:5433`
- Prometheus on `http://localhost:9090`
- Grafana on `http://localhost:3000`

Grafana uses:

```text
user: admin
password: admin
```

The Docker image for `master_db` is built from the local `Dockerfile`. That image contains the `pgpulse` extension.

## Build the Exporter

```bash
cargo build --release -p pgpulse-exporter
```

The binary is created at:

```text
target/release/pgpulse-exporter
```

## Run the Exporter

The default `config.yaml` points at the local Docker primary.

```bash
./target/release/pgpulse-exporter --config config.yaml
```

You should see logs similar to:

```text
INFO pgpulse_exporter: Starting pgpulse-exporter...
INFO pgpulse_exporter: Config loaded!
INFO pgpulse_exporter: pgpulse-exporter listening on 0.0.0.0:8080
```

## Check the Endpoints

In another terminal:

```bash
curl http://localhost:8080/health
curl http://localhost:8080/replication-status
curl http://localhost:8080/metrics
```

Expected health response:

```json
{"status":"ok","message":"PgPulse is running"}
```

The metrics endpoint should include lines like:

```text
pgpulse_replication_lag_seconds{replica_name="walreceiver"} 0.5
pgpulse_lsn_gap_bytes{replica_name="walreceiver"} 0
pgpulse_health_status{node="primary"} 0
```

## Open Grafana

Open:

```text
http://localhost:3000
```

Add Prometheus as a data source if needed:

```text
http://prometheus:9090
```

For local Docker, `prometheus.yml` scrapes the exporter at:

```text
host.docker.internal:8080
```

That means the exporter should be running on your host machine on port `8080`.

## Stop the Local Stack

```bash
docker compose down
```

To remove volumes too:

```bash
docker compose down -v
```
