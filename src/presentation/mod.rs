#[cfg(test)]
use mockall::predicate::*;

use super::domain::*;
use async_trait::async_trait;
use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, response::Response, Extension, Json,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::{ToResponse, ToSchema};

pub mod metadata_extension;

pub mod bank_account;
pub mod errors;
pub mod openapi;
pub mod todo;
pub mod graphql;

// Re-exports
pub use bank_account::*;
pub use errors::*;
pub use metadata_extension::*;
pub use openapi::*;
pub use todo::*;
pub use graphql::*;