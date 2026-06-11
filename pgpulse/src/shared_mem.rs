use crate::models::MetricSnapshot;
use pgrx::{pg_shmem_init, prelude::*, PgLwLock};

//  static pgpulse shared Memory for storing the latest collected metrics snapshot which will be read by the exporter
static PGPULSE_METRICS: PgLwLock<MetricSnapshot> = unsafe { PgLwLock::new(c"pgpulse_metrics") };

pub fn init() {
    pg_shmem_init!(PGPULSE_METRICS);
}

pub fn write_snapshot(snapshot: MetricSnapshot) {
    let mut guard = PGPULSE_METRICS.exclusive();
    *guard = snapshot;
}

pub fn read_snapshot() -> MetricSnapshot {
    let guard = PGPULSE_METRICS.share();
    guard.clone()
}
