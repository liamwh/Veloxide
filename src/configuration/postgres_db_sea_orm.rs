use super::*;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[instrument]
pub async fn get_db_connection_sea_orm(
    app_config: &AppConfiguration,
) -> crate::prelude::Result<DatabaseConnection> {
    tracing::event!(Level::DEBUG, "connecting to db");
    let db_connection_url = get_database_environment_variable().await;

    let mut opt = ConnectOptions::new(db_connection_url.to_owned());
    opt.max_connections(100).min_connections(5);

    let db = Database::connect(opt).await?;
    Migrator::up(&db, None).await?;

    Ok(db)
}
