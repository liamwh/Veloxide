pub mod config;
pub mod postgres_db_sqlx;
pub mod tracing_config;

// Re-exports
pub use config::*;
pub use postgres_db_sqlx::*;
pub use tracing::*;

use tracing::{instrument, Level};

#[instrument]
pub async fn load_app_configuration() -> crate::prelude::Result<AppConfiguration> {
    let filename = get_configuration_file_path_variable("CONFIGURATION_FILE_PATH").await?;

    let error_string = format!("Could not open file: {filename}");
    let error_string = error_string.as_str();
    let f = std::fs::File::open(&filename).expect(error_string);

    let application_config: AppConfiguration =
        serde_yaml::from_reader(f).expect("Could not read values.");

    Ok(application_config)
}

#[instrument]
async fn get_database_environment_variable() -> String {
    dotenvy::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:thisisnotsecure@localhost:5432/veloxdb".to_string()
    })
}

#[instrument]
pub async fn get_configuration_file_path_variable(
    variable_name: &str,
) -> crate::prelude::Result<String> {
    let filename = match dotenvy::var(variable_name) {
        Ok(val) => val,
        Err(err) => {
            return Err(crate::error::Error::Generic(format!(
                "Could not read {variable_name}: {err}"
            )));
        }
    };
    Ok(filename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_get_database_environment_variable() {
        env::set_var("DATABASE_URL", "test");
        let db_connection_url = get_database_environment_variable().await;
        assert_eq!(db_connection_url, "test");
    }

    #[tokio::test]
    async fn test_get_configuration_file_path_variable() {
        env::set_var("CONFIGURATION_FILE_PATH", "test");
        let filename = get_configuration_file_path_variable("CONFIGURATION_FILE_PATH")
            .await
            .unwrap();
        assert_eq!(filename, "test");
    }
}
