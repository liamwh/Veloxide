use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// TODO: Re-access error handling and dtos

/// Todo operation errors
#[derive(Serialize, Deserialize, ToSchema)]
pub enum TodoError {
    /// Todo already exists conflict.
    #[schema(example = "Todo already exists")]
    Conflict(String),
    /// Todo not found by id.
    #[schema(example = "id = 1")]
    NotFound(String),

    /// Todo operation unauthorized
    #[schema(example = "missing api key")]
    Unauthorized(String),

    /// Invalid input
    InvalidInput,

    /// Internal error
    InternalError,
}
