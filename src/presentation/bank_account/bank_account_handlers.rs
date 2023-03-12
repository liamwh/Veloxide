use super::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "postgres")] {
        // Serves as our query endpoint to respond with the materialized `BankAccountView`
// for the requested account.
#[utoipa::path(
    get,
    tag = "Bank Accounts",
    path = "/api/bank-accounts/{id}",
    params(
        ("id" = i32, Path, description = "Bank account ID")
    ),
    responses(
        (status = 200, description = "Get bank account details", body = [BankAccountView])
    )
  )]
  #[instrument(skip(view_repo))]
  pub async fn query_handler(
      Path(id): Path<String>,
      Extension(view_repo): Extension<Arc<PostgresViewRepository<BankAccountView, BankAccount>>>,
  ) -> Response {
      let view = match view_repo.load(&id).await {
          Ok(view) => view,
          Err(err) => {
              return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
          }
      };
      match view {
          None => StatusCode::NOT_FOUND.into_response(),
          Some(account_view) => (StatusCode::OK, Json(account_view)).into_response(),
      }
  }

  // Serves as our command endpoint to make changes in a `BankAccount` aggregate.
  #[utoipa::path(
    post,
    tag = "Bank Accounts",
    path = "/api/bank-accounts/{id}",
    responses(
      (status = 204, description = "Command issued successfully"),
      (status = 400, description = "Command failed", body = [String])
    ),
    request_body(content = BankAccountCommand, description = "Bank account command to execute, see the Bank Account Command schema at the bottom of the page for details", content_type = "application/json"),
    params(
      ("id" = i32, Path, description = "Bank account ID"),
    ),
  )]
  #[instrument(skip(cqrs))]
  pub async fn command_handler(
      Path(id): Path<String>,
      Extension(cqrs): Extension<Arc<PostgresCqrs<BankAccount>>>,
      MetadataExtension(metadata): MetadataExtension,
      Json(command): Json<BankAccountCommand>,
  ) -> Response {
      match cqrs.execute_with_metadata(&id, command, metadata).await {
          Ok(_) => StatusCode::NO_CONTENT.into_response(),
          Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
      }
  }

    } else if #[cfg(feature = "mysql")] {
// for the requested account.
#[utoipa::path(
    get,
    tag = "Bank Accounts",
    path = "/api/bank-accounts/{id}",
    params(
        ("id" = i32, Path, description = "Bank account ID")
    ),
    responses(
        (status = 200, description = "Get bank account details", body = [BankAccountView])
    )
  )]
  #[instrument(skip(view_repo))]
  pub async fn query_handler(
      Path(id): Path<String>,
      Extension(view_repo): Extension<Arc<MysqlViewRepository<BankAccountView, BankAccount>>>,
  ) -> Response {
      let view = match view_repo.load(&id).await {
          Ok(view) => view,
          Err(err) => {
              return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
          }
      };
      match view {
          None => StatusCode::NOT_FOUND.into_response(),
          Some(account_view) => (StatusCode::OK, Json(account_view)).into_response(),
      }
  }

  // Serves as our command endpoint to make changes in a `BankAccount` aggregate.
  #[utoipa::path(
    post,
    tag = "Bank Accounts",
    path = "/api/bank-accounts/{id}",
    responses(
      (status = 204, description = "Command issued successfully"),
      (status = 400, description = "Command failed", body = [String])
    ),
    request_body(content = BankAccountCommand, description = "Bank account command to execute, see the Bank Account Command schema at the bottom of the page for details", content_type = "application/json"),
    params(
      ("id" = i32, Path, description = "Bank account ID"),
    ),
  )]
  #[instrument(skip(cqrs))]
  pub async fn command_handler(
      Path(id): Path<String>,
      Extension(cqrs): Extension<Arc<MysqlCqrs<BankAccount>>>,
      MetadataExtension(metadata): MetadataExtension,
      Json(command): Json<BankAccountCommand>,
  ) -> Response {
      match cqrs.execute_with_metadata(&id, command, metadata).await {
          Ok(_) => StatusCode::NO_CONTENT.into_response(),
          Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
      }
  }
    }
}
