/// This module defines the GUC settings for pgpulse, allowing users to configure the peer PostgreSQL instance to monitor.
/// Referring to the config
use pgrx::guc::{GucContext, GucFlags, GucRegistry, GucSetting};
// use pgrx::prelude::*;
use std::ffi::CString;

pub static PEER_HOST: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
pub static PEER_PORT: GucSetting<i32> = GucSetting::<i32>::new(5432);
pub static PEER_USER: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
pub static PEER_PASSWORD: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
pub static PEER_DBNAME: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
pub static PEER_SSL_MODE: GucSetting<bool> = GucSetting::<bool>::new(false);
pub static POLL_INTERVAL_SECONDS: GucSetting<i32> = GucSetting::<i32>::new(10);
pub static REPLAY_LAG_WARNING_SECONDS: GucSetting<i32> = GucSetting::<i32>::new(10);
pub static REPLAY_LAG_CRITICAL_SECONDS: GucSetting<i32> = GucSetting::<i32>::new(60);
pub static LSN_GAP_WARNING_BYTES: GucSetting<i32> = GucSetting::<i32>::new(10485760); // 10 MB
pub static LSN_GAP_CRITICAL_BYTES: GucSetting<i32> = GucSetting::<i32>::new(104857600); // 100 MB
pub static LONG_RUNNING_QUERY_WARNING_SECONDS: GucSetting<i32> = GucSetting::<i32>::new(30);

pub fn init() {
    GucRegistry::define_string_guc(
        c"pgpulse.peer_host",
        c"host",
        c"Hostname or IP address of the peer PostgreSQL instance to monitor",
        &PEER_HOST,
        GucContext::Sighup, // User can set this in postgresql.conf and it takes effect on SIGHUP
        GucFlags::default(),
    );

    GucRegistry::define_int_guc(
        c"pgpulse.peer_port",
        c"port",
        c"Port number of the peer PostgreSQL instance to monitor",
        &PEER_PORT,
        1,
        65535,
        GucContext::Sighup,
        GucFlags::default(),
    );

    GucRegistry::define_string_guc(
        c"pgpulse.peer_user",
        c"user",
        c"Username for authenticating to the peer PostgreSQL instance",
        &PEER_USER,
        GucContext::Sighup,
        GucFlags::default(),
    );

    GucRegistry::define_string_guc(
        c"pgpulse.peer_password",
        c"password",
        c"Password for authenticating to the peer PostgreSQL instance",
        &PEER_PASSWORD,
        GucContext::Sighup,
        GucFlags::default(),
    );

    GucRegistry::define_string_guc(
        c"pgpulse.peer_dbname",
        c"dbname",
        c"Database name to connect to on the peer PostgreSQL instance",
        &PEER_DBNAME,
        GucContext::Sighup,
        GucFlags::default(),
    );

    GucRegistry::define_bool_guc(
        c"pgpulse.peer_ssl_mode",
        c"ssl_mode",
        c"Whether to use SSL when connecting to the peer PostgreSQL instance",
        &PEER_SSL_MODE,
        GucContext::Sighup,
        GucFlags::default(),
    );

    GucRegistry::define_int_guc(
        c"pgpulse.poll_interval_seconds",
        c"poll_interval_seconds",
        c"Interval in seconds between polling the peer PostgreSQL instance for status updates",
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
    GucRegistry::define_int_guc(
        c"pgpulse.lsn_gap_warning_bytes",
        c"lsn_gap_warning_bytes",
        c"Threshold in bytes for LSN gap to trigger a warning status",
        &LSN_GAP_WARNING_BYTES,
        1,
        i32::MAX,
        GucContext::Sighup,
        GucFlags::default(),
    );
    GucRegistry::define_int_guc(
        c"pgpulse.lsn_gap_critical_bytes",
        c"lsn_gap_critical_bytes",
        c"Threshold in bytes for LSN gap to trigger a critical status",
        &LSN_GAP_CRITICAL_BYTES,
        1,
        i32::MAX,
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
