use std::net::SocketAddr;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};

use serde::{Deserialize, Serialize};
use tracing::instrument;

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

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[instrument(skip(schema, req))]
async fn graphql_handler(
    schema: Extension<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[instrument]
pub async fn run_graphql_server(config: &GraphQlConfiguration) {
    let serve_address = config.parse_serve_address();

    // create the schema
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    Server::bind(&serve_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
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
