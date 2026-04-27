use crate::config::DbConfig;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use tokio_postgres::Client;

pub async fn connect(config: &DbConfig) -> anyhow::Result<Client> {
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let connector = MakeTlsConnector::new(connector);

    let conn_str = format!(
        "host={} port={} dbname={} user={} password={} sslmode={}",
        config.host,
        config.port,
        config.name,
        config.user,
        config.password,
        if config.ssl_enabled {
            "require"
        } else {
            "disable"
        }
    );

    let (client, connection) = tokio_postgres::connect(conn_str.as_str(), connector).await?;

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
