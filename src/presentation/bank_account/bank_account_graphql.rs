use super::*;

use async_graphql::{Context, FieldResult, Object};

#[derive(Default)]
pub struct BankAccountGraphQlQuery {}

#[Object]
impl BankAccountGraphQlQuery {
    /// Get a bank account by its ID
    async fn bank_account<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: String,
    ) -> FieldResult<BankAccountView> {
        let view_repo = ctx.data::<Arc<PostgresViewRepository<BankAccountView, BankAccount>>>()?;
        let view = match view_repo.load(&id).await? {
            Some(view) => view,
            None => {
                return Err(async_graphql::Error::new("Bank account not found"));
            }
        };
        tracing::debug!("Loaded view in GraphQL response: {:?}", view);
        Ok(view)
    }
}
