use tokio_postgres::Client;

use crate::models::LongRunningQueries;

pub async fn get_long_running_queries(
    client: &Client,
    threshold: u64,
) -> Result<Vec<LongRunningQueries>, tokio_postgres::Error> {
    let rows = client
        .query(
            "SELECT pid,
                EXTRACT(EPOCH FROM (now() - query_start))::float8 AS duration,
                state,
                left(query, 200) AS query_preview
         FROM pg_stat_activity
         WHERE state != 'idle'
           AND query_start IS NOT NULL
           AND now() - query_start > make_interval(secs => $1)
         ORDER BY duration DESC",
            &[&(threshold as f64)],
        )
        .await?;

    let mut result = Vec::new();
    for row in rows {
        result.push(LongRunningQueries {
            query: row.get("query_preview"),
            duration: row.get("duration"),
        });
    }
    Ok(result)
}
