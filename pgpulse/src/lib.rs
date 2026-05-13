mod bgw;
mod collectors;
mod guc;
mod health;
mod models;
use pgrx::prelude::*;

::pgrx::pg_module_magic!(name, version);

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
