use crate::presentation::graphql::GraphQlConfiguration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfiguration {
    pub repository: Repository,
    pub graphql: GraphQlConfiguration,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Repository {
    Postgres,
    Memory,
}
