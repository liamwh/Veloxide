use super::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[instrument]
pub async fn get_db_connection_postgres_sqlx() -> crate::prelude::Result<Pool<Postgres>> {
    let db_connection_url = get_database_environment_variable().await;

    tracing::event!(
        Level::INFO,
        "connecting to postgres db with connection string: {db_connection_url}"
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_connection_url.as_str())
        .await?;

    Ok(pool)
}
