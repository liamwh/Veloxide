#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(clippy::pedantic)]
#![warn(clippy::all)]

use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing::get, Extension, Router, Server};
use axum_prometheus::PrometheusMetricLayer;
use presentation::ApiDoc;
use tower::ServiceBuilder;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::prelude::*;
mod error;
use dotenvy::dotenv;
mod application;
mod configuration;
mod domain;
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
    let (cqrs, account_query) = presentation::get_bank_account_cqrs_framework(pool);

    // Start the GraphQL server
    if configuration.graphql.enabled {
        let account_query = account_query.clone();
        tokio::spawn(async move {
            presentation::graphql::run_graphql_server(&configuration.graphql, account_query).await;
        });
    }

    // Configure prometheus layer for Axum
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    // Set up the router
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .route(
            "/bank-accounts/:id",
            get(presentation::bank_account::query_handler)
                .post(presentation::bank_account::command_handler),
        )
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(cqrs))
                .layer(Extension(account_query))
                .layer(prometheus_layer),
        );

    // Run the router
    let port = dotenvy::var("HTTP_PORT").unwrap_or_else(|_| "4005".to_string());
    let port = port.parse::<u16>()?;
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    Ok(Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}
