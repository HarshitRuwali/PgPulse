# Development

PgPulse is a Rust workspace with two crates.

```text
pgpulse
pgpulse-exporter
```

The workspace root is defined in:

```text
Cargo.toml
```

## Crates

### `pgpulse`

The PostgreSQL extension.

Key files:

| File | Purpose |
| --- | --- |
| `pgpulse/src/lib.rs` | Extension entrypoint and SQL functions |
| `pgpulse/src/bgw.rs` | Background worker registration and polling loop |
| `pgpulse/src/guc.rs` | PostgreSQL settings |
| `pgpulse/src/shared_mem.rs` | Shared memory snapshot storage |
| `pgpulse/src/collectors/replication.rs` | Replication collectors |
| `pgpulse/src/collectors/queries.rs` | Long-running query collector |
| `pgpulse/src/health/evaluator.rs` | Health calculation |
| `pgpulse/sql/pgpulse--0.2.0.sql` | SQL views and grants |

### `pgpulse-exporter`

The HTTP and Prometheus exporter.

Key files:

| File | Purpose |
| --- | --- |
| `pgpulse-exporter/src/main.rs` | CLI, config load, PostgreSQL connection, HTTP server |
| `pgpulse-exporter/src/config.rs` | YAML config types and loader |
| `pgpulse-exporter/src/api/routes.rs` | HTTP handlers |
| `pgpulse-exporter/src/storage/metrics.rs` | Prometheus metric registration |

## Build Everything

```bash
cargo build
```

Build release exporter:

```bash
cargo build --release -p pgpulse-exporter
```

Build extension package:

```bash
cargo pgrx package \
    --features pg18 \
    --pg-config /usr/lib/postgresql/18/bin/pg_config \
    -p pgpulse
```

## Run Tests

Run Rust unit tests:

```bash
cargo test
```

Run extension tests with `pgrx`:

```bash
cargo pgrx test -p pgpulse
```

The extension test setup currently has a simple `hello_pgpulse` test and health evaluator tests.

## Local Development Loop

For exporter work:

```bash
docker compose up -d master_db replica_db
cargo run -p pgpulse-exporter -- --config config.yaml
```

Then test:

```bash
curl http://localhost:8080/metrics
```

For extension work:

1. Change code under `pgpulse/`.
2. Rebuild the package with `cargo pgrx package`.
3. Reinstall `pgpulse.so`, control file, and SQL file.
4. Restart PostgreSQL because the extension is loaded through `shared_preload_libraries`.
5. Run `CREATE EXTENSION pgpulse;` if it is a new database.

## Adding a New Metric

A typical new metric touches both crates.

In the extension:

1. Add data to the model.
2. Collect it in a collector or in the background worker.
3. Store it in the shared memory snapshot.
4. Expose it through a SQL function or view.

In the exporter:

1. Query the SQL object.
2. Register a Prometheus metric.
3. Update the HTTP handler that serves `/metrics`.
4. Add docs for the metric.

## Design Notes

The extension intentionally keeps only the latest snapshot. It does not write a history table.

That keeps the database impact low and leaves time-series storage to Prometheus.

The worker uses `heapless` containers with fixed capacities. Current limits include:

- Up to 16 replication clients
- Up to 16 long-running queries

If you raise those limits, check shared memory size, serialization needs, and exporter behavior.

## Documentation Site

Serve the docs locally:

```bash
mkdocs serve
```

Build static files:

```bash
mkdocs build
```

The generated site appears in:

```text
site/
```
