use crate::presentation::graphql::GraphQlConfiguration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfiguration {
    pub graphql: GraphQlConfiguration,
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

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
