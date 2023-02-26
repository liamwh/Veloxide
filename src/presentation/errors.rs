use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde_json::json;

use crate::presentation::todo::TodoError;

// Our app's top level error type.
pub enum AppError {
    /// Something went wrong when calling the todo service.
    TodoError(TodoError),
}

impl From<TodoError> for AppError {
    fn from(inner: TodoError) -> Self {
        AppError::TodoError(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::TodoError(TodoError::Conflict(err)) => {
                (StatusCode::CONFLICT, format!("Todo already existsL {err}",))
            }
            AppError::TodoError(TodoError::NotFound(err)) => {
                (StatusCode::NOT_FOUND, format!("Todo not found: {err}"))
            }
            AppError::TodoError(TodoError::InvalidInput) => {
                (StatusCode::BAD_REQUEST, "Invalid input".to_string())
            }
            AppError::TodoError(TodoError::InternalError) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal error".to_string(),
            ),
            AppError::TodoError(TodoError::Unauthorized(err)) => {
                (StatusCode::UNAUTHORIZED, format!("Unauthorized: {err}"))
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
