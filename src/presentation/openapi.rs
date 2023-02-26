use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::domain::Todo;
use crate::presentation::todo;

#[derive(OpenApi)]
#[openapi(
      paths(
          todo::list_todos,
          todo::mark_as_completed,
          todo::get_todo_by_id,
          todo::post_todo,
          todo::delete_todo,
      ),
      components(
          schemas(Todo, todo::TodoError),
      ),
      modifiers(&SecurityAddon),
      tags(
          (name = "Todo", description = "Todo items management API")
      ),
        info(
            title = "Todo API: built with Velox",
            version = "0.1.0",
            description = "A simple API to manage todo items",
            contact(name = "Liam Woodleigh", url="https://github.com/liamwh/"),
        ),
  )]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }
}
