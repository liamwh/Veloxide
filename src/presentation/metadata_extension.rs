use super::*;
use axum::extract::{FromRequest, FromRequestParts};
use axum::http::{request::Parts, Request, StatusCode};
use std::collections::HashMap;
use std::convert::Infallible;

// This is a custom Axum extension that builds metadata from the inbound request.
pub struct MetadataExtension(pub HashMap<String, String>);

const USER_AGENT_HDR: &str = "User-Agent";

#[async_trait]
impl<S, B> FromRequest<S, B> for MetadataExtension
where
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = Infallible;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        // Here we are including the current date/time, the uri that was called and the user-agent
        // in a HashMap that we will submit as metadata with the command.
        let mut metadata = HashMap::default();
        metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
        metadata.insert("uri".to_string(), req.uri().to_string());
        if let Some(user_agent) = req.headers().get(USER_AGENT_HDR) {
            if let Ok(value) = user_agent.to_str() {
                metadata.insert(USER_AGENT_HDR.to_string(), value.to_string());
            }
        }
        Ok(MetadataExtension(metadata))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for MetadataExtension
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    // This is the method that is called when we use the MetadataExtension as a parameter in a
    // handler. This implementation does not consume the request, unlike from_request which does.
    // Note that json will consume the body, so must always be the last parameter in a handler.
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Here we are including the current date/time, the uri that was called and the user-agent
        // in a HashMap that we will submit as metadata with the command.
        let mut metadata = HashMap::default();
        metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
        metadata.insert("path".to_string(), parts.uri.path().to_string());
        Ok(MetadataExtension(metadata))
    }
}
