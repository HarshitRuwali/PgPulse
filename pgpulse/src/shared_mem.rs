use crate::models::MetricSnapshot;
use pgrx::{pg_shmem_init, prelude::*, PgLwLock};

//  static pgpulse shared Memory for storing the latest collected metrics snapshot which will be read by the exporter
static PGPULSE_METRICS: PgLwLock<MetricSnapshot> = unsafe { PgLwLock::new(c"pgpulse_metrics") };

pub fn init() {
    pg_shmem_init!(PGPULSE_METRICS);
}

pub fn write_snapshot(snapshot: MetricSnapshot) {
    let mut gaurd = PGPULSE_METRICS.exclusive();
    *gaurd = snapshot;
}

pub fn read_snapshot() -> MetricSnapshot {
    let gaurd = PGPULSE_METRICS.share();
    gaurd.clone()
}
