use crate::models::{PrimaryMetrics, ReplicationClient};
use chrono::Utc;
use tokio_postgres::Client;

pub async fn collect_primary_metrics(client: &Client) -> anyhow::Result<PrimaryMetrics> {
    let result = client
        .query(
            "SELECT 
            application_name, 
            client_addr::text, 
            state, 
            sync_state,
            sent_lsn::text,
            write_lsn::text,
            flush_lsn::text,
            replay_lsn::text
         FROM pg_stat_replication",
            &[],
        )
        .await?;

    let mut clients = Vec::new();

    for row in result.iter() {
        clients.push(ReplicationClient {
            application_name: row.get("application_name"),
            client_addr: row.get("client_addr"),
            stage: row.get("state"),
            sent_lsn: row.get("sent_lsn"),
            relay_lsn: row.get("replay_lsn"),
        })
    }

    Ok(PrimaryMetrics {
        replication_clients: clients,
        collected_at: Utc::now(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_collect_primary_metrics() -> anyhow::Result<()> {
        let config = crate::config::load_config("config.yaml")?;
        let primary_client = crate::db::primary::connect(&config.primary)
            .await
            .expect("Failed to connect to primary database");
        let metrics = collect_primary_metrics(&primary_client).await?;
        println!("Primary Metrics: {:?}", metrics);
        Ok(())
    }
}
