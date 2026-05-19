/// This module defines the GUC settings for pgpulse, allowing users to configure the peer PostgreSQL instance to monitor.
/// Referring to the config
use pgrx::guc::{GucContext, GucFlags, GucRegistry, GucSetting};
// use pgrx::prelude::*;
use std::ffi::CString;

pub static REPLICA_HOST: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None); // fn new expects Option<&'static CStr>
pub static REPLICA_PORT: GucSetting<i32> = GucSetting::<i32>::new(5432);
pub static REPLICA_USER: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
pub static REPLICA_PASSWORD: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
pub static REPLICA_DBNAME: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
pub static REPLICA_SSL_MODE: GucSetting<bool> = GucSetting::<bool>::new(false);
pub static POLL_INTERVAL_SECONDS: GucSetting<i32> = GucSetting::<i32>::new(10);
pub static REPLAY_LAG_WARNING_SECONDS: GucSetting<i32> = GucSetting::<i32>::new(10);
pub static REPLAY_LAG_CRITICAL_SECONDS: GucSetting<i32> = GucSetting::<i32>::new(60);
pub static LSN_GAP_WARNING_BYTES: GucSetting<f64> = GucSetting::<f64>::new(10_485_760.0); // default 10 MB; using f64 to avoid overflow in case of large gaps
pub static LSN_GAP_CRITICAL_BYTES: GucSetting<f64> = GucSetting::<f64>::new(104_857_600.0); // default 100 MB
pub static LONG_RUNNING_QUERY_WARNING_SECONDS: GucSetting<i32> = GucSetting::<i32>::new(30);

pub fn init() {
    GucRegistry::define_string_guc(
        c"pgpulse.replica_host",
        c"host",
        c"Hostname or IP address of the replica PostgreSQL instance to monitor",
        &REPLICA_HOST,
        GucContext::Sighup, // User can set this in postgresql.conf and it takes effect on SIGHUP
        GucFlags::default(),
    );

    GucRegistry::define_int_guc(
        c"pgpulse.replica_port",
        c"port",
        c"Port number of the replica PostgreSQL instance to monitor",
        &REPLICA_PORT,
        1,
        65535,
        GucContext::Sighup,
        GucFlags::default(),
    );

    GucRegistry::define_string_guc(
        c"pgpulse.replica_user",
        c"user",
        c"Username for authenticating to the replica PostgreSQL instance",
        &REPLICA_USER,
        GucContext::Sighup,
        GucFlags::default(),
    );

    GucRegistry::define_string_guc(
        c"pgpulse.replica_password",
        c"password",
        c"Password for authenticating to the replica PostgreSQL instance",
        &REPLICA_PASSWORD,
        GucContext::Sighup,
        GucFlags::default(),
    );

    GucRegistry::define_string_guc(
        c"pgpulse.replica_dbname",
        c"dbname",
        c"Database name to connect to on the replica PostgreSQL instance",
        &REPLICA_DBNAME,
        GucContext::Sighup,
        GucFlags::default(),
    );

    GucRegistry::define_bool_guc(
        c"pgpulse.replica_ssl_mode",
        c"ssl_mode",
        c"Whether to use SSL when connecting to the replica PostgreSQL instance",
        &REPLICA_SSL_MODE,
        GucContext::Sighup,
        GucFlags::default(),
    );

    GucRegistry::define_int_guc(
        c"pgpulse.poll_interval_seconds",
        c"poll_interval_seconds",
        c"Interval in seconds between polling the replica PostgreSQL instance for status updates",
        &POLL_INTERVAL_SECONDS,
        1,
        i32::MAX,
        GucContext::Sighup,
        GucFlags::default(),
    );
    GucRegistry::define_int_guc(
        c"pgpulse.replay_lag_warning_seconds",
        c"replay_lag_warning_seconds",
        c"Threshold in seconds for replay lag to trigger a warning status",
        &REPLAY_LAG_WARNING_SECONDS,
        1,
        i32::MAX,
        GucContext::Sighup,
        GucFlags::default(),
    );
    GucRegistry::define_int_guc(
        c"pgpulse.replay_lag_critical_seconds",
        c"replay_lag_critical_seconds",
        c"Threshold in seconds for replay lag to trigger a critical status",
        &REPLAY_LAG_CRITICAL_SECONDS,
        1,
        i32::MAX,
        GucContext::Sighup,
        GucFlags::default(),
    );
    GucRegistry::define_float_guc(
        c"pgpulse.lsn_gap_warning_bytes",
        c"lsn_gap_warning_bytes",
        c"Threshold in bytes for LSN gap to trigger a warning status",
        &LSN_GAP_WARNING_BYTES,
        1.0,
        f64::MAX,
        GucContext::Sighup,
        GucFlags::default(),
    );
    GucRegistry::define_float_guc(
        c"pgpulse.lsn_gap_critical_bytes",
        c"lsn_gap_critical_bytes",
        c"Threshold in bytes for LSN gap to trigger a critical status",
        &LSN_GAP_CRITICAL_BYTES,
        1.0,
        f64::MAX,
        GucContext::Sighup,
        GucFlags::default(),
    );
    GucRegistry::define_int_guc(
        c"pgpulse.long_running_query_warning_seconds",
        c"long_running_query_warning_seconds",
        c"Threshold in seconds for long-running queries to trigger a warning status",
        &LONG_RUNNING_QUERY_WARNING_SECONDS,
        1,
        i32::MAX,
        GucContext::Sighup,
        GucFlags::default(),
    );
}
