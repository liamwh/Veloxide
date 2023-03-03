use super::*;

// Our second query, this one will be handled with Postgres `GenericQuery`
// which will serialize and persist our view after it is updated. It also
// provides a `load` method to deserialize the view on request.
pub type AccountQuery = GenericQuery<
    PostgresViewRepository<BankAccountView, BankAccount>,
    BankAccountView,
    BankAccount,
>;

// The view for a BankAccount query, for a standard http application this should
// be designed to reflect the response dto that will be returned to a user.
#[derive(Debug, Default, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct BankAccountView {
    account_id: Option<String>,
    balance: f64,
    written_checks: Vec<String>,
    ledger: Vec<LedgerEntry>,
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
                self.ledger.push(LedgerEntry::new("deposit", *amount));
                self.balance = *balance;
            }

            BankAccountEvent::CustomerWithdrewCash { amount, balance } => {
                self.ledger
                    .push(LedgerEntry::new("atm withdrawal", *amount));
                self.balance = *balance;
            }

            BankAccountEvent::CustomerWroteCheck {
                check_number,
                amount,
                balance,
            } => {
                self.ledger.push(LedgerEntry::new(check_number, *amount));
                self.written_checks.push(check_number.clone());
                self.balance = *balance;
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct LedgerEntry {
    description: String,
    amount: f64,
}
impl LedgerEntry {
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
