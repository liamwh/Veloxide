#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(clippy::pedantic)]
#![warn(clippy::all)]

use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use presentation::{todo_handlers, ApiDoc};

use application::todo_service;
use axum::{
    routing::{get, post},
    Extension, Router, Server,
};
use axum_prometheus::PrometheusMetricLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::domain::DynTodoRepo;
use crate::infrastructure::*;
use crate::prelude::*;
use dotenvy::dotenv;
mod application;
mod configuration;
mod domain;
mod error;
mod infrastructure;
mod prelude;
mod presentation;
use tracing_log::LogTracer;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    LogTracer::init()?;
    match configuration::tracing_config::configure_tracing().await {
        Ok(_) => {
            tracing::debug!("tracing subscriber set");
        }
        Err(err) => {
            tracing::error!("error setting tracing subscriber: {}", err);
        }
    };

    let configuration = configuration::load_app_configuration().await?;
    let pool = configuration::get_db_connection_sqlx(&configuration).await?;
    let pool2 = configuration::get_db_connection_sqlx(&configuration).await?;
    let (cqrs, account_query) = presentation::get_cqrs_framework(pool);

    let repository = match configuration.repository {
        configuration::Repository::Postgres => {
            tracing::debug!("using postgres repository");
            Arc::new(PostgresTodoRepository::new(pool2)) as DynTodoRepo
        }
        configuration::Repository::Memory => {
            tracing::debug!("using memory repository");
            Arc::new(MemoryTodoRepository::new()) as DynTodoRepo
        }
    };
    let todo_service = Arc::new(todo_service::TodoServiceImpl::new(repository));

    // Configure prometheus layer
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    if configuration.graphql.enabled {
        tracing::debug!("Starting graphql server");
        tokio::spawn(async move {
            presentation::graphql::run_graphql_server(&configuration.graphql).await;
        });
    }

    // Set up the router
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .route(
            "/bank-accounts/:id",
            get(presentation::bank_account::query_handler)
                .post(presentation::bank_account::command_handler),
        )
        .route(
            "/todo",
            get(todo_handlers::list_todos).post(todo_handlers::post_todo),
        )
        .route(
            "/todo/:id",
            get(todo_handlers::get_todo_by_id).delete(todo_handlers::delete_todo),
        )
        .route(
            "/todo/:id/mark-as-completed",
            post(todo_handlers::mark_as_completed),
        )
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .with_state(todo_service)
        .layer(Extension(cqrs))
        .layer(Extension(account_query))
        .layer(prometheus_layer);

    // Run the router
    let port = dotenvy::var("HTTP_PORT").unwrap_or_else(|_| "4005".to_string());
    let port = port.parse::<u16>()?;
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    Ok(Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}
