use std::{net::SocketAddr, sync::Arc};

use async_graphql::{http::*, *};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};

use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::domain::BankAccount;

use super::{BankAccountGraphQlMutation, BankAccountGraphQlQuery, BankAccountView};

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "postgres")] {
        use postgres_es::{PostgresCqrs, PostgresViewRepository};
    } else if #[cfg(feature = "mysql")] {
        use mysql_es::{MysqlCqrs, MysqlViewRepository};
    } else {
        compile_error!("Must specify either mysql or postgres feature");
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GraphQlConfiguration {
    pub enabled: bool,
    pub port: u16,
}

impl GraphQlConfiguration {
    pub fn parse_serve_address(&self) -> SocketAddr {
        let serve_address_str = format!("127.0.0.1:{}", self.port);
        tracing::debug!("Parsing serve address: {}", serve_address_str);

        serve_address_str
            .parse::<SocketAddr>()
            .expect("Expected to be able to parse the serve_address to a SocketAddr")
    }
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[instrument(skip(schema, req))]
async fn graphql_handler(
    schema: Extension<Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[derive(MergedObject, Default)]
struct QueryRoot(BankAccountGraphQlQuery);

#[derive(MergedObject, Default)]
struct MutationRoot(BankAccountGraphQlMutation);

cfg_if! {
    if #[cfg(feature = "postgres")] {
        #[instrument(skip(bank_account_view_repsitory, bank_account_cqrs_framework))]
pub async fn run_graphql_server(
    config: &GraphQlConfiguration,
    bank_account_cqrs_framework: Arc<PostgresCqrs<BankAccount>>,
    bank_account_view_repsitory: Arc<PostgresViewRepository<BankAccountView, BankAccount>>,
) {
    tracing::debug!("Starting graphql server");
    let serve_address = config.parse_serve_address();

    // create the schema
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(bank_account_view_repsitory)
    .data(bank_account_cqrs_framework)
    .finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

    Server::bind(&serve_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
    } else if #[cfg(feature = "mysql")] {
        #[instrument(skip(bank_account_view_repsitory, bank_account_cqrs_framework))]
pub async fn run_graphql_server(
    config: &GraphQlConfiguration,
    bank_account_cqrs_framework: Arc<MysqlCqrs<BankAccount>>,
    bank_account_view_repsitory: Arc<MysqlViewRepository<BankAccountView, BankAccount>>,
) {
    tracing::debug!("Starting graphql server");
    let serve_address = config.parse_serve_address();

    // create the schema
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(bank_account_view_repsitory)
    .data(bank_account_cqrs_framework)
    .finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

    Server::bind(&serve_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
    } else {
        compile_error!("Must specify either mysql or postgres feature");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn parse_serve_address_parses_correctly() {
        let config = GraphQlConfiguration {
            enabled: true,
            port: 8080,
        };
        let serve_address = config.parse_serve_address();
        assert_eq!(
            serve_address,
            "127.0.0.1:8080".parse::<SocketAddr>().unwrap()
        );
    }
}
