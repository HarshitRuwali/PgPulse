use tokio_postgre::{Client, NoTls};
use crate::config::DbConfig;

pub async fn connect(config: &DbConfig) -> anyhow::Result<Client>{
    let conn_str = format!("host={} port={} name={} user={} password={}",
        config.host, config.port, config.name, config.user, config.password);

    let (client, connection) = tokio_postgres::connect(conn_str.as_str(), NoTls).await?;

    tokio::spawn(async move{
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    Ok(client)
}
