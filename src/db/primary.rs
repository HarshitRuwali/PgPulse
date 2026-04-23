use crate::config::DbConfig;
use tokio_postgres::{Client, NoTls};

pub async fn connect(config: &DbConfig) -> anyhow::Result<Client> {
    let conn_str = format!(
        "host={} port={} dbname={} user={} password={}",
        config.host, config.port, config.name, config.user, config.password
    );

    let (client, connection) = tokio_postgres::connect(conn_str.as_str(), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    Ok(client)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_replica_connection() -> anyhow::Result<()> {
        let config = crate::config::load_config("config.yaml")?;
        let client = connect(&config.replica).await?;

        let result = client.query_one("SELECT 1 AS test", &[]).await?;
        let val: i32 = result.get("test");
        assert_eq!(val, 1);
        Ok(())
    }
}
