#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(clippy::pedantic)]
#![warn(clippy::all)]
#![cfg_attr(coverage_nightly, feature(no_coverage))]

use std::net::{Ipv4Addr, SocketAddr};

use axum::{
    headers::{self, Header},
    routing::get,
    Extension, Router, Server,
};
use axum_prometheus::PrometheusMetricLayer;
use hyper::{header::CONTENT_TYPE, Method};
use presentation::ApiDoc;

use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::prelude::*;
mod error;
use cfg_if::cfg_if;
use dotenvy::dotenv;
mod application;
mod configuration;
mod domain;
mod prelude;
mod presentation;
use tracing_log::LogTracer;

cfg_if! {
    if #[cfg(feature = "postgres")] {
        use sqlx::{Pool, Postgres};
        async fn get_db_connection(app_config: &configuration::AppConfiguration) -> crate::prelude::Result<Pool<Postgres>> {
            configuration::get_db_connection_postgres_sqlx(&app_config).await
        }
    } else if #[cfg(feature = "mysql")] {
        use sqlx::{Pool, mysql};
        async fn get_db_connection(app_config: &configuration::AppConfiguration) -> crate::prelude::Result<Pool<mysql::MySql>> {
            configuration::get_db_connection_mysql_sqlx(&app_config).await
        }
    } else {
        compile_error!("Must specify either mysql or postgres feature");
    }
}

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
    let pool = get_db_connection(&configuration).await?;
    let (cqrs, account_query) = presentation::get_bank_account_cqrs_framework(pool);

    // Start the GraphQL server
    if configuration.graphql.enabled {
        let cqrs = cqrs.clone();
        let account_query = account_query.clone();
        tokio::spawn(async move {
            presentation::graphql::run_graphql_server(&configuration.graphql, cqrs, account_query)
                .await;
        });
    }

    // Configure prometheus layer for Axum
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    // Configure CORS
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // Allow headers
        .allow_headers([CONTENT_TYPE])
        // allow requests from any origin TODO: Make me more secure
        .allow_origin(Any);

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
                .layer(prometheus_layer)
                .layer(cors),
        );

    // Run the router
    let port = dotenvy::var("HTTP_PORT").unwrap_or_else(|_| "4005".to_string());
    let port = port.parse::<u16>()?;
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    Ok(Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}
