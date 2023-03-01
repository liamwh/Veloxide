use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum BankAccountCommand {
    #[schema(example = "Supply an account ID to open a new account")]
    OpenAccount { account_id: String },

    #[schema(example = "Supply the amount to deposit")]
    DepositMoney { amount: f64 },

    #[schema(example = "Supply the amount to withdraw and the ATM ID it will be withdrawn from")]
    WithdrawMoney { amount: f64, atm_id: String },

    #[schema(example = "Supply the amount and check number to write a check")]
    WriteCheck { check_number: String, amount: f64 },
}
