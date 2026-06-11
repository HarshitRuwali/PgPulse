# PgPulse

PgPulse monitors PostgreSQL replication from inside PostgreSQL and exposes the result in a way Prometheus and Grafana can use.

It has two main parts:

1. `pgpulse`, a PostgreSQL extension written in Rust with `pgrx`.
2. `pgpulse-exporter`, a Rust HTTP service that reads from the extension and exposes JSON plus Prometheus metrics.

The short version is this:

```text
PostgreSQL primary
  loads pgpulse extension
  runs pgpulse background worker
  stores latest monitoring snapshot in shared memory

pgpulse-exporter
  connects to the primary
  reads pgpulse SQL views and functions
  serves /health, /replication-status, and /metrics

Prometheus
  scrapes /metrics

Grafana
  visualizes the Prometheus data
```

## What PgPulse Watches

PgPulse currently tracks:

- Replication clients from `pg_stat_replication`
- WAL LSN gap in bytes between the primary and each replica
- Replay lag in seconds
- Replica replay delay from `pg_last_xact_replay_timestamp()`
- Long-running active queries
- A simple health status: `Healthy`, `Warning`, or `Critical`

## Who This Is For

Use PgPulse when you want a small Rust-based monitor for PostgreSQL replication lag that can be scraped by Prometheus.

It is useful for:

- Local replication experiments
- Learning how PostgreSQL background workers work
- Building a simple replication dashboard
- Getting a Prometheus-friendly view of lag and health

## Repository Layout

```text
.
├── pgpulse/                 # PostgreSQL extension crate
├── pgpulse-exporter/        # HTTP and Prometheus exporter crate
├── scripts/                 # Docker init and replica startup scripts
├── assets/                  # Screenshots used by the README
├── docker-compose.yml       # Local primary, replica, Prometheus, Grafana
├── Dockerfile               # Builds a PostgreSQL 18 image with pgpulse installed
├── prometheus.yml           # Prometheus scrape config
├── config.yaml              # Exporter config example
└── mkdocs.yml               # Documentation site config
```

## Current PostgreSQL Target

The project defaults to PostgreSQL 18.

The extension crate has feature flags for PostgreSQL 13 through 18, but the Dockerfile and documented install flow use PostgreSQL 18.

## Important Notes

- The PostgreSQL extension must be installed and loaded on the primary.
- The exporter does not collect database metrics by itself. It reads data the extension already collected.
- SSL is not implemented in the exporter yet. Set `primary.ssl_enabled: false`.
- Replica connection details for the extension are currently stored as PostgreSQL settings. Treat them like secrets.
