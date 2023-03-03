use std::{net::SocketAddr, sync::Arc};

use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, FieldResult, MergedObject,
    Object, Schema, SimpleObject,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};

use cqrs_es::persist::ViewRepository;
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::domain::BankAccount;

use super::BankAccountView;

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
    schema: Extension<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[derive(Default)]
struct AddQuery;

#[Object]
impl AddQuery {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn bank_account<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: String,
    ) -> FieldResult<BankAccountView> {
        let view_repo = ctx.data::<Arc<PostgresViewRepository<BankAccountView, BankAccount>>>()?;
        let view = match view_repo.load(&id).await? {
            Some(view) => view,
            None => {
                return Err(async_graphql::Error::new("Bank account not found"));
            }
        };
        tracing::debug!("Loaded view in GraphQL response: {:?}", view);
        Ok(view)
    }
}
#[derive(MergedObject, Default)]
struct QueryRoot(AddQuery, TodoTwo);

#[derive(SimpleObject, Default)]
struct TodoTwo {
    id: i32,
    description: String,
    completed: bool,
}

#[instrument(skip(bank_account_view_repsitory))]
pub async fn run_graphql_server(
    config: &GraphQlConfiguration,
    bank_account_view_repsitory: Arc<PostgresViewRepository<BankAccountView, BankAccount>>,
) {
    tracing::debug!("Starting graphql server");
    let serve_address = config.parse_serve_address();

    // create the schema
    let schema = Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(bank_account_view_repsitory)
        .finish();

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
