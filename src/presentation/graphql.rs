use crate::configuration::GraphQlConfiguration;
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};

use tracing::instrument;

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
    // create the schema
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    let serve_address = format!("http://localhost:{}", config.port);
    println!("GraphiQL IDE accessible at: {serve_address}",);

    Server::bind(&serve_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
