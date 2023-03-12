use super::*;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

#[instrument]
pub async fn get_db_connection_mysql_sqlx() -> crate::prelude::Result<Pool<MySql>> {
    let db_connection_url = get_database_environment_variable().await;

    tracing::event!(
        Level::INFO,
        "connecting to mysql db with connection string: {db_connection_url}"
    );
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(db_connection_url.as_str())
        .await?;

    Ok(pool)
}
