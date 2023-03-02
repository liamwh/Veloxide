pub mod config;
pub mod postgres_db_sqlx;

// Re-exports
pub use config::*;
pub use postgres_db_sqlx::*;

use tracing::{instrument, Level};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

#[instrument]
pub async fn configure_tracing() -> std::result::Result<(), crate::error::Error> {
    // Configure the OpenTelemetry tracer
    opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    // Get the service name from the environment
    let tracing_service_name =
        dotenvy::var("TRACING_SERVICE_NAME").unwrap_or_else(|_| "velox-todo-app".to_string());

    // Create a new Jaeger tracer
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(tracing_service_name)
        .install_simple()
        .expect("Expected Jaeger tracer to install successfully, regardless of whether Jaeger is running or not");

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = tracing_subscriber::Registry::default().with(telemetry);
    Ok(tracing::subscriber::set_global_default(subscriber)?)
}

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
        "postgres://postgres:thisisnotsecure@localhost:5432/veloxtodoapi".to_string()
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
