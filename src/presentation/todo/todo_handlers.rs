use crate::{application::DynTodoService, domain::Todo};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use hyper::{HeaderMap, StatusCode};
use tracing::instrument;

use super::todo_dtos::TodoError;

/// List all Todo items
#[utoipa::path(
  get,
  tag = "Todo",
  path = "/todo",
  responses(
      (status = 200, description = "Get todos by query parameters", body = [Todo])
  )
)]
#[instrument(skip(todo_service))]
#[axum_macros::debug_handler]
pub async fn list_todos(State(todo_service): State<DynTodoService>) -> Json<Vec<Todo>> {
    let todos = todo_service.get_all().await;

    Json(todos)
}

/// Create new Todo
///
/// Tries to create a new Todo item to in-memory storage or fails with 409 conflict if already exists.
#[utoipa::path(
    post,
    tag = "Todo",
    path = "/todo",
    request_body = Todo,
    responses(
        (status = 201, description = "Todo item created successfully", body = Todo),
        (status = 409, description = "Todo already exists", body = TodoError)
    )
)]
#[instrument(skip(todo_service))]
pub(crate) async fn post_todo(
    State(todo_service): State<DynTodoService>,
    Json(todo): Json<Todo>,
) -> impl IntoResponse {
    match todo_service.create_todo(&todo).await {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(e) => (
            StatusCode::CONFLICT,
            Json(TodoError::Conflict(e.to_string())),
        )
            .into_response(),
    }
}

/// Mark Todo item as completed by id
///
/// Mark Todo item as completed by a given id.
#[utoipa::path(
    post,
    tag = "Todo",
    path = "/todo/{id}/mark-as-completed",
    responses(
        (status = 200, description = "Todo marked as completed successfully"),
        (status = 404, description = "Todo not found"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = i32, Path, description = "Todo database id")
    ),
    security(
        (), // <-- make authentication optional
        ("api_key" = [])
    )
)]
#[instrument(skip(todo_service))]
pub(crate) async fn mark_as_completed(
    Path(id): Path<i32>,
    State(todo_service): State<DynTodoService>,
    headers: HeaderMap,
) -> impl IntoResponse {
    match check_api_key(false, headers) {
        Ok(_) => (),
        Err(_) => return StatusCode::UNAUTHORIZED,
    }
    let result = todo_service.mark_todo_as_completed(&id).await;
    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::NOT_FOUND,
    }
}

/// Get Todo by id
///
/// Get Todo item by given id. Return only status 200 on success or 404 if Todo is not found.
#[utoipa::path(
    get,
    tag = "Todo",
    path = "/todo/{id}",
    responses(
        (status = 200, description = "Todo returned successfully", body = Todo),
        (status = 404, description = "Todo not found")
    ),
    params(
        ("id" = i32, Path, description = "Todo database id")
    ),
    security(
        (), // <-- make optional authentication
        ("api_key" = [])
    ),
)]
#[instrument(skip(todo_service))]
pub(crate) async fn get_todo_by_id(
    Path(id): Path<i32>,
    State(todo_service): State<DynTodoService>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let result = todo_service.get_todo_by_id(&id).await;
    match result {
        Ok(todo) => (StatusCode::OK, Json(todo)).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(TodoError::NotFound(e.to_string())),
        )
            .into_response(),
    }
}

// normally you should create a middleware for this but this is sufficient for sake of example.
#[instrument]
fn check_api_key(require_api_key: bool, headers: HeaderMap) -> Result<(), impl IntoResponse> {
    match headers.get("todo_apikey") {
        Some(header) if header != "example_api_key" => Err((
            StatusCode::UNAUTHORIZED,
            Json(TodoError::Unauthorized(String::from("incorrect api key"))),
        )
            .into_response()),
        None if require_api_key => Err((
            StatusCode::UNAUTHORIZED,
            Json(TodoError::Unauthorized(String::from("missing api key"))),
        )
            .into_response()),
        _ => Ok(()),
    }
}

/// Delete Todo item by id
///
/// Delete Todo item by id.
#[utoipa::path(
    delete,
    tag = "Todo",
    path = "/todo/{id}",
    responses(
        (status = 200, description = "Todo deleted successfully"),
        (status = 401, description = "Unauthorized to delete Todo", body = TodoError, example = json!(TodoError::Unauthorized(String::from("missing api key")))),
        (status = 404, description = "Todo not found", body = TodoError, example = json!(TodoError::NotFound(String::from("id = 1"))))
    ),
    params(
        ("id" = i32, Path, description = "Todo database id")
    ),
    security(
        ("api_key" = [])
    )
)]
#[instrument(skip(todo_service))]
pub(crate) async fn delete_todo(
    Path(id): Path<i32>,
    State(todo_service): State<DynTodoService>,
    headers: HeaderMap,
) -> impl IntoResponse {
    match check_api_key(true, headers) {
        Ok(_) => (),
        Err(error) => return error.into_response(),
    }
    let result = todo_service.delete_todo(&id).await;
    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_e) => (StatusCode::NOT_FOUND).into_response(),
    }
}
