mod bgw;
mod collectors;
mod guc;
mod health;
mod models;
mod shared_mem;
use pgrx::prelude::*;

::pgrx::pg_module_magic!(name, version);

#[pg_guard]
pub extern "C-unwind" fn _PG_init() {
    guc::init();
    shared_mem::init();
    bgw::init();
}

#[pg_extern]
fn hello_pgpulse() -> &'static str {
    "Hello, pgpulse"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_pgpulse() {
        assert_eq!("Hello, pgpulse", crate::hello_pgpulse());
    }
}

/// Return the current replication status
#[pg_extern]
fn pgpulse_replication_status() -> TableIterator<
    'static,
    (
        name!(replica_name, String),
        name!(state, Option<String>),
        name!(replay_lag_seconds, Option<f64>),
        name!(lsn_gap_bytes, Option<i64>),
    ),
> {
    let snapshot = shared_mem::read_snapshot();
    let mut rows = Vec::new();

    for replica in snapshot.primary_metrics.replication_clients {
        rows.push((
            replica.application_name,
            replica.state,
            replica.replay_lag_seconds,
            replica.lsn_gap_bytes,
        ));
    }

    TableIterator::new(rows)
}

/// Return the health status
#[pg_extern]
fn pgpulse_health_status() -> String {
    let snapshot = shared_mem::read_snapshot();
    let health_status = format!("{:?}", snapshot.health_status);
    health_status
}

/// Return the long running queries
#[pg_extern]
fn pgpulse_long_running_queries(
) -> TableIterator<'static, (name!(query, String), name!(duration, f64))> {
    let snapshot = shared_mem::read_snapshot();
    let mut rows = Vec::new();

    for query in snapshot.long_running_queries {
        rows.push((query.query, query.duration));
    }

    TableIterator::new(rows)
}

#[cfg(feature = "pg_bench")]
#[pg_schema]
mod benches {
    use pgrx::prelude::*;
    use pgrx_bench::{black_box, Bencher};

    #[pg_bench]
    fn bench_hello_pgpulse(b: &mut Bencher) {
        b.iter(|| {
            black_box(crate::hello_pgpulse());
        });
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
