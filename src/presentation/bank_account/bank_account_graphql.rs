use super::*;

use async_graphql::{Context, Object};

pub struct QueryRoot {}

#[Object]
impl QueryRoot {
    async fn account<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "If omitted, bank account with id 1. If provided, returns the bank account with that particular id."
        )]
        account_id: u32,
    ) -> BankAccountView {
        let query = ctx.data_unchecked::<AccountQuery>();
        query.load(&account_id).await.unwrap()
    }
}
