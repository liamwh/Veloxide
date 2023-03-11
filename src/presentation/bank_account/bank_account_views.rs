use async_graphql::SimpleObject;
use ts_rs::TS;

use super::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "postgres")] {

// Our second query, this one will be handled with Postgres `GenericQuery`
// which will serialize and persist our view after it is updated. It also
// provides a `load` method to deserialize the view on request.
pub type AccountQuery = GenericQuery<
PostgresViewRepository<BankAccountView, BankAccount>,
BankAccountView,
BankAccount,
>;
    } else if #[cfg(feature = "mysql")] {

// Our second query, this one will be handled with Mysql `GenericQuery`
// which will serialize and persist our view after it is updated. It also
// provides a `load` method to deserialize the view on request.
pub type AccountQuery = GenericQuery<
    MysqlViewRepository<BankAccountView, BankAccount>,
    BankAccountView,
    BankAccount,
>;
} else {
        compile_error!("Must specify either mysql or postgres feature");
    }
}

// The view for a BankAccount query, for a standard http application this should
// be designed to reflect the response dto that will be returned to a user.

#[derive(SimpleObject, Debug, Default, Serialize, Deserialize, ToSchema, ToResponse, TS)]
#[ts(export, export_to = "frontend/src/bindings/")]
#[ts(export)]
pub struct BankAccountView {
    account_id: Option<String>,
    balance: f64,
    written_checks: Vec<String>,
    account_transactions: Vec<AccountTransaction>,
}

// This updates the view with events as they are committed.
// The logic should be minimal here, e.g., don't calculate the account balance,
// design the events to carry the balance information instead.
impl View<BankAccount> for BankAccountView {
    fn update(&mut self, event: &EventEnvelope<BankAccount>) {
        match &event.payload {
            BankAccountEvent::AccountOpened { account_id } => {
                self.account_id = Some(account_id.clone());
            }

            BankAccountEvent::CustomerDepositedMoney { amount, balance } => {
                self.account_transactions
                    .push(AccountTransaction::new("deposit", *amount));
                self.balance = *balance;
            }

            BankAccountEvent::CustomerWithdrewCash { amount, balance } => {
                self.account_transactions
                    .push(AccountTransaction::new("atm withdrawal", *amount));
                self.balance = *balance;
            }

            BankAccountEvent::CustomerWroteCheck {
                check_number,
                amount,
                balance,
            } => {
                self.account_transactions
                    .push(AccountTransaction::new(check_number, *amount));
                self.written_checks.push(check_number.clone());
                self.balance = *balance;
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, ToResponse, SimpleObject, TS)]
#[ts(export, export_to = "frontend/src/bindings/")]
pub struct AccountTransaction {
    description: String,
    amount: f64,
}
impl AccountTransaction {
    fn new(description: &str, amount: f64) -> Self {
        Self {
            description: description.to_string(),
            amount,
        }
    }
}

pub struct SimpleLoggingQuery;

#[async_trait]
impl Query<BankAccount> for SimpleLoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<BankAccount>]) {
        for event in events {
            println!("{}-{}\n{:#?}", aggregate_id, event.sequence, &event.payload);
        }
    }
}
