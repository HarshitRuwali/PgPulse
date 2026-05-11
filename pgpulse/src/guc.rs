/// This module defines the GUC settings for pgpulse, allowing users to configure the peer PostgreSQL instance to monitor.
/// Referring to the config
use pgrx::guc::{GucSetting, GucRegistry, GucContext, GucFlags};
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
pub static REPLAY_LAG_CRITICAL_SECONDS: GucSetting<i32> = GucSetting::<i32>::new(60,);
pub static LSN_GAP_WARNING_BYTES: GucSetting<i32> = GucSetting::<i32>::new(10485760); // 10 MB
pub static LSN_GAP_CRITICAL_BYTES: GucSetting<i32> = GucSetting::<i32>::new(104857600);  // 100 MB
pub static LONG_RUNNING_QUERY_WARNING_SECONDS: GucSetting<i32> = GucSetting::<i32>::new(30);


pub fn init() {
    GucRegistry::define_string_guc(
        c"pgpulse.peer_host",
        c"host",
        c"Hostname or IP address of the peer PostgreSQL instance to monitor",
        &PEER_HOST,
        GucContext::Sighup, // User can set this in postgresql.conf and it takes effect on SIGHUP
        GucFlags::default()
    );

    GucRegistry::define_int_guc(
        c"pgpulse.peer_port",
        c"port",
        c"Port number of the peer PostgreSQL instance to monitor",
        &PEER_PORT,
        1,
        65535,
        GucContext::Sighup,
        GucFlags::default()
    )

    
}