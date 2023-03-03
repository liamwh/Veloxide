use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::domain::BankAccountCommand;
use crate::presentation::*;

#[derive(OpenApi)]
#[openapi(
      paths(
          bank_account::query_handler,
          bank_account::command_handler,
      ),
      components(
          schemas(
            BankAccountView,
            BankAccountCommand,
            AccountTransaction),
    ),
      modifiers(&SecurityAddon),
      tags(
          (name = "Bank Accounts", description = "Bank Account Management API")
      ),
        info(
            title = "Bank Account API: built with Velox",
            version = "0.1.0",
            description = "An event-sourced bank account API built with Velox",
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
