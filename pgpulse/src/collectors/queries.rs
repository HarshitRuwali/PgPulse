// use tokio_postgres::Client;
use pgrx::{
    spi::{Spi, SpiError, SpiTupleTable},
    IntoDatum,
};

use crate::{guc, models::LongRunningQueries};

pub async fn get_long_running_queries() -> Result<Vec<LongRunningQueries>, SpiError> {
    let threshold = guc::LONG_RUNNING_QUERY_WARNING_SECONDS.get() as f64;
    let query = "
        SELECT pid,
            EXTRACT(EPOCH FROM (now() - query_start))::float8 AS duration,
            state,
            left(query, 200) AS query_preview
        FROM pg_stat_activity
        WHERE state != 'idle'
        AND query_start IS NOT NULL
        AND now() - query_start > make_interval(secs => $1)
        ORDER BY duration DESC";

    // client.select uses DatumWithOid for arguments
    // i.e. Oid is a unique 4-byte unsigned integer used internally to identify
    // various database objects like tables, functions, and data types
    Spi::connect(|client| {
        let rows: SpiTupleTable = client.select(query, None, &[threshold.into_datum().into()])?;
        let mut result = Vec::new();
        for row in rows {
            result.push(LongRunningQueries {
                query: row
                    .get_by_name("query_preview")
                    .unwrap_or(Some(String::new()))
                    .unwrap_or_default(),
                duration: row
                    .get_by_name("duration")
                    .unwrap_or(Some(0.0_f64))
                    .unwrap_or(0.0_f64),
            });
        }
        Ok(result)
    })

    // let rows = client
    //     .query(
    //         "SELECT pid,
    //             EXTRACT(EPOCH FROM (now() - query_start))::float8 AS duration,
    //             state,
    //             left(query, 200) AS query_preview
    //      FROM pg_stat_activity
    //      WHERE state != 'idle'
    //        AND query_start IS NOT NULL
    //        AND now() - query_start > make_interval(secs => $1)
    //      ORDER BY duration DESC",
    //         &[&(threshold as f64)],
    //     )
    //     .await?;

    // let mut result = Vec::new();
    // for row in rows {
    //     result.push(LongRunningQueries {
    //         query: row.get("query_preview"),
    //         duration: row.get("duration"),
    //     });
    // }
    // Ok(result)
}
