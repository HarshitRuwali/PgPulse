use heapless;
use pgrx::{
    spi::{Spi, SpiError},
    IntoDatum,
};

use crate::{guc, models::LongRunningQuery};

pub fn get_long_running_queries(// ) -> Result<HVec<LongRunningQuery,>, SpiError> {
) -> Result<heapless::Vec<LongRunningQuery, 16>, SpiError> {
    let threshold = guc::LONG_RUNNING_QUERY_WARNING_SECONDS.get() as f64;
    let query = "
        SELECT
            EXTRACT(EPOCH FROM (now() - query_start))::float8 AS duration,
            left(query, 200) AS query_preview
        FROM pg_stat_activity
        WHERE state != 'idle'
          AND query_start IS NOT NULL
          AND EXTRACT(EPOCH FROM (now() - query_start)) > $1
        ORDER BY duration DESC";

    Spi::connect(|client| {
        let rows = client.select(query, None, &[threshold.into_datum().into()])?;
        let mut result: heapless::Vec<LongRunningQuery, 16> = heapless::Vec::new();

        for row in rows {
            if result.is_full() {
                break;
            }
            let q: Option<String> = row.get_by_name("query_preview")?;
            let d: Option<f64> = row.get_by_name("duration")?;
            let s = q.as_deref().unwrap_or("");
            // SQL already truncates to 200 chars; 256-cap string is always large enough
            let lrq = LongRunningQuery {
                query: heapless::String::<256>::try_from(s).unwrap_or_default(),
                duration: d.unwrap_or(0.0),
            };
            let _ = result.push(lrq);
        }
        Ok(result)
    })
}
